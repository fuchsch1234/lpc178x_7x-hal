#![no_std]

pub use lpc178x_7x as pac;
pub use nb as nb;
pub use embedded_hal::digital::*;
use crate::typestates::Disabled;

pub mod gpio;
mod typestates;

pub struct Hal {
    pub gpio0: gpio::gpio0::GPIO0,
    pub gpio1: gpio::gpio1::GPIO1,
    pub gpio2: gpio::gpio1::GPIO2,
    pub gpio3: gpio::gpio1::GPIO3,
    pub gpio4: gpio::gpio1::GPIO4,
    pub gpio5: gpio::gpio1::GPIO5,
}

impl Hal {
    pub fn new() -> Hal {
        let _peripherals = pac::Peripherals::take().unwrap();
        Hal {
            gpio0: gpio::gpio0::GPIO0::new(),
            gpio1: gpio::gpio1::GPIO1::new(),
            gpio2: gpio::gpio1::GPIO1::new(),
            gpio3: gpio::gpio1::GPIO1::new(),
            gpio4: gpio::gpio1::GPIO1::new(),
            gpio5: gpio::gpio1::GPIO1::new(),
        }
    }
}
