//! Timer Counter.

use core::ops::Deref;
use core::marker::PhantomData;

use crate::config::*;
pub use crate::config::{
    Count8,
    Count16,
};

#[cfg(feature = "samd51")]
use crate::target_device::tc0::RegisterBlock;

#[cfg(feature = "samd21")]
use crate::target_device::tc3::RegisterBlock;

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
/// buffer value.
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

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, enable, bit_is_set) {}
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

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, enable, bit_is_set) {}
    }

    /// Disable and reset the timer.
    pub fn reset(&mut self) {
        self.disable();

        tc0_mode_access!(C::MODE, self.tc, ctrla, modify, |_, w| w.swrst().set_bit());

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, swrst, bit_is_set) {}
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

    /// Force the values stored in buffer registers to be written into their
    /// counterpart registers.
    pub fn force_buffer_update(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| w.cmd().update());

        while tc0_mode_access!(C::MODE, self.tc, ctrlbset, read, cmd, is_update) {}
    }

    /// Return true if the timer is in oneshot mode.
    pub fn get_oneshot(&self) -> bool {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, read, oneshot, bit_is_set)
    }

    /// Turn onehsot mode on and off.
    pub fn set_oneshot(&mut self, oneshot: bool) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| w.oneshot().bit(oneshot));

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, ctrlb, bit_is_set) {}
    }

    /// Enable double buffering for period and compare values.
    /// 
    /// Values in the buffers for each of the listed configurations will be
    /// copied to the timer when hardware update conditions occur.
    pub fn enable_double_buffering(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbclr, write, |w| w.lupd().set_bit());

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, ctrlb, bit_is_set) {}
    }

    /// Disable double buffering for period and compare values.
    /// 
    /// Values in the buffers for each of the listed configurations will NOT be
    /// copied to the timer when hardware update conditions occur.
    pub fn disable_double_buffering(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| w.lupd().set_bit());

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, ctrlb, bit_is_set) {}
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

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, ctrlb, bit_is_set) {}
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

    /// Get the current value of the counter.
    /// 
    /// # Synchronisation
    /// 
    /// This operation requires synchronisation to read the value, which will block until complete.
    pub fn get_count(&self) -> C::Size {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| {
            w.cmd().readsync()
        });

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, count, bit_is_set) {}

        C::get_count(&self.tc)
    }

    /// Set the value of the timer counter.
    pub fn set_count(&mut self, value: C::Size) {
        C::set_count(&self.tc, value);

        while tc0_mode_access!(C::MODE, self.tc, syncbusy, read, count, bit_is_set) {}
    }

    /// Set the compare/capture value of channel 0.
    pub fn set_cc0(&mut self, value: C::Size) {
        C::set_cc0(&self.tc, value);
    }

    /// Get the compare/capture value of channel 0.
    pub fn get_cc0(&self) -> C::Size {
        C::get_cc0(&self.tc)
    }

    /// Set the compare/capture value buffer of channel 0.
    pub fn set_cc0_buffer(&mut self, value: C::Size) {
        C::set_cc0_buf(&self.tc, value);
    }

    /// Get the compare/capture value buffer of channel 0.
    pub fn get_cc0_buffer(&self) -> C::Size {
        C::get_cc0_buf(&self.tc)
    }

    /// Set the compare/capture value of channel 1.
    pub fn set_cc1(&mut self, value: C::Size) {
        C::set_cc1(&self.tc, value);
    }

    /// Get the compare/capture value of channel 1.
    pub fn get_cc1(&self) -> C::Size {
        C::get_cc1(&self.tc)
    }

    /// Set the compare/capture buffer value of channel 1.
    pub fn set_cc1_buffer(&mut self, value: C::Size) {
        C::set_cc1_buf(&self.tc, value);
    }

    /// Get the compare/capture buffer value of channel 1.
    pub fn get_cc1_buffer(&self) -> C::Size {
        C::get_cc1_buf(&self.tc)
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

        while self.tc.count8().syncbusy.read().per().bit_is_set() {}
    }

    /// Get period buffer value
    pub fn get_period_buffer(&self) -> u8 {
        while self.tc.count8().syncbusy.read().per().bit_is_set() {}

        self.tc.count8().perbuf.read().perbuf().bits()
    }

    /// Set period buffer value
    pub fn set_period_buffer(&mut self, perbuf: u8) {
        self.tc.count8().perbuf.write(|w| unsafe { w.perbuf().bits(perbuf) });
    }
}