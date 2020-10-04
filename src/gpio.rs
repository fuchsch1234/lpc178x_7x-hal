use core::marker::PhantomData;
use crate::pac;
use crate::typestates;
use crate::typestates::gpio::direction;
use embedded_hal::digital::{OutputPin, InputPin};

macro_rules! gpio {
    ($GPIO: ident, $mod: ident, [$(($PIN: ident, $pin: ident)),+ $(,)?]) => {
        pub mod $mod {

        use super::*;
        use crate::typestates;
        use crate::typestates::gpio::direction;

        pub struct $GPIO {}

        #[allow(dead_code)]
        pub struct Target {
            $(
                pub $pin: $PIN<typestates::Gpio<direction::Unknown>>,
            )+
        }

        impl $GPIO {

            pub fn new() -> Self {
                $GPIO{}
            }

            pub fn split(self) -> Target {
                Target{
                    $(
                        $pin: $PIN{ _p: PhantomData },
                    )+
                }
            }

        }
        }
    }
}

gpio!(GPIO0, gpio0, [
    (P0_0, p0_0),
    (P0_1, p0_1),
    (P0_2, p0_2),
    (P0_3, p0_3),
    (P0_4, p0_4),
    (P0_5, p0_5),
    (P0_6, p0_6),
    (P0_7, p0_7),
    (P0_8, p0_8),
    (P0_9, p0_9),
    (P0_10, p0_10),
    (P0_11, p0_11),
    (P0_12, p0_12),
    (P0_13, p0_13),
    (P0_14, p0_14),
    (P0_15, p0_15),
    (P0_16, p0_16),
    (P0_17, p0_17),
    (P0_18, p0_18),
    (P0_19, p0_19),
    (P0_20, p0_20),
    (P0_21, p0_21),
    (P0_22, p0_22),
    (P0_23, p0_23),
    (P0_24, p0_24),
    (P0_25, p0_25),
    (P0_26, p0_26),
    (P0_27, p0_27),
    (P0_28, p0_28),
    (P0_29, p0_29),
    (P0_30, p0_30),
    (P0_31, p0_31),
]);

gpio!(GPIO1, gpio1, [
    (P1_0, p1_0),
    (P1_1, p1_1),
    (P1_2, p1_2),
    (P1_3, p1_3),
    (P1_4, p1_4),
    (P1_5, p1_5),
    (P1_6, p1_6),
    (P1_7, p1_7),
    (P1_8, p1_8),
    (P1_9, p1_9),
    (P1_10, p1_10),
    (P1_11, p1_11),
    (P1_12, p1_12),
    (P1_13, p1_13),
    (P1_14, p1_14),
    (P1_15, p1_15),
    (P1_16, p1_16),
    (P1_17, p1_17),
    (P1_18, p1_18),
    (P1_19, p1_19),
    (P1_20, p1_20),
    (P1_21, p1_21),
    (P1_22, p1_22),
    (P1_23, p1_23),
    (P1_24, p1_24),
    (P1_25, p1_25),
    (P1_26, p1_26),
    (P1_27, p1_27),
    (P1_28, p1_28),
    (P1_29, p1_29),
    (P1_30, p1_30),
    (P1_31, p1_31),
]);

gpio!(GPIO2, gpio2, [
    (P2_0, p2_0),
    (P2_1, p2_1),
    (P2_2, p2_2),
    (P2_3, p2_3),
    (P2_4, p2_4),
    (P2_5, p2_5),
    (P2_6, p2_6),
    (P2_7, p2_7),
    (P2_8, p2_8),
    (P2_9, p2_9),
    (P2_10, p2_10),
    (P2_11, p2_11),
    (P2_12, p2_12),
    (P2_13, p2_13),
    (P2_14, p2_14),
    (P2_15, p2_15),
    (P2_16, p2_16),
    (P2_17, p2_17),
    (P2_18, p2_18),
    (P2_19, p2_19),
    (P2_20, p2_20),
    (P2_21, p2_21),
    (P2_22, p2_22),
    (P2_23, p2_23),
    (P2_24, p2_24),
    (P2_25, p2_25),
    (P2_26, p2_26),
    (P2_27, p2_27),
    (P2_28, p2_28),
    (P2_29, p2_29),
    (P2_30, p2_30),
    (P2_31, p2_31),
]);

gpio!(GPIO3, gpio3, [
    (P3_0, p3_0),
    (P3_1, p3_1),
    (P3_2, p3_2),
    (P3_3, p3_3),
    (P3_4, p3_4),
    (P3_5, p3_5),
    (P3_6, p3_6),
    (P3_7, p3_7),
    (P3_8, p3_8),
    (P3_9, p3_9),
    (P3_10, p3_10),
    (P3_11, p3_11),
    (P3_12, p3_12),
    (P3_13, p3_13),
    (P3_14, p3_14),
    (P3_15, p3_15),
    (P3_16, p3_16),
    (P3_17, p3_17),
    (P3_18, p3_18),
    (P3_19, p3_19),
    (P3_20, p3_20),
    (P3_21, p3_21),
    (P3_22, p3_22),
    (P3_23, p3_23),
    (P3_24, p3_24),
    (P3_25, p3_25),
    (P3_26, p3_26),
    (P3_27, p3_27),
    (P3_28, p3_28),
    (P3_29, p3_29),
    (P3_30, p3_30),
    (P3_31, p3_31),
]);

gpio!(GPIO4, gpio4, [
    (P4_0, p4_0),
    (P4_1, p4_1),
    (P4_2, p4_2),
    (P4_3, p4_3),
    (P4_4, p4_4),
    (P4_5, p4_5),
    (P4_6, p4_6),
    (P4_7, p4_7),
    (P4_8, p4_8),
    (P4_9, p4_9),
    (P4_10, p4_10),
    (P4_11, p4_11),
    (P4_12, p4_12),
    (P4_13, p4_13),
    (P4_14, p4_14),
    (P4_15, p4_15),
    (P4_16, p4_16),
    (P4_17, p4_17),
    (P4_18, p4_18),
    (P4_19, p4_19),
    (P4_20, p4_20),
    (P4_21, p4_21),
    (P4_22, p4_22),
    (P4_23, p4_23),
    (P4_24, p4_24),
    (P4_25, p4_25),
    (P4_26, p4_26),
    (P4_27, p4_27),
    (P4_28, p4_28),
    (P4_29, p4_29),
    (P4_30, p4_30),
    (P4_31, p4_31),
]);

gpio!(GPIO5, gpio5, [
    (P5_0, p5_0),
    (P5_1, p5_1),
    (P5_2, p5_2),
    (P5_3, p5_3),
    (P5_4, p5_4),
]);

macro_rules! pins {
    ($clr: ident, $set: ident, $pin: ident, $dir: ident, [$(($type: ident, $port: expr, $pin_no: expr)),+ $(,)?]) => {
        $(
        pub struct $type<S: typestates::PinState> {
            _p: PhantomData<S>,
        }

        impl<S> $type<S>
        where
            S: typestates::PinState
        {
            pub fn erase_pin(self) -> Pin<S> {
                Pin{ port: $port, pin: $pin_no, _p: PhantomData }
            }
        }

        impl OutputPin for $type<typestates::Gpio<direction::Output>> {
            type Error = ();

            fn try_set_low(&mut self) -> Result<(), Self::Error> {
                Ok(unsafe { (*pac::GPIO::ptr()).$clr.write(|w| w.bits(1 << $pin_no)); })
            }

            fn try_set_high(&mut self) -> Result<(), Self::Error> {
                Ok(unsafe { (*pac::GPIO::ptr()).$set.write(|w| w.bits(1 << $pin_no)); })
            }
        }

        impl InputPin for $type<typestates::Gpio<direction::Input>> {
            type Error = ();

            fn try_is_high(&self) -> Result<bool, Self::Error> {
                self.try_is_low().map(|v| !v)
            }

            fn try_is_low(&self) -> Result<bool, Self::Error> {
                Ok(unsafe { ((*pac::GPIO::ptr()).$pin.read().bits() & (1 << $pin_no)) == 0 })
            }
        }

        impl<D> $type<typestates::Gpio<D>>
            where
                D: direction::Direction
        {
            pub fn into_output(self) -> $type<typestates::Gpio<direction::Output>> {
                unsafe { (*pac::GPIO::ptr()).$dir.modify(|r, w| w.bits(r.bits() | (1 << $pin_no))); };
                $type::<typestates::Gpio<direction::Output>>{ _p: PhantomData }
            }

            pub fn into_input(self) -> $type<typestates::Gpio<direction::Input>> {
                unsafe { (*pac::GPIO::ptr()).$dir.modify(|r, w| w.bits(r.bits() & !(1 << $pin_no))); };
                $type::<typestates::Gpio<direction::Input>>{ _p: PhantomData }
            }
        }
        )+
    }
}

pins!(clr0, set0, pin0, dir0, [
    (P0_0, 0, 0),
    (P0_1, 0, 1),
    (P0_2, 0, 2),
    (P0_3, 0, 3),
    (P0_4, 0, 4),
    (P0_5, 0, 5),
    (P0_6, 0, 6),
    (P0_7, 0, 7),
    (P0_8, 0, 8),
    (P0_9, 0, 9),
    (P0_10, 0, 10),
    (P0_11, 0, 11),
    (P0_12, 0, 12),
    (P0_13, 0, 13),
    (P0_14, 0, 14),
    (P0_15, 0, 15),
    (P0_16, 0, 16),
    (P0_17, 0, 17),
    (P0_18, 0, 18),
    (P0_19, 0, 19),
    (P0_20, 0, 20),
    (P0_21, 0, 21),
    (P0_22, 0, 22),
    (P0_23, 0, 23),
    (P0_24, 0, 24),
    (P0_25, 0, 25),
    (P0_26, 0, 26),
    (P0_27, 0, 27),
    (P0_28, 0, 28),
    (P0_29, 0, 29),
    (P0_30, 0, 30),
    (P0_31, 0, 31),
]);

pins!(clr1, set1, pin1, dir1, [
    (P1_0, 1, 0),
    (P1_1, 1, 1),
    (P1_2, 1, 2),
    (P1_3, 1, 3),
    (P1_4, 1, 4),
    (P1_5, 1, 5),
    (P1_6, 1, 6),
    (P1_7, 1, 7),
    (P1_8, 1, 8),
    (P1_9, 1, 9),
    (P1_10, 1, 10),
    (P1_11, 1, 11),
    (P1_12, 1, 12),
    (P1_13, 1, 13),
    (P1_14, 1, 14),
    (P1_15, 1, 15),
    (P1_16, 1, 16),
    (P1_17, 1, 17),
    (P1_18, 1, 18),
    (P1_19, 1, 19),
    (P1_20, 1, 20),
    (P1_21, 1, 21),
    (P1_22, 1, 22),
    (P1_23, 1, 23),
    (P1_24, 1, 24),
    (P1_25, 1, 25),
    (P1_26, 1, 26),
    (P1_27, 1, 27),
    (P1_28, 1, 28),
    (P1_29, 1, 29),
    (P1_30, 1, 30),
    (P1_31, 1, 31),
]);

pins!(clr2, set2, pin2, dir2, [
    (P2_0, 2, 0),
    (P2_1, 2, 1),
    (P2_2, 2, 2),
    (P2_3, 2, 3),
    (P2_4, 2, 4),
    (P2_5, 2, 5),
    (P2_6, 2, 6),
    (P2_7, 2, 7),
    (P2_8, 2, 8),
    (P2_9, 2, 9),
    (P2_10, 2, 10),
    (P2_11, 2, 11),
    (P2_12, 2, 12),
    (P2_13, 2, 13),
    (P2_14, 2, 14),
    (P2_15, 2, 15),
    (P2_16, 2, 16),
    (P2_17, 2, 17),
    (P2_18, 2, 18),
    (P2_19, 2, 19),
    (P2_20, 2, 20),
    (P2_21, 2, 21),
    (P2_22, 2, 22),
    (P2_23, 2, 23),
    (P2_24, 2, 24),
    (P2_25, 2, 25),
    (P2_26, 2, 26),
    (P2_27, 2, 27),
    (P2_28, 2, 28),
    (P2_29, 2, 29),
    (P2_30, 2, 30),
    (P2_31, 2, 31),
]);

pins!(clr3, set3, pin3, dir3, [
    (P3_0, 3, 0),
    (P3_1, 3, 1),
    (P3_2, 3, 2),
    (P3_3, 3, 3),
    (P3_4, 3, 4),
    (P3_5, 3, 5),
    (P3_6, 3, 6),
    (P3_7, 3, 7),
    (P3_8, 3, 8),
    (P3_9, 3, 9),
    (P3_10, 3, 10),
    (P3_11, 3, 11),
    (P3_12, 3, 12),
    (P3_13, 3, 13),
    (P3_14, 3, 14),
    (P3_15, 3, 15),
    (P3_16, 3, 16),
    (P3_17, 3, 17),
    (P3_18, 3, 18),
    (P3_19, 3, 19),
    (P3_20, 3, 20),
    (P3_21, 3, 21),
    (P3_22, 3, 22),
    (P3_23, 3, 23),
    (P3_24, 3, 24),
    (P3_25, 3, 25),
    (P3_26, 3, 26),
    (P3_27, 3, 27),
    (P3_28, 3, 28),
    (P3_29, 3, 29),
    (P3_30, 3, 30),
    (P3_31, 3, 31),
]);

pins!(clr4, set4, pin4, dir4, [
    (P4_0, 4, 0),
    (P4_1, 4, 1),
    (P4_2, 4, 2),
    (P4_3, 4, 3),
    (P4_4, 4, 4),
    (P4_5, 4, 5),
    (P4_6, 4, 6),
    (P4_7, 4, 7),
    (P4_8, 4, 8),
    (P4_9, 4, 9),
    (P4_10, 4, 10),
    (P4_11, 4, 11),
    (P4_12, 4, 12),
    (P4_13, 4, 13),
    (P4_14, 4, 14),
    (P4_15, 4, 15),
    (P4_16, 4, 16),
    (P4_17, 4, 17),
    (P4_18, 4, 18),
    (P4_19, 4, 19),
    (P4_20, 4, 20),
    (P4_21, 4, 21),
    (P4_22, 4, 22),
    (P4_23, 4, 23),
    (P4_24, 4, 24),
    (P4_25, 4, 25),
    (P4_26, 4, 26),
    (P4_27, 4, 27),
    (P4_28, 4, 28),
    (P4_29, 4, 29),
    (P4_30, 4, 30),
    (P4_31, 4, 31),
]);

pins!(clr5, set5, pin5, dir5, [
    (P5_0, 5, 0),
    (P5_1, 5, 1),
    (P5_2, 5, 2),
    (P5_3, 5, 3),
    (P5_4, 5, 4),
]);

pub struct Pin<S: typestates::PinState> {
    port: u8,
    pin: u8,
    _p: PhantomData<S>,
}

impl OutputPin for Pin<typestates::Gpio<direction::Output>> {
    type Error = ();

    fn try_set_low(&mut self) -> Result<(), Self::Error> {
        match self.port {
            0 => Ok(unsafe { (*pac::GPIO::ptr()).clr0.write(|w| w.bits(1 << self.pin)); }),
            1 => Ok(unsafe { (*pac::GPIO::ptr()).clr1.write(|w| w.bits(1 << self.pin)); }),
            2 => Ok(unsafe { (*pac::GPIO::ptr()).clr2.write(|w| w.bits(1 << self.pin)); }),
            3 => Ok(unsafe { (*pac::GPIO::ptr()).clr3.write(|w| w.bits(1 << self.pin)); }),
            4 => Ok(unsafe { (*pac::GPIO::ptr()).clr4.write(|w| w.bits(1 << self.pin)); }),
            5 => Ok(unsafe { (*pac::GPIO::ptr()).clr5.write(|w| w.bits(1 << self.pin)); }),
            _ => Err(()),
        }
    }

    fn try_set_high(&mut self) -> Result<(), Self::Error> {
        match self.port {
            0 => Ok(unsafe { (*pac::GPIO::ptr()).set0.write(|w| w.bits(1 << self.pin)); }),
            1 => Ok(unsafe { (*pac::GPIO::ptr()).set1.write(|w| w.bits(1 << self.pin)); }),
            2 => Ok(unsafe { (*pac::GPIO::ptr()).set2.write(|w| w.bits(1 << self.pin)); }),
            3 => Ok(unsafe { (*pac::GPIO::ptr()).set3.write(|w| w.bits(1 << self.pin)); }),
            4 => Ok(unsafe { (*pac::GPIO::ptr()).set4.write(|w| w.bits(1 << self.pin)); }),
            5 => Ok(unsafe { (*pac::GPIO::ptr()).set5.write(|w| w.bits(1 << self.pin)); }),
            _ => Err(()),
        }
    }
}

impl InputPin for Pin<typestates::Gpio<direction::Input>> {
    type Error = ();

    fn try_is_high(&self) -> Result<bool, Self::Error> {
        self.try_is_low().map(|v| !v)
    }

    fn try_is_low(&self) -> Result<bool, Self::Error> {
        match self.port {
            0 => Ok(unsafe { ((*pac::GPIO::ptr()).pin0.read().bits() & (1 << self.pin)) == 0 }),
            1 => Ok(unsafe { ((*pac::GPIO::ptr()).pin1.read().bits() & (1 << self.pin)) == 0 }),
            2 => Ok(unsafe { ((*pac::GPIO::ptr()).pin2.read().bits() & (1 << self.pin)) == 0 }),
            3 => Ok(unsafe { ((*pac::GPIO::ptr()).pin3.read().bits() & (1 << self.pin)) == 0 }),
            4 => Ok(unsafe { ((*pac::GPIO::ptr()).pin4.read().bits() & (1 << self.pin)) == 0 }),
            5 => Ok(unsafe { ((*pac::GPIO::ptr()).pin5.read().bits() & (1 << self.pin)) == 0 }),
            _ => Err(()),
        }
    }
}

impl<D> Pin<typestates::Gpio<D>>
where
    D: direction::Direction
{
    pub fn into_output(self) -> Pin<typestates::Gpio<direction::Output>> {
        match self.port {
            0 => unsafe { (*pac::GPIO::ptr()).dir0.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); },
            1 => unsafe { (*pac::GPIO::ptr()).dir1.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); },
            2 => unsafe { (*pac::GPIO::ptr()).dir2.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); },
            3 => unsafe { (*pac::GPIO::ptr()).dir3.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); },
            4 => unsafe { (*pac::GPIO::ptr()).dir4.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); },
            5 => unsafe { (*pac::GPIO::ptr()).dir5.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); },
            _ => ()
        }
        Pin{ pin: self.pin, port: self.port, _p: PhantomData }
    }

    pub fn into_input(self) -> Pin<typestates::Gpio<direction::Input>> {
        match self.port {
            0 => unsafe { ( * pac::GPIO::ptr()).dir0.modify( | r, w | w.bits(r.bits() & ! (1 << self.pin))); },
            1 => unsafe { ( * pac::GPIO::ptr()).dir1.modify( | r, w | w.bits(r.bits() & ! (1 << self.pin))); },
            2 => unsafe { ( * pac::GPIO::ptr()).dir2.modify( | r, w | w.bits(r.bits() & ! (1 << self.pin))); },
            3 => unsafe { ( * pac::GPIO::ptr()).dir3.modify( | r, w | w.bits(r.bits() & ! (1 << self.pin))); },
            4 => unsafe { ( * pac::GPIO::ptr()).dir4.modify( | r, w | w.bits(r.bits() & ! (1 << self.pin))); },
            5 => unsafe { ( * pac::GPIO::ptr()).dir5.modify( | r, w | w.bits(r.bits() & ! (1 << self.pin))); },
            _ => ()
        }
        Pin{ pin: self.pin, port: self.port, _p: PhantomData }
    }
}