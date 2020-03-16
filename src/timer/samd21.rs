use core::ops::Deref;
use core::mem;
use crate::Timer;
use crate::config::CountMode;

use crate::target_device::tc3::RegisterBlock;

impl<T, C: CountMode> Timer<T, C> where T: Deref<Target=RegisterBlock> {
    /// Get the current value of the counter.
    /// 
    /// # Synchronisation
    /// 
    /// This operation requires synchronisation to read the value, which will block until complete.
    pub fn get_count(&self) -> C::Size {
        tc0_mode_access!(C::MODE, self.tc, readreq, write, |w| unsafe {
            w.addr().bits(0x10)
                .rreq().set_bit( )
        });

        while tc0_mode_access!(C::MODE, self.tc, status, read, syncbusy, bit_is_set) {}

        C::get_count(&self.tc)
    }

    /// Get the compare/capture value of channel 0.
    pub fn get_cc0(&self) -> C::Size {
        tc0_mode_access!(C::MODE, self.tc, readreq, write, |w| unsafe {
            w.addr().bits(0x18)
                .rreq().set_bit( )
        });

        while tc0_mode_access!(C::MODE, self.tc, status, read, syncbusy, bit_is_set) {}

        C::get_cc0(&self.tc)
    }

    /// Get the compare/capture value of channel 1.
    pub fn get_cc1(&self) -> C::Size {
        tc0_mode_access!(C::MODE, self.tc, readreq, write, |w| unsafe {
            w.addr().bits(0x18 + mem::size_of::<C::Size>() as u8)
                .rreq().set_bit( )
        });

        while tc0_mode_access!(C::MODE, self.tc, status, read, syncbusy, bit_is_set) {}

        C::get_cc1(&self.tc)
    }
}