#![no_std]

pub use lpc178x_7x as pac;
pub use nb as nb;
pub use embedded_hal::digital::*;
pub use embedded_hal::serial::*;
use crate::typestates::Disabled;

pub mod clock;
pub mod gpio;
pub mod uart;
mod typestates;

pub struct Hal {
    pub clock: clock::Clock<Disabled>,
    pub gpio0: gpio::gpio0::GPIO0,
    pub gpio1: gpio::gpio1::GPIO1,
    pub gpio2: gpio::gpio2::GPIO2,
    pub gpio3: gpio::gpio3::GPIO3,
    pub gpio4: gpio::gpio4::GPIO4,
    pub gpio5: gpio::gpio5::GPIO5,
    pub uart0: uart::Uart0,
    pub uart1: uart::Uart1,
    pub uart2: uart::Uart2,
    pub uart3: uart::Uart3,
    pub uart4: uart::Uart4,
}

impl Hal {
    pub fn new() -> Hal {
        let _peripherals = pac::Peripherals::take().unwrap();
        Hal {
            clock: clock::Clock::<Disabled>::new(),
            gpio0: gpio::gpio0::GPIO0::new(),
            gpio1: gpio::gpio1::GPIO1::new(),
            gpio2: gpio::gpio2::GPIO2::new(),
            gpio3: gpio::gpio3::GPIO3::new(),
            gpio4: gpio::gpio4::GPIO4::new(),
            gpio5: gpio::gpio5::GPIO5::new(),
            uart0: _peripherals.UART0.into(),
            uart1: _peripherals.UART1.into(),
            uart2: _peripherals.UART2.into(),
            uart3: _peripherals.UART3.into(),
            uart4: _peripherals.UART4.into(),
        }
    }
}
