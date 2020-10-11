#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use lpc178x_7x_hal::*;

#[entry]
#[allow(unused_must_use)]
fn main() -> ! {
    let hal = Hal::new();
    let clock = hal.clock.enable(120_000_000, 12_000_000);
    let pins = hal.gpio0.split();
    let tx = pins.p0_2;
    let rx = pins.p0_3;
    let mut uart = uart::Serial::new(hal.uart0, (rx, tx).into()).enable(&clock);
    if let Ok(()) = nb::block!(uart.try_write(b'!')) {
        nb::block!(uart.try_flush());
        match nb::block!(uart.try_read()) {
            Ok(byte) => {
                if byte == b'!' {
                    hprintln!("[OK]");
                } else {
                    hprintln!("[Fail]: Wrote '!' but received '{}'", byte);
                }
            },
            Err(_) => { hprintln!("[Fail]: Reading failed"); () }
        }
    } else {
        hprintln!("[Fail]: Writing failed");
    }
    loop {}
}