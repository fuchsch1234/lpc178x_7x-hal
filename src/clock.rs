use crate::pac;
use crate::typestates::{Disabled, Enabled, InitState};
use crate::uart::UartConfig;
use core::convert::TryInto;
use core::marker::PhantomData;
use core::ops::Shr;


pub struct Clock<State: InitState> {
    _state: PhantomData<State>,
    cpu_freq: i32,
}

impl<State> Clock<State>
where
    State: InitState,
{
    pub fn new() -> Clock<Disabled> {
        Clock {
            _state: PhantomData,
            cpu_freq: 0,
        }
    }
}

impl Clock<Disabled> {
    fn compute_pllp(cpu_freq: i32) -> u8 {
        const FCCO_MIN: i32 = 156_000_000;
        const FCCO_MAX: i32 = 320_000_000;
        for p in 0u8..3 {
            let fcco = cpu_freq * 2 * 2i32.pow(p as u32);
            if fcco >= FCCO_MIN && fcco <= FCCO_MAX {
                return p;
            }
        }
        return 0;
    }

    pub fn enable(self, cpu_freq: i32, crystal_freq: i32) -> Clock<Enabled> {
        let m: u8 = ((cpu_freq / crystal_freq) - 1).try_into().unwrap();
        let p: u8 = Clock::<Disabled>::compute_pllp(cpu_freq);
        let syscon = unsafe { &(*pac::SYSCON::ptr()) };
        // Enable main oscillator
        syscon.scs.write(|w| w.oscen().enabled());
        // Wait until main oscillator is ready
        while syscon.scs.read().oscstat().bit_is_clear() {}
        syscon.clksrcsel.write(|w| w.clksrc().main_oscillator());

        // Setup PLL0 configuration
        unsafe {
            syscon.pll0cfg.write(|w| w.msel().bits(m).psel().bits(p));
            syscon.pll0con.write(|w| w.plle().set_bit());
            syscon.pll0feed.write(|w| w.pllfeed().bits(0xAA));
            syscon.pll0feed.write(|w| w.pllfeed().bits(0x55));
        }
        // Wait until PLL0 is locked to configured frequency
        while syscon.pll0stat.read().plock().bit_is_clear() {}

        unsafe {
            let divider = 1;
            // Setup clock divider
            syscon
                .cclksel
                .write(|w| w.cclksel().set_bit().cclkdiv().bits(divider));
            // Setup peripheral clock divider
            syscon.pclksel.write(|w| w.bits(divider as u32));

            // Enable boost to allow frequencies up to 120MHz
            syscon.pboost.write(|w| w.boost().bits(3));
            // Setup how many ticks flash operation take
            syscon.flashcfg.write(|w| w.bits(0x403A));
        }

        Clock {
            _state: PhantomData,
            cpu_freq,
        }
    }
}

impl Clock<Enabled> {
    pub fn get_frequency(&self) -> i32 {
        self.cpu_freq
    }

    fn abs(a: f32) -> f32 { if a < 0f32 { -a } else { a } }

    pub fn get_uart_config(&self, baudrate: i32) -> UartConfig {
        let dl_est = self.cpu_freq as f32 / (16.0 * 1.5 * baudrate as f32);
        let f_est = self.cpu_freq as f32 / (16.0 * dl_est * baudrate as f32);
        let mut config = UartConfig {
            dlm: (dl_est as u32).shr(8) & 0xFFu32,
            dll: (dl_est as u32) & 0xFF,
            mul: 2u8,
            div: 1u8
        };
        let mut diff = 1000f32;
        for d in 0..14 {
            for m in 1..15 {
                let dd = Clock::<Enabled>::abs(f_est - (1f32 + (d as f32) / (m as f32)));
                if dd < diff {
                    diff = dd;
                    config.mul = m;
                    config.div = d;
                }
            }
        }
        return config;
    }
}
