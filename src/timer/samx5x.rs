use core::ops::Deref;

use crate::Timer;
use crate::config::*;
pub use crate::config::{
    Count8,
    Count16,
};

use crate::target_device::tc0::RegisterBlock;

impl<T, C: CountMode> Timer<T, C> where T: Deref<Target=RegisterBlock> {
    /// Force the values stored in buffer registers to be written into their
    /// counterpart registers.
    pub fn force_buffer_update(&mut self) {
        tc0_mode_access!(C::MODE, self.tc, ctrlbset, write, |w| w.cmd().update());

        while tc0_mode_access!(C::MODE, self.tc, ctrlbset, read, cmd, is_update) {}
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

    /// Get the compare/capture value of channel 0.
    pub fn get_cc0(&self) -> C::Size {
        C::get_cc0(&self.tc)
    }

    /// Get the compare/capture value of channel 1.
    pub fn get_cc1(&self) -> C::Size {
        C::get_cc1(&self.tc)
    }

    /// Set the compare/capture value buffer of channel 0.
    pub fn set_cc0_buffer(&mut self, value: C::Size) {
        C::set_cc0_buf(&self.tc, value);
    }

    /// Get the compare/capture value buffer of channel 0.
    pub fn get_cc0_buffer(&self) -> C::Size {
        C::get_cc0_buf(&self.tc)
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