//! Configuration types for `Timer` and `ControlTimer`.
//! 
//! See [`TimerConfig`](samd_timer::TimerConfig) for more information on how to
//! use the types in this module.

macro_rules! tcc {
    ($mclk:ty, $(($name:ident, $clock:ty, $instance:ty, $apb:ident),)+) => {
        impl TimerConfig<TCC> {
            $(
                /// Build and configure a TCC instance.
                pub fn $name(&self, mclk: &mut $mclk, _clock: &$clock, tcc: $instance) -> $crate::control::ControlTimer<$instance> {
                    mclk.$apb.modify(paste::expr!{ |_, r| r.[<$name _>]().set_bit() });

                    self.configure(&tcc);
                    $crate::control::ControlTimer::new(tcc)
                }
            )+
        }
    };
}

macro_rules! tc {
    ($mclk:ty, $(($name:ident, $clock:ty, $instance:ty, $apb:ident),)+) => {
        impl TimerConfig<TC<Count8>> {
            $(
                pub fn $name(&self, mclk: &mut $mclk, _clock: &$clock, tc: $instance) -> $crate::timer::Timer<$instance, Count8> {
                    mclk.$apb.modify(paste::expr!{ |_, r| r.[<$name _>]().set_bit() });

                    self.configure(&tc);
                    $crate::timer::Timer::new(tc)
                }
            )+
        }

        impl TimerConfig<TC<Count16>> {
            $(
                pub fn $name(&self, mclk: &mut $mclk, _clock: &$clock, tc: $instance) -> $crate::timer::Timer<$instance, Count16> {
                    mclk.$apb.modify(paste::expr!{ |_, r| r.[<$name _>]().set_bit() });

                    self.configure(&tc);
                    $crate::timer::Timer::new(tc)
                }
            )+
        }
    };
    ($mclk:ty, $(($name:ident, $clock:ty, $instance:ty, $apb:ident, $m:ident, $s:ident),)+) => {
        impl TimerConfig<TC<Count32>> {
            $(
                pub fn $name(&self, mclk: &mut $mclk, _clock: &$clock, tc: $instance) -> $crate::timer::Timer<$instance, Count32> {
                    mclk.$apb.modify(paste::expr!{ |_, r| {
                        r.[<$m _>]().set_bit().[<$s _>]().set_bit()
                    } });

                    self.configure(&tc);
                    $crate::timer::Timer::new(tc)
                }
            )+
        }
    };
}

#[cfg(feature = "samd21")]
mod samd21;
#[cfg(feature = "samx5x")]
mod samx5x;

mod types;

use core::ops::Deref;

use crate::target_device::tcc0::RegisterBlock as TCC_RB;
#[cfg(feature = "samx5x")]
use crate::target_device::tc0::RegisterBlock as TC_RB;
#[cfg(feature = "samd21")]
use crate::target_device::tc3::RegisterBlock as TC_RB;

#[cfg(feature = "samd21")]
pub use samd21::TC;

#[cfg(feature = "samx5x")]
pub use samx5x::TC;

pub use self::types::*;

/// Common initialisation time configuration for `Timer` and `ControlTimer`.
/// 
/// Both the TC and TCC peripherals require that some configuration values are
/// set before an instance is enabled. This type represents those values using
/// a Builder pattern. To configure a `ControlTimer`, create a default
/// configuration using `TimerConfig::control()`. For a `Timer`, use either
/// `TimerConfig::count8()` or `TimerConfig::count16()`, depending on what
/// bit-width you want the timer to use.
/// 
/// Builder methods do not consume the configuration object, so you can reuse
/// the object to configure or reconfigure other timer instances.
#[derive(Default, Clone)]
pub struct TimerConfig<KIND> {
    debug_state: bool,
    dma_oneshot: bool,
    auto_lock: bool,
    sync: Synchronization,
    run_standby: bool,
    prescaler: Prescaler,
    kind: KIND,
}

impl<KIND> TimerConfig<KIND> {
    /// Enable or disable the DMA oneshot trigger.
    pub fn dma_oneshot(&mut self, value: bool) -> &mut Self {
        self.dma_oneshot = value;
        self
    }

    /// If set, double buffering will be disabled on overflow, underflow, and
    /// re-trigger events.
    pub fn auto_lock(&mut self, value: bool) -> &mut Self {
        self.auto_lock = value;
        self
    }

    /// Chose which clock source is used to reset or reload the counter.
    pub fn sync(&mut self, value: Synchronization) -> &mut Self {
        self.sync = value;
        self
    }

    /// If set, the timer will continue operation in standby mode.
    pub fn run_standby(&mut self, value: bool) -> &mut Self {
        self.run_standby = value;
        self
    }

    /// Select which factor is used for the timer prescaler.
    pub fn prescaler(&mut self, value: Prescaler) -> &mut Self {
        self.prescaler = value;
        self
    }

    /// Halt the timer when the device enters debug mode.
    pub fn halt_on_debug(&mut self) -> &mut Self {
        self.debug_state = false;
        self
    }

    /// The timer will continue in debug mode.
    pub fn continue_on_debug(&mut self) -> &mut Self {
        self.debug_state = true;
        self
    }
}

/// Type marker for `ControlTimer` specific configuration.
#[derive(Default)]
pub struct TCC {
    debug_fault: bool,
    channels: Channels,
    dither: Dither,
    dti_output_cycles: (u8, u8),
    dti_enable: DTIEnable,
    output_config: OutputMatrixConfig,
    output_inversion: WaveformSelect,
}

impl TimerConfig<TCC> {
    /// Create a configuration for a `ControlTimer`.
    pub fn control_timer() -> TimerConfig<TCC> {
        Default::default()
    }

    /// Select how many PWM frames are between dithered values.
    pub fn dither(&mut self, value: Dither) -> &mut Self {
        self.kind.dither = value;
        self
    }

    /// Do not generate faults when device is in debug mode.
    pub fn no_fault_on_debug(&mut self) -> &mut Self {
        self.kind.debug_fault = false;
        self
    }

    /// Generate a non-recoverable fault when device is in debug mode.
    pub fn fault_on_debug(&mut self) -> &mut Self {
        self.kind.debug_fault = true;
        self
    }

    /// Set the number of cycles for the dead-time generator's high and low
    /// side, respectively.
    pub fn dti_output_cycles(&mut self, value: (u8, u8)) -> &mut Self {
        self.kind.dti_output_cycles = value;
        self
    }

    /// Enable the dead-time insertion generator for the given outputs.
    pub fn dti_enable(&mut self, value: DTIEnable) -> &mut Self {
        self.kind.dti_enable = value;
        self
    }

    /// Set the matrix routing mode of the waveform generation outputs.
    pub fn output_config(&mut self, value: OutputMatrixConfig) -> &mut Self {
        self.kind.output_config = value;
        self
    }

    /// Select which waveform outputs are inverted.
    pub fn output_inversion(&mut self, value: WaveformSelect) -> &mut Self {
        self.kind.output_inversion = value;
        self
    }

    pub(crate) fn configure<T>(&self, instance: &T) where T: Deref<Target=TCC_RB> {
        instance.ctrla.write(|w| unsafe {
            #[cfg(feature = "samx5x")]
            w.dmaos().bit(self.dma_oneshot)
                .cpten4().bit(self.kind.channels.intersects(Channels::CHAN_4))
                .cpten5().bit(self.kind.channels.intersects(Channels::CHAN_5));

            w.alock().bit(self.auto_lock)
                .prescsync().bits(self.sync as u8)
                .runstdby().bit(self.run_standby)
                .prescaler().bits(self.prescaler as u8)
                .resolution().bits(self.kind.dither as u8)
                .cpten0().bit(self.kind.channels.intersects(Channels::CHAN_0))
                .cpten1().bit(self.kind.channels.intersects(Channels::CHAN_1))
                .cpten2().bit(self.kind.channels.intersects(Channels::CHAN_2))
                .cpten3().bit(self.kind.channels.intersects(Channels::CHAN_3))
        });

        instance.dbgctrl.write(|w| w.dbgrun().bit(self.debug_state));

        instance.wexctrl.write(|w| unsafe {
            w.otmx().bits(self.kind.output_config as u8)
                .dtls().bits(self.kind.dti_output_cycles.0)
                .dths().bits(self.kind.dti_output_cycles.1)
                .dtien0().bit(self.kind.dti_enable.intersects(DTIEnable::GEN_0))
                .dtien1().bit(self.kind.dti_enable.intersects(DTIEnable::GEN_1))
                .dtien2().bit(self.kind.dti_enable.intersects(DTIEnable::GEN_2))
                .dtien3().bit(self.kind.dti_enable.intersects(DTIEnable::GEN_3))
        });

        instance.drvctrl.write(|w|
            w.inven0().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_0))
                .inven1().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_1))
                .inven2().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_2))
                .inven3().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_3))
                .inven4().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_4))
                .inven5().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_5))
                .inven6().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_6))
                .inven7().bit(self.kind.output_inversion.intersects(WaveformSelect::WO_7)));
    }
}

/// Trait for the different counter widths of the `Timer` peripheral.
/// 
/// This is a sealed trait that provides an interface for the `Timer`
/// implementation. It is used by [`Count8`] and [`Count16`].
/// 
/// [`Count8`]: samd_timer::Count8
/// [`Count16`]: samd_timer::Count16
pub trait CountMode: Default + crate::private::Sealed {
    const MODE: TimerMode;
    type Size;

    fn get_count(rb: &TC_RB) -> Self::Size;
    fn set_count(rb: &TC_RB, value: Self::Size);
    fn get_cc0(rb: &TC_RB) -> Self::Size;
    fn set_cc0(rb: &TC_RB, value: Self::Size);
    fn get_cc1(rb: &TC_RB) -> Self::Size;
    fn set_cc1(rb: &TC_RB, value: Self::Size);
    #[cfg(feature = "samx5x")]
    fn get_cc0_buf(rb: &TC_RB) -> Self::Size;
    #[cfg(feature = "samx5x")]
    fn set_cc0_buf(rb: &TC_RB, value: Self::Size);
    #[cfg(feature = "samx5x")]
    fn get_cc1_buf(rb: &TC_RB) -> Self::Size;
    #[cfg(feature = "samx5x")]
    fn set_cc1_buf(rb: &TC_RB, value: Self::Size);
}

macro_rules! count_mode {
    ($(($name:ident, $mode:expr, $size:ty, $reg:ident)),+) => {
        $(
            #[derive(Default)]
            /// Marker type for the TC count mode.
            pub struct $name;

            impl CountMode for $name {
                const MODE: TimerMode = $mode;
                type Size = $size;

                fn get_count(rb: &TC_RB) -> Self::Size {
                    rb.$reg().count.read().count().bits()
                }
            
                fn set_count(rb: &TC_RB, value: Self::Size){
                    rb.$reg().count.write(|w| unsafe { w.count().bits(value) });
                }
            
                fn get_cc0(rb: &TC_RB) -> Self::Size {
                    rb.$reg().cc[0].read().cc().bits()
                }
            
                fn set_cc0(rb: &TC_RB, value: Self::Size) {
                    rb.$reg().cc[0].write(|w| unsafe { w.cc().bits(value) });
                }
            
                fn get_cc1(rb: &TC_RB) -> Self::Size {
                    rb.$reg().cc[1].read().cc().bits()
                }
            
                fn set_cc1(rb: &TC_RB, value: Self::Size) {
                    rb.$reg().cc[1].write(|w| unsafe { w.cc().bits(value) });
                }
            
                #[cfg(feature = "samx5x")]
                fn get_cc0_buf(rb: &TC_RB) -> Self::Size {
                    rb.$reg().ccbuf[0].read().ccbuf().bits()
                }
            
                #[cfg(feature = "samx5x")]
                fn set_cc0_buf(rb: &TC_RB, value: Self::Size) {
                    rb.$reg().ccbuf[0].write(|w| unsafe { w.ccbuf().bits(value) });
                }
            
                #[cfg(feature = "samx5x")]
                fn get_cc1_buf(rb: &TC_RB) -> Self::Size {
                    rb.$reg().ccbuf[1].read().ccbuf().bits()
                }
            
                #[cfg(feature = "samx5x")]
                fn set_cc1_buf(rb: &TC_RB, value: Self::Size) {
                    rb.$reg().ccbuf[1].write(|w| unsafe { w.ccbuf().bits(value) });
                }
            }
        )+
    };
}

count_mode!(
    (Count8, TimerMode::Count8, u8, count8),
    (Count16, TimerMode::Count16, u16, count16),
    (Count32, TimerMode::Count32, u32, count32)
);

impl<C: CountMode> TimerConfig<TC<C>> {
    /// Create a configuration for `Timer` with an 8-bit counter.
    pub fn count8() -> TimerConfig<TC<Count8>> {
        Default::default()
    }

    /// Create a configuration for `Timer` with a 16-bit counter.
    pub fn count16() -> TimerConfig<TC<Count16>> {
        Default::default()
    }

    /// Create a configuration for `Timer` with a 32-bit counter.
    pub fn count32() -> TimerConfig<TC<Count32>> {
        Default::default()
    }

    pub(crate) fn generic_configure<T>(&self, instance: &T)
    where T: Deref<Target=TC_RB>
    {
        tc0_mode_access!(C::MODE, instance, ctrla, write, |w| unsafe {
            #[cfg(feature = "samx5x")]
            w.alock().bit(self.auto_lock);

            w.prescsync().bits(self.sync as u8)
                .runstdby().bit(self.run_standby)
                .prescaler().bits(self.prescaler as u8)
        });

        tc0_mode_access!(C::MODE, instance, dbgctrl, write, |w| {
            w.dbgrun().bit(self.debug_state)
        });
    }
}