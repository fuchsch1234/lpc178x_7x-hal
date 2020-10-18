use crate::gpio;
use crate::clock;
use crate::typestates::{
    InitState,
    Enabled,
    Disabled,
    PinState,
};
use core::convert::From;
use core::fmt;
use core::marker::PhantomData;

use embedded_hal::serial::{Read, Write};

use crate::pac::{
    UART0, UART1, UART2, UART3, UART4
};
use nb::Error::WouldBlock;

pub trait UartRx<UART> { fn into_uartrx(&self); }
pub trait UartTx<UART> { fn into_uarttx(&self); }

macro_rules! uarts {
    ($(($type: ident, $rx: ident, $tx: ident, $hardware: ident, $pcon: ident)),* $(,)?) => {
        $(

            #[doc="A HAL wrapper for UART hardware $hardware"]
            pub struct $type<S: InitState, Rx = (), Tx = ()> {
                _state: PhantomData<S>,
                _uart: $hardware,
                _rx: Rx,
                _tx: Tx,
            }

            impl From<$hardware> for $type<Disabled>
            {
                fn from(uart: $hardware) -> Self {
                    $type{ _state: PhantomData, _uart: uart, _rx: (), _tx: () }
                }
            }

            impl $type<Disabled>
            {
                #[allow(dead_code)]
                /// Releases the HAL wrapper and returns the wrapped hardware.
                /// Consumes the HAL wrapper.
                ///
                /// # Return
                ///
                /// * The hardware wrapped during construction.
                fn free(self) -> $hardware {
                    self._uart
                }

                /// Sets up the UART hardware for use as a serial line.
                ///
                /// # Arguments
                ///
                /// * rx - A pin implementing the matching UartXRx trait, that can be used for Rx.
                /// * tx - A pin implementing the matching UartXTx trait, that can be used for Tx.
                /// * baudrate - The baudrate for the UART.
                /// * clock - Enabled CPU clock component.
                ///
                /// # Example
                /// ```
                /// let hal = crate::Hal::new();
                /// let clock = hal.clock.enable(120_000_000, 12_000_000);
                /// let pins = hal.gpio0.split();
                /// let tx = pins.p0_2;
                /// let rx = pins.p0_3;
                /// let uart = hal.uart0.enable(rx, tx);
                /// ```
                pub fn enable<Rx, Tx>(self, rx: Rx, tx: Tx) -> $type<Enabled, Rx, Tx>
                where
                    Rx: UartRx<$type<Enabled, Rx, Tx>>,
                    Tx: UartTx<$type<Enabled, Rx, Tx>>,
                {
                    unsafe { (*crate::pac::SYSCON::ptr()).pconp.write(|w| w.$pcon().set_bit()); };

                    self._uart.fcr().write(|w| w.fifoen().set_bit());
                    self._uart.lcr.write(|w| w.wls()._8_bit_character_leng().dlab().set_bit());
                    unsafe {
                        self._uart.dlm_mut().write(|w| w.bits(0));
                        self._uart.dll_mut().write(|w| w.bits(34));
                        self._uart.fdr.write(|w| w.mulval().bits(15).divaddval().bits(8));
                    }

                    self._uart.lcr.modify(|_, w| w.dlab().clear_bit());
                    $type::<Enabled, Rx, Tx> { _state: PhantomData, _uart: self._uart, _rx: rx, _tx: tx }
                }

            }

            impl<Rx, Tx> Read<u8> for $type<Enabled, Rx, Tx>
            where
                Rx: UartRx<$type<Enabled, Rx, Tx>>,
                Tx: UartTx<$type<Enabled, Rx, Tx>>,
            {
                type Error = ();

                fn try_read(&mut self) -> Result<u8, nb::Error<()>> {
                    if self._uart.lsr.read().rdr().bit() {
                        Ok(self._uart.rbr().read().rbr().bits())
                    } else {
                        Err(WouldBlock)
                    }
                }
            }

            impl<Rx, Tx> Write<u8> for $type<Enabled, Rx, Tx>
            where
                Rx: UartRx<$type<Enabled, Rx, Tx>>,
                Tx: UartTx<$type<Enabled, Rx, Tx>>,
            {
                type Error = ();

                fn try_write(&mut self, data: u8) -> Result<(), nb::Error<()>> {
                    unsafe { self._uart.thr().write(|w| w.thr().bits(data)); }
                    Ok(())
                }

                fn try_flush(&mut self) -> Result<(), nb::Error<()>> {
                    if self._uart.lsr.read().thre().bit() {
                        Ok(())
                    } else {
                        Err(WouldBlock)
                    }
                }
            }

            impl<Rx, Tx> fmt::Write for $type<Enabled, Rx, Tx>
            where
                Rx: UartRx<$type<Enabled, Rx, Tx>>,
                Tx: UartTx<$type<Enabled, Rx, Tx>>,
            {
                fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
                    let _ = s
                        .as_bytes()
                        .into_iter()
                        .map(|c| nb::block!(self.try_write(*c)))
                        .last();
                    Ok(())
                }
            }
        )*
    }
}

uarts!(
    (Uart0, Uart0Rx, Uart0Tx, UART0, pcuart0),
    (Uart1, Uart1Rx, Uart1Tx, UART1, pcuart1),
    (Uart2, Uart2Rx, Uart2Tx, UART2, pcuart2),
    (Uart3, Uart3Rx, Uart3Tx, UART3, pcuart3),
    (Uart4, Uart4Rx, Uart4Tx, UART4, pcuart4),
);

macro_rules! uart_rx {
($(($pin: ident, $io: ident, $type: ident, $func: ident)),* $(,)?) => {
    $(
        impl<T, S, Rx, Tx> UartRx<$type<S, Rx, Tx>> for gpio::$pin<T>
        where
            T: PinState,
            S: InitState,
        {
            fn into_uartrx(&self) {
                unsafe {
                    (*crate::pac::IOCON::ptr()).$io.write(|w| w.func().$func());
                }
            }
        }
    )*
    }
}

macro_rules! uart_tx {
($(($pin: ident, $io: ident, $type: ident, $func: ident)),* $(,)?) => {
    $(
        impl<T, S, Rx, Tx> UartTx<$type<S, Rx, Tx>> for gpio::$pin<T>
        where
            T: PinState,
            S: InitState,
        {
            fn into_uarttx(&self) {
                unsafe {
                    (*crate::pac::IOCON::ptr()).$io.write(|w| w.func().$func());
                }
            }
        }
    )*
    }
}

uart_rx!(
    (P0_1, p0_1, Uart0, u0_rxd),
    (P0_1, p0_1, Uart3, u3_rxd),
    (P0_3, p0_3, Uart0, u0_rxd),
    (P0_3, p0_3, Uart3, u3_rxd),
    (P0_11, p0_11, Uart2, u2_rxd),
    (P0_16, p0_16, Uart1, u1_rxd),
    (P0_26, p0_26, Uart3, u3_rxd),
    (P2_1, p2_1, Uart1, u1_rxd),
    (P2_9, p2_9, Uart2, u2_rxd),
    (P2_9, p2_9, Uart4, u4_rxd),
    (P3_17, p3_17, Uart1, u1_rxd),
    (P4_23, p4_23, Uart2, u2_rxd),
    (P4_29, p4_29, Uart3, u3_rxd),
    (P5_3, p5_3, Uart4, u4_rxd),
);

uart_tx!(
    (P0_0, p0_0, Uart0, u0_txd),
    (P0_0, p0_0, Uart3, u3_txd),
    (P0_2, p0_2, Uart0, u0_txd),
    (P0_2, p0_2, Uart3, u3_txd),
    (P0_10, p0_10, Uart2, u2_txd),
    (P0_15, p0_15, Uart1, u1_txd),
    (P0_22, p0_22, Uart4, u4_txd),
    (P0_25, p0_25, Uart3, u3_txd),
    (P1_29, p1_29, Uart4, u4_txd),
    (P2_0, p2_0, Uart1, u1_txd),
    (P2_8, p2_8, Uart2, u2_txd),
    (P3_16, p3_16, Uart1, u1_txd),
    (P4_22, p4_22, Uart2, u2_txd),
    (P4_28, p4_28, Uart3, u3_txd),
    (P5_4, p5_4, Uart4, u4_txd),
);
