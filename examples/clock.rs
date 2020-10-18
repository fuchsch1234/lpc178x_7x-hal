#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

use lpc178x_7x_hal::*;

#[entry]
fn main() -> ! {
    let hal = Hal::new();
    let _clock = hal.clock.enable(120_000_000, 12_000_000);
    if let Err(_) = hprintln!("[OK]") {}
    loop {}
}
