use core::marker::PhantomData;
use gpio::direction::Direction;

mod sealed {
    pub trait Sealed {}
}

pub trait InitState: sealed::Sealed {}

pub struct Enabled;
impl sealed::Sealed for Enabled {}
impl InitState for Enabled {}

pub struct Disabled;
impl sealed::Sealed for Disabled {}
impl InitState for Disabled {}

pub trait PinState: sealed::Sealed {}

pub struct Gpio<D: Direction> {
    _p: PhantomData<D>,
}
impl<D> sealed::Sealed for Gpio<D> where D: Direction {}
impl<D> PinState for Gpio<D> where D: Direction {}

pub mod gpio {
    pub mod direction {
        pub trait Direction: crate::typestates::sealed::Sealed {}

        pub struct Unknown;
        impl crate::typestates::sealed::Sealed for Unknown {}
        impl Direction for Unknown {}

        pub struct Input;
        impl crate::typestates::sealed::Sealed for Input {}
        impl Direction for Input {}

        pub struct Output;
        impl crate::typestates::sealed::Sealed for Output {}
        impl Direction for Output {}
    }
}