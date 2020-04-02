//! # samd-timer
//! 
//! This library provides a type-safe API for the Timer/Counter and
//! Timer/Counter for Control peripherals that are present on Microchip SAM
//! series microcontrollers.
//! 
//! ## Configuration
//! 
//! This library uses a builder pattern to turn a timer configuration into a
//! timer instance. Configurations are not consumed upon building and can be
//! used to configure multiple timer instances.
//! 
//! ## Clocks
//! 
//! This library uses the clock system provided by the `atsamd-hal` crate in
//! order to make configuring the clock system as easy as possible.
//! 
//! ## Example
//! 
//! ```
//! use target_device::Peripherals;
//! use atsamd_hal::clock::GenericClockController;
//! 
//! let mut peri = Peripherals::take().unwrap();
//! 
//! // Configure the HAL clock controller.
//! let mut clock_ctrl = GenericClockController::with_internal_32kosc(
//!     peri.GCLK,
//!     &mut peri.MCLK,
//!     &mut peri.OSC32KCTRL,
//!     &mut peri.OSCCTRL,
//!     &mut peri.NVMCTRL
//! );
//!
//! // Assign 120MHz clock to TCC0
//! let fast_clock = clock_ctrl.gclk0();
//! let timer_clk = clock_ctrl.tcc0_tcc1(&fast_clock).unwrap();
//! 
//! let mut config = TimerConfig::control();
//! 
//! config.prescaler(Prescaler::Div64)
//!     .sync(Synchronisation::Prescaler);
//! 
//! let timer = config.tcc0(&mut peri.MCLK, &timer_clk, peri.TCC0);
//! ```
//! 
//! The last line of the above example shows how you can build an instance of
//! a `ControlTimer` by using one of the instance specific methods. These
//! methods take care of setting up the relavent clocks for the peripheral
//! instance.
#![no_std]

#![allow(dead_code)]

#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate bitflags;
extern crate atsamd_hal;

#[cfg(feature = "samd21g18a")]
use atsamd21g18a as target_device;

#[cfg(feature = "samd21e18a")]
use atsamd21e18a as target_device;

#[cfg(feature = "samd21j18a")]
use atsamd21j18a as target_device;

#[cfg(feature = "samd51j19a")]
use atsamd51j19a as target_device;

#[cfg(feature = "samd51j20a")]
use atsamd51j20a as target_device;

#[cfg(feature = "samd51g19a")]
use atsamd51g19a as target_device;

macro_rules! tc0_mode_access {
    ($mode:expr, $rb:expr, $reg:ident, read, $($p:ident),+) => {
        {
            use $crate::config::TimerMode::*;
            match $mode {
                Count8 => $rb.count8().$reg.read()$(.$p())+,
                Count16 => $rb.count16().$reg.read()$(.$p())+
            }
        }
    };
    ($mode:expr, $rb:expr, $reg:ident, $op:ident, $c:expr) => {
        {
            use $crate::config::TimerMode::*;
            match $mode {
                Count8 => $rb.count8().$reg.$op($c),
                Count16 => $rb.count16().$reg.$op($c)
            }
        }
    };
}

pub mod config;
pub mod control;
pub mod timer;

pub use config::{TimerConfig, TC, TCC, Count8, Count16};
pub use control::ControlTimer;
pub use timer::Timer;

mod private {
    use super::{TC, TCC, Count8, Count16, Count32, config::CountMode};
    
    pub trait Sealed {}
    
    impl Sealed for TCC {}
    impl<C: CountMode> Sealed for TC<C> {}
    impl Sealed for Count8 {}
    impl Sealed for Count16 {}
}