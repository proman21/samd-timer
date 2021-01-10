//! Timer Counter.

#[cfg(feature = "samx5x")]
macro_rules! syncbusy_check {
    ($mode:expr, $tc:expr, $flag:ident) => {
        tc0_mode_access!($mode, $tc, syncbusy, read, $flag, bit_is_set);
    };
    ($tc:expr, $flag:ident) => {
        $tc.count8().syncbusy.read().$flag().bit_is_set()
    }
}

#[cfg(feature = "samd21")]
macro_rules! syncbusy_check {
    ($mode:expr, $tc:expr, $flag:ident) => {
        tc0_mode_access!($mode, $tc, status, read, syncbusy, bit_is_set);
    };
    ($tc:expr, $flag:ident) => {
        $tc.count8().status.read().syncbusy().bit_is_set()
    }
}

macro_rules! tc_32_bit {
    ($name:ident, $m:ty, $s:ty) => {
        pub struct $name(pub $m, pub $s);

        impl Deref for $name {
            type Target = RegisterBlock;

            fn deref(&self) -> &RegisterBlock { &self.0 }
        }
    };
}

#[cfg(feature = "samd21")]
mod samd21;

#[cfg(feature = "samx5x")]
mod samx5x;

use core::ops::Deref;
use core::marker::PhantomData;

use crate::config::*;
pub use crate::config::{
    Count8,
    Count16,
};

#[cfg(feature = "samd21")]
use samd21::RegisterBlock;

#[cfg(feature = "samd21j")]
pub use samd21::TC6_7;

#[cfg(feature = "samx5x")]
use samx5x::RegisterBlock;

#[cfg(feature = "samx5x")]
pub use samx5x::{TC0_1, TC2_3};

#[cfg(not(feature = "samd51g"))]
use crate::target_device::{TC4, TC5};

#[cfg(not(feature = "samd51g"))]
tc_32_bit!(TC4_5, TC4, TC5);

bitflags! {
    /// A bitfield that describes the interrupt flags that a timer can trigger.
    #[derive(Default)]
    pub struct Interrupts: u8 {
        /// Match or Compare Channel 1
        const MC1 = 0x20;
        /// Match or Compare Channel 0
        const MC0 = 0x10;
        /// Error
        const ERR = 0x02;
        /// Overflow
        const OVF = 0x01;
    }
}

/// Timer is used to work with an instance of a TC peripheral.
/// 
/// ## 8-bit Mode
/// 
/// When using `Timer<T, Count8>` mode, you can access a period and period
/// buffer value (on SAMX5X targets).
pub struct Timer<T, C: CountMode> where T: Deref<Target=RegisterBlock> {
    tc: T,
    _mode: PhantomData<C>,
}

impl<T, C: CountMode> Timer<T, C> where T: Deref<Target=RegisterBlock> {
    pub(crate) fn new(tc: T) -> Timer<T, C> {
        Timer {
            tc,
            _mode: PhantomData
        }
    }

    /// Reset and eject the TC instance.
    /// 
    /// Use if you need the timer control structure back before drop.
    pub fn eject(mut self) -> T {
        self.reset();
        self.tc
    }

    /// Enable the timer.
    pub fn enable(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrla, modify, |r, w| {
            if r.enable().bit_is_clear() {
                w.enable().set_bit()
            } else {
                w
            }
        });

        while syncbusy_check!(C::MODE, self.tc, enable) {}
    }

    /// Disable the timer.
    pub fn disable(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrla, modify, |r, w| {
            if r.enable().bit_is_set() {
                w.enable().clear_bit()
            } else {
                w
            }
        });

        while syncbusy_check!(C::MODE, self.tc, enable) {}
    }

    /// Disable and reset the timer.
    pub fn reset(&mut self) {
        self.disable();

        tc0_mode_access!(C::MODE, self.tc, ctrla, modify, |_, w| w.swrst().set_bit());

        while syncbusy_check!(C::MODE, self.tc, swrst) {}
    }

     /// Reset and reconfigure the timer.
     pub fn reconfigure(&mut self, config: &TimerConfig<TC<C>>) {
        self.reset();

        config.configure(&self.tc);
    }

    /// Retrigger the timer.
    pub fn retrigger(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| w.cmd().retrigger());

        while tc0_mode_access!(C::MODE, self.tc, ctrlbset, read, cmd, is_retrigger) {} 
    }

    /// Stop the timer.
    pub fn stop(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| w.cmd().stop());

        while tc0_mode_access!(C::MODE, self.tc, ctrlbset, read, cmd, is_stop) {}
    }

    /// Return true if the timer is stopped.
    pub fn is_stopped(&self) -> bool {
        tc0_mode_access!(C::MODE, self.tc, status, read, stop, bit_is_set)
    }

    /// Return true if the timer is in oneshot mode.
    pub fn get_oneshot(&self) -> bool {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, read, oneshot, bit_is_set)
    }

    /// Turn onehsot mode on and off.
    pub fn set_oneshot(&mut self, oneshot: bool) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| w.oneshot().bit(oneshot));

        while syncbusy_check!(C::MODE, self.tc, ctrlb) {}
    }

    /// Enable or disable the timer interrupts specified by the given bitfield.
    pub fn enable_interrupts(&mut self, value: Interrupts) {
        tc0_mode_access!(C::MODE, self.tc, intenset, write, |w| unsafe {
            w.bits(value.bits())
        });
        tc0_mode_access!(C::MODE, self.tc, intenclr, write, |w| unsafe {
            w.bits(!value.bits())
        });
    }

    /// Get the timer instance's interrupt flags.
    pub fn interrupt_status(&self) -> Interrupts {
        Interrupts::from_bits(tc0_mode_access!(C::MODE, self.tc, intflag, read, bits)).unwrap()
    }

    /// Clear the timer instance's interrupt flags as specified by the bitfield.
    pub fn clear_interrupts(&mut self, value: Interrupts) {
        tc0_mode_access!(C::MODE, self.tc, intflag, write, |w| unsafe {
            w.bits(value.bits())
        });
    }

    /// Set the direction of counting for the timer.
    pub fn set_direction(&mut self, value: Direction) {
        match value {
            Direction::Up => tc0_mode_access!(C::MODE, self.tc, ctrlbclr, modify, |_, w| w.dir().set_bit()),
            Direction::Down => tc0_mode_access!(C::MODE, self.tc, ctrlbset, modify, |_, w| w.dir().set_bit())
        }

        while syncbusy_check!(C::MODE, self.tc, ctrlb) {}
    }

    /// Reverse the counting direction of the timer.
    pub fn toggle_direction(&mut self) {
        self.set_direction(self.get_direction().reverse());
    }

    /// Get the counting direction of the timer.
    pub fn get_direction(&self) -> Direction {
        if tc0_mode_access!(C::MODE, self.tc, ctrlbset, read, dir, bit_is_set) {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    /// Set the value of the timer counter.
    pub fn set_count(&mut self, value: C::Size) {
        C::set_count(&self.tc, value);

        while syncbusy_check!(C::MODE, self.tc, count) {}
    }

    /// Set the compare/capture value of channel 0.
    pub fn set_cc0(&mut self, value: C::Size) {
        C::set_cc0(&self.tc, value);
    }

    /// Set the compare/capture value of channel 1.
    pub fn set_cc1(&mut self, value: C::Size) {
        C::set_cc1(&self.tc, value);
    }
}

impl<T> Timer<T, Count8> where T: Deref<Target=RegisterBlock> {
    /// Get period value
    pub fn get_period(&self) -> u8 {
        self.tc.count8().per.read().per().bits()
    }

    /// Set period value
    pub fn set_period(&mut self, per: u8) {
        self.tc.count8().per.write(|w| unsafe { w.per().bits(per) });

        while syncbusy_check!(self.tc, per) {}
    }
}