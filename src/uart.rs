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
pub trait Uart {

    fn can_read(&self) -> bool;

    fn read(&mut self) -> u8;

    fn flush(&mut self) -> bool;

    fn write(&mut self, data: u8);

}

macro_rules! uarts {
    ($(($type: ident, $hardware: ident, $pcon: ident)),* $(,)?) => {
        $(
            #[doc="A HAL wrapper for UART hardware $hardware"]
            pub struct $type<S: InitState> {
                _state: PhantomData<S>,
                _uart: $hardware,
            }

            impl From<$hardware> for $type<Disabled> {
                fn from(uart: $hardware) -> Self {
                    $type{ _state: PhantomData, _uart: uart }
                }
            }

            impl<S> $type<S>
             where S: InitState
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
            }

            impl $type<Disabled> {

                /// Sets up the UART hardware for use as a serial line.
                ///
                /// # Arguments
                ///
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
                /// let uart = hal.uart0.enable();
                /// let mut uart = uart::Serial::new(hal.uart0, (rx, tx).into()).enable(&clock);
                /// ```
                pub fn enable(self) -> $type<Enabled> {
                    unsafe { (*crate::pac::SYSCON::ptr()).pconp.write(|w| w.$pcon().set_bit()); };

                    self._uart.fcr().write(|w| w.fifoen().set_bit());
                    self._uart.lcr.write(|w| w.wls()._8_bit_character_leng().dlab().set_bit());
                    unsafe {
                        self._uart.dlm_mut().write(|w| w.bits(0));
                        self._uart.dll_mut().write(|w| w.bits(34));
                        self._uart.fdr.write(|w| w.mulval().bits(15).divaddval().bits(8));
                    }

                    self._uart.lcr.modify(|_, w| w.dlab().clear_bit());
                    $type::<Enabled> { _state: PhantomData, _uart: self._uart }
                }

            }

            impl Uart for $type<Enabled> {

                fn can_read(&self) -> bool {
                    self._uart.lsr.read().rdr().bit()
                }

                fn read(&mut self) -> u8 {
                    self._uart.rbr().read().rbr().bits()
                }

                fn flush(&mut self) -> bool {
                    self._uart.lsr.read().thre().bit()
                }

                fn write(&mut self, data: u8) {
                    unsafe { self._uart.thr().write(|w| w.thr().bits(data)); }
                }

            }
        )*
    }
}

uarts!(
    (Uart0, UART0, pcuart0),
    (Uart1, UART1, pcuart1),
    (Uart2, UART2, pcuart2),
    (Uart3, UART3, pcuart3),
    (Uart4, UART4, pcuart4),
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

pub struct Serial<UART: Uart, RX: UartRx, TX: UartTx> {
    _uart: UART,
    _rx: RX,
    _tx: TX,
}

impl<UART, RX, TX> Serial<UART, RX, TX>
where
    UART: Uart,
    RX: UartRx,
    TX: UartTx,
{

    /// Create a new serial line terminal.
    ///
    /// # Arguments
    ///
    /// * uart - A UART that shall be used to transmit and receive data.
    /// * pins - Pair of RX/TX pins used for the UART.
    ///
    /// # Example
    /// ```
    /// let hal = crate::Hal::new();
    /// let clock = hal.clock.enable(120_000_000, 12_000_000);
    /// let pins = hal.gpio0.split();
    /// let tx = pins.p0_2;
    /// let rx = pins.p0_3;
    /// let uart = hal.uart0.enable();
    /// let mut uart = uart::Serial::new(uart, (rx, tx).into());
    /// ```
    pub fn new(uart: UART, pins: UartPins<RX, TX>) -> Serial<UART, RX, TX> {
        Serial {
            _uart: uart,
            _rx: pins.rx,
            _tx: pins.tx,
        }
    }

    pub fn free(self) -> (UART, RX, TX) {
        (self._uart, self._rx, self._tx)
    }

}

impl<UART, RX, TX> Read<u8> for Serial<UART, RX, TX>
where
    UART: Uart,
    RX: UartRx,
    TX: UartTx,
{
    type Error = ();

    fn try_read(&mut self) -> Result<u8, nb::Error<()>> {
        if self._uart.can_read() {
            Ok(self._uart.read())
        } else {
            Err(WouldBlock)
        }
    }
}

impl<UART, RX, TX> Write<u8> for Serial<UART, RX, TX>
where
    UART: Uart,
    RX: UartRx,
    TX: UartTx,
{
    type Error = ();

    fn try_write(&mut self, word: u8) -> Result<(), nb::Error<()>> {
        self._uart.write(word);
        Ok(())
    }

    fn try_flush(&mut self) -> Result<(), nb::Error<()>> {
        if self._uart.flush() {
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
