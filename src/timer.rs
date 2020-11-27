use crate::clock::Clock;
use crate::pac::{TIMER0, TIMER1, TIMER2, TIMER3};
use crate::typestates::{Disabled, Enabled, InitState, NonPeriodic, Periodic, TimerType};

use core::marker::PhantomData;

use embedded_hal::timer::{Cancel, CountDown, Periodic as HalPeriodic};
use embedded_time::duration::*;
use nb::Error::WouldBlock;

pub enum TimerError {
    NotStarted,
}

macro_rules! timers {
    ($(($type: ident, $hardware: ident, $pcon: ident)),* $(,)?) => {
        $(
            pub struct $type<S: InitState, T: TimerType> {
                state: PhantomData<S>,
                typ: PhantomData<T>,
                timer: $hardware,
            }

            impl From<$hardware> for $type<Disabled, NonPeriodic> {
                fn from(timer: $hardware) -> Self {
                    $type::<Disabled, NonPeriodic> { state: PhantomData, typ: PhantomData, timer}
                }
            }

            impl $type<Disabled, NonPeriodic> {
                pub fn enable(self, clock: &Clock<Enabled>) -> $type<Enabled, NonPeriodic> {
                    self.timer.pr.write(|w| unsafe { w.pm().bits((clock.get_frequency() / 1_000_000) as u32) });
                    self.timer.mcr.write(|w| w.mr0i().set_bit().mr0s().set_bit());
                    unsafe { (*crate::pac::SYSCON::ptr()).pconp.write(|w| w.$pcon().set_bit()); };
                    $type::<Enabled, NonPeriodic> { state: PhantomData, typ: PhantomData, timer: self.timer }
                }
            }

            impl $type<Enabled, NonPeriodic> {
                pub fn into_periodic(self) -> $type<Enabled, Periodic> {
                    self.timer.mcr.write(|w| w.mr0i().set_bit().mr0r().set_bit());
                    $type::<Enabled, Periodic> { state: PhantomData, typ: PhantomData, timer: self.timer }
                }
            }

            impl $type<Enabled, Periodic> {
                pub fn into_non_periodic(self) -> $type<Enabled, NonPeriodic> {
                    self.timer.mcr.write(|w| w.mr0i().set_bit().mr0s().set_bit());
                    $type::<Enabled, NonPeriodic> { state: PhantomData, typ: PhantomData, timer: self.timer }
                }
            }

            impl<Type> CountDown for $type<Enabled, Type>
                where Type: TimerType
            {
                type Error = TimerError;
                type Time = Microseconds<u32>;

                fn try_start<T>(&mut self, count: T) -> Result<(), Self::Error> where
                    T: Into<Self::Time>
                {
                    let microseconds = count.into();
                    self.timer.tcr.write(|w| w.crst().set_bit());
                    self.timer.mr[0].write(|w| unsafe { w.bits(*microseconds.integer()) });
                    self.timer.tcr.write(|w| w.cen().set_bit());
                    Ok(())
                }

                fn try_wait(&mut self) -> nb::Result<(), Self::Error> {
                    if self.timer.ir.read().mr0int().bit_is_set() {
                        self.timer.ir.write(|w| w.mr0int().clear_bit());
                        Ok(())
                    } else {
                        Err(WouldBlock)
                    }
                }
            }

            impl<T> Cancel for $type<Enabled, T>
                where T: TimerType
            {
                fn try_cancel(&mut self) -> Result<(), Self::Error> {
                    if self.timer.tcr.read().cen().bit_is_set() {
                        self.timer.tcr.write(|w| w.crst().set_bit());
                        Ok(())
                    } else {
                        Err(TimerError::NotStarted)
                    }
                }
            }

            impl HalPeriodic for $type<Enabled, Periodic> { }
        )*
    }
}

timers!(
    (Timer0, TIMER0, pctim0),
    (Timer1, TIMER1, pctim1),
    (Timer2, TIMER2, pctim2),
    (Timer3, TIMER3, pctim3),
);