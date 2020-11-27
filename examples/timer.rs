#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use embedded_time::duration::*;
use panic_halt as _;

use lpc178x_7x_hal::*;
use nb::*;

#[entry]
fn main() -> ! {
    let hal = Hal::new();
    let clock = hal.clock.enable(96_000_000, 12_000_000);
    let mut timer = hal.timer0.enable(&clock);
    hprintln!("Starting timer...").unwrap();
    if let Err(_) = timer.try_start(3_000_000.microseconds()) {
        hprintln!("Starting timer failed").unwrap();
    }
    if let Err(_) = block!(timer.try_wait()) {
        hprintln!("Waiting for timer failed").unwrap();
    }
    hprintln!("[OK]").unwrap();
    loop {}
}
