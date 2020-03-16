use core::marker::PhantomData;
use core::ops::Deref;

use crate::*;
use crate::config::{CountMode, TimerWaveGen};

use crate::target_device::PM;
use crate::target_device::tc3::RegisterBlock;
use crate::target_device::{
    TCC0,
    TCC1,
    TCC2,
    TC3,
    TC4,
    TC5
};
#[cfg(feature = "samd21j18a")]
use crate::target_device::{
    TC6,
    TC7,
};

use atsamd_hal::samd21::clock::{
    Tcc0Tcc1Clock,
    Tcc2Tc3Clock,
    Tc4Tc5Clock,
};

tcc!(
    PM,
    (tcc0, Tcc0Tcc1Clock, TCC0, apbcmask),
    (tcc1, Tcc0Tcc1Clock, TCC1, apbcmask),
    (tcc2, Tcc2Tc3Clock, TCC2, apbcmask),
);

tc!(
    PM,
    (tc3, Tcc2Tc3Clock, TC3, apbcmask),
    (tc4, Tc4Tc5Clock, TC4, apbcmask),
    (tc5, Tc4Tc5Clock, TC5, apbcmask),
);

#[cfg(feature = "samd21j18a")]
use atsamd_hal::samd21::clock::Tc6Tc7Clock;

#[cfg(feature = "samd21j18a")]
tc!(
    PM,
    (tc6, Tc6Tc7Clock, TC6, apbcmask),
    (tc7, Tc6Tc7Clock, TC7, apbcmask),
);

/// Type marker for [`Timer`](samd_timer::Timer) specific configuration.
#[derive(Default)]
pub struct TC<C: CountMode> {
    wave_gen: TimerWaveGen,
    mode: PhantomData<C>,
}

impl<C: CountMode> TimerConfig<TC<C>> {
    pub fn wave_gen(&mut self, value: TimerWaveGen) -> &mut Self {
        self.kind.wave_gen = value;
        self
    }

    pub(crate) fn configure<T: Deref<Target = RegisterBlock>>(&self, instance: &T) {
        self.generic_configure::<T>(&instance);

        tc0_mode_access!(C::MODE, instance, ctrla, modify, |_, w| unsafe {
            w.mode().bits(C::MODE as u8)
                .wavegen().bits(self.kind.wave_gen as u8)
        });
    }
}