use crate::gpio;
use crate::clock;
use crate::typestates::{
    InitState,
    Enabled,
    Disabled,
    PinState,
};
use core::convert::From;
use core::marker::PhantomData;

use embedded_hal::serial::{Read, Write};

use crate::pac::{
    UART0, UART1, UART2, UART3, UART4
};
use nb::Error::WouldBlock;

pub trait UartRx {
    fn into_uartrx(&self);
}
pub trait UartTx {
    fn into_uarttx(&self);
}
/// Marker trait for different UART Hal types that wrap each UART hardware.
pub trait Uart{}

macro_rules! uarts {
    ($(($type: ident, $hardware: ident)),* $(,)?) => {
        $(
            #[doc="A HAL wrapper for UART hardware $hardware"]
            pub struct $type {
                _uart: $hardware,
            }

            impl From<$hardware> for $type {
                fn from(uart: $hardware) -> Self {
                    $type{ _uart: uart }
                }
            }

            impl $type {
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
            }

            impl Uart for $type {}
        )*
    }
}

uarts!(
    (Uart0, UART0),
    (Uart1, UART1),
    (Uart2, UART2),
    (Uart3, UART3),
    (Uart4, UART4),
);

pub struct UartPins<RX: UartRx, TX: UartTx> {
    rx: RX,
    tx: TX,
}

impl<RX, TX> From<(RX, TX)> for UartPins<RX, TX>
where
    RX: UartRx,
    TX: UartTx,
{
    fn from(pins: (RX, TX)) -> Self {
        pins.0.into_uartrx();
        pins.1.into_uarttx();
        UartPins::<RX, TX>{ rx: pins.0, tx: pins.1 }
    }
}

impl<RX, TX> UartPins<RX, TX>
where
    RX: UartRx,
    TX: UartTx,
{
    pub fn free(self) -> (RX, TX) {
        (self.rx, self.tx)
    }
}

pub struct Serial<Init: InitState, UART: Uart, RX: UartRx, TX: UartTx> {
    _state: PhantomData<Init>,
    _uart: UART,
    _rx: RX,
    _tx: TX,
}

impl<State, UART, RX, TX> Serial<State, UART, RX, TX>
where
    State: InitState,
    UART: Uart,
    RX: UartRx,
    TX: UartTx,
{

    pub fn free(self) -> (UART, RX, TX) {
        (self._uart, self._rx, self._tx)
    }

}

impl<UART, RX, TX> Serial<Disabled, UART, RX, TX>
where
    UART: Uart,
    RX: UartRx,
    TX: UartTx,
{

    pub fn new(uart: UART, pins: UartPins<RX, TX>) -> Serial<Disabled, UART, RX, TX> {
        Serial{
            _state: PhantomData,
            _uart: uart,
            _rx: pins.rx,
            _tx: pins.tx,
        }
    }

    /// Sets up the UART hardware for use as a serial line.
    ///
    /// # Arguments
    ///
    /// * baudrate - The baudrate for the serial line.
    /// * clock - Enabled CPU clock component.
    ///
    /// # Example
    /// ```
    /// let hal = crate::Hal::new();
    /// let clock = hal.clock.enable(120_000_000, 12_000_000);
    /// let pins = hal.gpio0.split();
    /// let tx = pins.p0_2;
    /// let rx = pins.p0_3;
    /// let mut uart = uart::Serial::new(hal.uart0, (rx, tx).into()).enable(&clock);
    /// ```
    pub fn enable(self, _clock: &clock::Clock<Enabled>) -> Serial<Enabled, UART, RX, TX> {

        let uart = unsafe {
            (*crate::pac::SYSCON::ptr()).pconp.write(|w| w.pcuart0().set_bit());
            &(*crate::pac::UART0::ptr())
        };

        uart.fcr().write(|w| w.fifoen().set_bit());
        uart.lcr.write(|w| w.wls()._8_bit_character_leng().dlab().set_bit());
        unsafe {
            uart.dlm_mut().write(|w| w.bits(2));
            uart.dll_mut().write(|w| w.bits(8));
            uart.fdr.write(|w| w.mulval().bits(2).divaddval().bits(1));
        }

        uart.lcr.modify(|_, w| w.dlab().clear_bit());

        Serial{
            _state: PhantomData,
            _uart: self._uart,
            _rx: self._rx,
            _tx: self._tx
        }
    }

}

impl<UART, RX, TX> Read<u8> for Serial<Enabled, UART, RX, TX>
where
    UART: Uart,
    RX: UartRx,
    TX: UartTx,
{
    type Error = ();

    fn try_read(&mut self) -> Result<u8, nb::Error<()>> {
        if unsafe { (*crate::pac::UART0::ptr()).lsr.read().rdr().bit() } {
            unsafe {
                Ok((*crate::pac::UART0::ptr()).rbr().read().rbr().bits())
            }
        } else {
            Err(WouldBlock)
        }
    }
}

impl<UART, RX, TX> Write<u8> for Serial<Enabled, UART, RX, TX>
where
    UART: Uart,
    RX: UartRx,
    TX: UartTx,
{
    type Error = ();

    fn try_write(&mut self, word: u8) -> Result<(), nb::Error<()>> {
        unsafe {
            (*crate::pac::UART0::ptr()).thr().write(|w| w.thr().bits(word));
        }
        Ok(())
    }

    fn try_flush(&mut self) -> Result<(), nb::Error<()>> {
        if unsafe { (*crate::pac::UART0::ptr()).lsr.read().thre().bit() } {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }
}

macro_rules! uart_rx {
($(($pin: ident, $io: ident, $func: ident)),* $(,)?) => {
    $(
        impl<T> UartRx for gpio::$pin<T> where T: PinState {
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
($(($pin: ident, $io: ident, $func: ident)),* $(,)?) => {
    $(
        impl<T> UartTx for gpio::$pin<T> where T: PinState {
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
    (P0_1, p0_1, u0_rxd),
    // (P0_1, p0_1, u3_rxd),
    (P0_3, p0_3, u0_rxd),
    // (P0_3, p0_3, u3_rxd),
    (P0_11, p0_11, u2_rxd),
    (P0_16, p0_16, u1_rxd),
    (P0_26, p0_26, u3_rxd),
    (P2_1, p2_1, u1_rxd),
    (P2_9, p2_9, u2_rxd),
    // (P2_9, p2_9, u4_rxd),
    (P3_17, p3_17, u1_rxd),
    (P4_23, p4_23, u2_rxd),
    (P4_29, p4_29, u3_rxd),
    (P5_3, p5_3, u4_rxd),
);

uart_tx!(
    (P0_0, p0_0, u0_txd),
    // (P0_0, p0_0, u3_txd),
    (P0_2, p0_2, u0_txd),
    // (P0_2, p0_2, u3_txd),
    (P0_10, p0_10, u2_txd),
    (P0_15, p0_15, u1_txd),
    (P0_22, p0_22, u4_txd),
    (P0_25, p0_25, u3_txd),
    (P1_29, p1_29, u4_txd),
    (P2_0, p2_0, u1_txd),
    (P2_8, p2_8, u2_txd),
    (P3_16, p3_16, u1_txd),
    (P4_22, p4_22, u2_txd),
    (P4_28, p4_28, u3_txd),
    (P5_4, p5_4, u4_txd),
);
