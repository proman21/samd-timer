pub use crate::target_device::PM as MasterClock;

use crate::target_device::{
    TCC0,
    TCC1,
    TCC2
};

use atsamd_hal::samd21::clock::{
    Tcc0Tcc1Clock,
    Tcc2Tc3Clock
};

impl super::ControlTimerPeripheral for TCC0 {
    type GenericClock = Tcc0Tcc1Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbcmask.modify(|_, r| r.tcc0_().set_bit());
    }
}

impl super::ControlTimerPeripheral for TCC1 {
    type GenericClock = Tcc0Tcc1Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbcmask.modify(|_, r| r.tcc1_().set_bit());
    }
}

impl super::ControlTimerPeripheral for TCC2 {
    type GenericClock = Tcc2Tc3Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbcmask.modify(|_, r| r.tcc2_().set_bit());
    }
}