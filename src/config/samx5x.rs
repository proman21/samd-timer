use core::marker::PhantomData;

use crate::config::*;

use crate::target_device::tc0::RegisterBlock;

use crate::target_device::MCLK;
use crate::target_device::{
    TCC0,
    TCC1,
    TCC2,
    TC0,
    TC1,
    TC2,
    TC3,
};
#[cfg(not(feature = "samd51g19a"))]
use crate::target_device::{
    TCC3,
    TCC4,
    TC4,
    TC5,
};

use atsamd_hal::samd51::clock::{
    Tcc0Tcc1Clock,
    Tcc2Tcc3Clock,
    Tc0Tc1Clock,
    Tc2Tc3Clock,
};

#[cfg(not(feature = "samd51g19a"))]
use atsamd_hal::samd51::clock::{
    Tc4Tc5Clock,
    Tcc4Clock,
};

tcc!(
    MCLK,
    (tcc0, Tcc0Tcc1Clock, TCC0, apbbmask),
    (tcc1, Tcc0Tcc1Clock, TCC1, apbbmask),
    (tcc2, Tcc2Tcc3Clock, TCC2, apbcmask),
);

tc!(
    MCLK,
    (tc0, Tc0Tc1Clock, TC0, apbamask),
    (tc1, Tc0Tc1Clock, TC1, apbamask),
    (tc2, Tc2Tc3Clock, TC2, apbbmask),
    (tc3, Tc2Tc3Clock, TC3, apbbmask),
);

#[cfg(not(feature = "samd51g19a"))]
tc!(
    MCLK,
    (tc4, Tc4Tc5Clock, TC4, apbcmask),
    (tc5, Tc4Tc5Clock, TC5, apbcmask),
);

#[cfg(not(feature = "samd51g19a"))]
tcc!(
    MCLK,
    (tcc3, Tcc2Tcc3Clock, TCC3, apbcmask),
    (tcc4, Tcc4Clock, TCC4, apbdmask),
);

/// Type marker for [`Timer`](samd_timer::Timer) specific configuration.
#[derive(Default)]
pub struct TC<C: CountMode> {
    wave_gen: TimerWaveGen,
    capture_0: bool,
    capture_1: bool,
    capture_mode_0: CaptureMode,
    capture_mode_1: CaptureMode,
    capture_trig_0: CaptureTrigger,
    capture_trig_1: CaptureTrigger,
    on_demand_clock: bool,
    wave_invert_0: bool,
    wave_invert_1: bool,
    mode: PhantomData<C>,
}

impl<C: CountMode> TimerConfig<TC<C>> {
    /// Select the waveform generation mode.
    pub fn wave_gen(&mut self, value: TimerWaveGen) -> &mut Self {
        self.kind.wave_gen = value;
        self
    }

    /// Use channel 0 for counter capture.
    pub fn cc0_capture(&mut self) -> &mut Self {
        self.kind.capture_0 = true;
        self
    }

    /// Use channel 1 for counter capture.
    pub fn cc1_capture(&mut self) -> &mut Self {
        self.kind.capture_1 = true;
        self
    }

    /// Use channel 0 for counter compare.
    pub fn cc0_compare(&mut self) -> &mut Self {
        self.kind.capture_0 = false;
        self
    }

    /// Use channel 1 for counter compare.
    pub fn cc1_compare(&mut self) -> &mut Self {
        self.kind.capture_1 = false;
        self
    }

    /// Select the capture mode of channel 0.
    pub fn cc0_capture_mode(&mut self, value: CaptureMode) -> &mut Self {
        self.kind.capture_mode_0 = value;
        self
    }

    /// Select the capture mode of channel 1.
    pub fn cc1_capture_mode(&mut self, value: CaptureMode) -> &mut Self {
        self.kind.capture_mode_1 = value;
        self
    }

    /// Select the capture trigger of channel 0.
    pub fn cc0_capture_trigger(&mut self, value: CaptureTrigger) -> &mut Self {
        self.kind.capture_trig_0 = value;
        self
    }

    /// Select the capture trigger of channel 0.
    pub fn cc1_capture_trigger(&mut self, value: CaptureTrigger) -> &mut Self {
        self.kind.capture_trig_1 = value;
        self
    }

    /// Set whether or not the timer will continue to request the clock when
    /// halted.
    pub fn on_demand_clock(&mut self, value: bool) -> &mut Self {
        self.kind.on_demand_clock = value;
        self
    }

    /// Invert the waveform output of channel 0.
    pub fn waveform_invert_0(&mut self, value: bool) -> &mut Self {
        self.kind.wave_invert_0 = value;
        self
    }

    /// Invert the waveform output of channel 1.
    pub fn waveform_invert_1(&mut self, value: bool) -> &mut Self {
        self.kind.wave_invert_1 = value;
        self
    }

    pub(crate) fn configure<T: Deref<Target = RegisterBlock>>(&self, instance: &T) {
        self.generic_configure::<T>(&instance);

        tc0_mode_access!(C::MODE, instance, ctrla, modify, |_, w| unsafe {
            w.mode().bits(C::MODE as u8)
                .capten0().bit(self.kind.capture_0)
                .capten1().bit(self.kind.capture_1)
                .captmode0().bits(self.kind.capture_mode_0 as u8)
                .captmode1().bits(self.kind.capture_mode_1 as u8)
                .copen0().bit(self.kind.capture_trig_0.to_bit())
                .copen1().bit(self.kind.capture_trig_1.to_bit())
                .ondemand().bit(self.kind.on_demand_clock)
        });

        tc0_mode_access!(C::MODE, instance, drvctrl, write, |w| {
            w.inven1().bit(self.kind.wave_invert_1)
                .inven0().bit(self.kind.wave_invert_0)
        });

        tc0_mode_access!(C::MODE, instance, wave, write, |w| {
            w.wavegen().bits(self.kind.wave_gen as u8)
        });
    }
}