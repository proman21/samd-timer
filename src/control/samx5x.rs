pub use crate::target_device::MCLK as MasterClock;
use crate::target_device::{
    TCC0,
    TCC1,
    TCC2
};
#[cfg(not(feature = "samd51g19a"))]
use crate::target_device::{
    TCC3,
    TCC4
};

use atsamd_hal::samd51::clock::{
    Tcc0Tcc1Clock,
    Tcc2Tcc3Clock
};

#[cfg(not(feature = "samd51g19a"))]
use atsamd_hal::samd51::clock::Tcc4Clock;

impl super::ControlTimerPeripheral for TCC0 {
    type GenericClock = Tcc0Tcc1Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbbmask.modify(|_, r| r.tcc0_().set_bit());
    }
}

impl super::ControlTimerPeripheral for TCC1 {
    type GenericClock = Tcc0Tcc1Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbbmask.modify(|_, r| r.tcc1_().set_bit());
    }
}

impl super::ControlTimerPeripheral for TCC2 {
    type GenericClock = Tcc2Tcc3Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbcmask.modify(|_, r| r.tcc2_().set_bit());
    }
}

#[cfg(not(feature = "samd51g19a"))]
impl super::ControlTimerPeripheral for TCC3 {
    type GenericClock = Tcc2Tcc3Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbcmask.modify(|_, r| r.tcc3_().set_bit());
    }
}

#[cfg(not(feature = "samd51g19a"))]
impl super::ControlTimerPeripheral for TCC3 {
    type GenericClock = Tcc4Clock;

    fn configure_clocks(mclk: &mut MasterClock, _clock: &GenericClock) {
        mclk.apbdmask.modify(|_, r| r.tcc4_().set_bit());
    }
}

