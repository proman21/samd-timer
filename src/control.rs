//! Timer Counter for Control applications.
use core::convert::TryInto;
use core::ops::Deref;

use crate::config::*;
pub use crate::config::{
    WaveformSelect,
};

use crate::target_device::{TCC0, TCC1};
use crate::target_device::tcc0::{
    RegisterBlock,
    ctrla::RESOLUTION_A,
    ctrlbset::CMD_A
};

macro_rules! tcc_reg_res {
    ($tcc:expr, $reg:ident, $op:ident, $c:expr) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => $tcc.$reg().$op($c),
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }.$op($c),
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }.$op($c),
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }.$op($c),
        }
    };
    (dither, $tcc:expr, $reg:ident, $op:ident, $c:expr) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => {},
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }.$op($c),
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }.$op($c),
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }.$op($c),
        }
    };
    ($tcc:expr, $reg:ident, read, $($ops:ident),+) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => $tcc.$reg().read()$(.$ops())+,
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }.read()$(.$ops())+,
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }.read()$(.$ops())+,
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }.read()$(.$ops())+,
        }
    };
    (dither, $tcc:expr, $reg:ident, read, $($ops:ident),+) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => 0,
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }.read()$(.$ops())+,
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }.read()$(.$ops())+,
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }.read()$(.$ops())+,
        }
    };
    ($tcc:expr, $reg:ident, $index:expr, $op:ident, $c:expr) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => $tcc.$reg()[$index].$op($c),
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }[$index].$op($c),
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }[$index].$op($c),
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }[$index].$op($c),
        }
    };
    (dither, $tcc:expr, $reg:ident, $index:expr, $op:ident, $c:expr) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => {},
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }[$index].$op($c),
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }[$index].$op($c),
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }[$index].$op($c),
        }
    };
    ($tcc:expr, $reg:ident, $index:expr, read, $($ops:ident),+) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => $tcc.$reg()[$index].read()$(.$ops())+,
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }[$index].read()$(.$ops())+,
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }[$index].read()$(.$ops())+,
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }[$index].read()$(.$ops())+,
        }
    };
    (dither, $tcc:expr, $reg:ident, $index:expr, read, $($ops:ident),+) => {
        match $tcc.ctrla.read().resolution().variant() {
            RESOLUTION_A::NONE => 0,
            RESOLUTION_A::DITH4 => paste::expr!{ $tcc.[<$reg _dith4>]() }[$index].read()$(.$ops())+,
            RESOLUTION_A::DITH5 => paste::expr!{ $tcc.[<$reg _dith5>]() }[$index].read()$(.$ops())+,
            RESOLUTION_A::DITH6 => paste::expr!{ $tcc.[<$reg _dith6>]() }[$index].read()$(.$ops())+,
        }
    };
}

bitflags! {
    /// A bitfield that describes the interrupt flags that a timer can trigger.
    #[derive(Default)]
    pub struct Interrupts: u32 {
        /// Match or Capture Channel 3
        const MC3 = 0x80000;
        /// Match or Capture Channel 2
        const MC2 = 0x40000;
        /// Match or Capture Channel 1
        const MC1 = 0x20000;
        /// Match or Capture Channel 0
        const MC0 = 0x10000;
        /// Non-Recoverable Fault 1
        const FAULT1 = 0x08000;
        /// Non-Recoverable Fault 0
        const FAULT0 = 0x04000;
        /// Recoverable Fault B
        const FAULTB = 0x02000;
        /// Recoverable Fault A
        const FAULTA = 0x01000;
        /// Non-Recoverable Debug Fault State
        const DFS = 0x00800;
        /// Non-Recoverable Update Fault
        const UFS = 0x00400;
        /// Error
        const ERR = 0x00008;
        /// Counter
        const CNT = 0x00004;
        /// Retrigger
        const TRG = 0x00002;
        /// Overflow
        const OVF = 0x00001;
    }
}

/// Possible waveform generation modes for a TCC instance.
#[derive(SmartDefault, Debug)]
pub enum WaveformGen {
    /// Normal Frequency (Default)
    #[default]
    NFRQ = 0,
    /// Match Frequency
    MFRQ,
    /// Normal PWM
    NPWM,
    /// Dual-Slope Critical PWM
    DSCritical = 4,
    /// Dual-Slope Bottom PWM
    DSBottom,
    /// Dual-Slope Both PWM
    DSBoth,
    /// Dual-Slope Top PWM
    DSTop
}

/// `ControlTimer` is used to work with a TCC peripheral instance.
/// 
/// All configuration options that are not configurable when the instance is
/// enabled are found in the `TimerConfig<TCC>` type. You can use one of the
/// `tccx` methods on `TimerConfig<TCC>` to configure a new instance of
/// `ControlTimer`, or use the `reconfigure` method on `ControlTimer` to
/// reconfigure an existing timer instance.
/// 
/// # Timer Width
/// 
/// The size of all timer value accesses are always 32-bit, even if the timer
/// instance is 24-bit or 16-bit wide. As per the reference manual, the upper
/// bytes will be read as zero.
pub struct ControlTimer<T>{
    tcc: T
}

impl<T> ControlTimer<T> where T: Deref<Target=RegisterBlock> {
    pub(crate) fn new(tcc: T) -> ControlTimer<T> {
        ControlTimer {
            tcc
        }
    }

    /// Reset and eject the TCC instance.
    /// 
    /// Use if you need the timer control structure back before drop.
    pub fn eject(mut self) -> T {
        self.reset();
        self.tcc
    }

    /// Enable the timer.
    pub fn enable(&mut self) {
        self.tcc.ctrla.modify(|r, w| {
            if r.enable().bit_is_clear() {
                w.enable().set_bit()
            } else {
                w
            }
        });

        while self.tcc.syncbusy.read().enable().bit_is_set() {  }
    }

    /// Disable the timer.
    pub fn disable(&mut self) {
        self.tcc.ctrla.modify(|r, w| {
            if r.enable().bit_is_set() {
                w.enable().clear_bit()
            } else {
                w
            }
        });

        while self.tcc.syncbusy.read().enable().bit_is_set() {}
    }

    /// Disable the timer and issue a software reset. This clears out all
    /// configuration and resets them to the default.
    pub fn reset(&mut self) {
        self.disable();
        self.tcc.ctrla.modify(|_, w| w.swrst().set_bit());

        while self.tcc.syncbusy.read().swrst().bit_is_set() {}
    }


    /// Reset and reconfigure the timer.
    pub fn reconfigure(&mut self, config: &TimerConfig<TCC>) {
        self.reset();

        config.configure(&self.tcc);
    }

    /// Stop the timer counting.
    pub fn stop(&mut self) {
        self.tcc.ctrlbset.write(|w| {
            w.cmd().stop()
        });

        while self.tcc.ctrlbset.read().cmd() != CMD_A::NONE {}
    }

    /// Restart the timer count.
    pub fn retrigger(&mut self) {
        self.tcc.ctrlbset.write(|w| w.cmd().retrigger());

        while self.tcc.ctrlbset.read().cmd().is_retrigger() {}
    }

    /// Returns whether the timer is stopped or not.
    pub fn is_stopped(&self) -> bool {
        self.tcc.status.read().stop().bit_is_set()
    }

    /// Get the enabled interrupts for the timer instance.
    pub fn enabled_interrupts(&self) -> Interrupts {
        Interrupts::from_bits(self.tcc.intenset.read().bits()).unwrap()
    }

    /// Enable or disable the timer interrupts specified by the given bitfield.
    pub fn enable_interrupts(&mut self, value: Interrupts) {
        self.tcc.intenset.write(|w| unsafe {
            w.bits(value.bits())
        });
        self.tcc.intenclr.write(|w| unsafe {
            w.bits((!value).bits())
        })
    }

    /// Get the timer instance's interrupt flags.
    pub fn interrupt_status(&self) -> Interrupts {
        Interrupts::from_bits(self.tcc.intflag.read().bits()).unwrap()
    }

    /// Clear the timer instance's interrupt flags as specified by the bitfield.
    pub fn clear_interrupts(&mut self, value: Interrupts) {
        self.tcc.intflag.write(|r| unsafe {
            r.bits(value.bits())
        });
    }

    /// Enable double buffering for period, pattern, and compare values.
    /// 
    /// Values in the buffers for each of the listed configurations will be copied to the timer
    /// when hardware update conditions occur.
    pub fn enable_double_buffering(&mut self) {
        self.tcc.ctrlbclr.write(|r| r.lupd().set_bit());

        while self.tcc.syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Disable double buffering for period, pattern, and compare values.
    /// 
    /// Values in the buffers for each of the listed configurations will NOT be copied to the timer
    /// when hardware update conditions occur.
    pub fn disable_double_buffering(&mut self) {
        self.tcc.ctrlbset.write(|r| r.lupd().set_bit());

        while self.tcc.syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Force the values stored in buffer registers to be written into their counterpart registers.
    pub fn force_buffer_update(&mut self) {
        self.tcc.ctrlbset.write(|w| {
            w.cmd().update()
        });
        while self.tcc.ctrlbset.read().cmd() != CMD_A::NONE {}
    }

    /// Set whether or not the timer is a oneshot timer.
    pub fn set_oneshot(&mut self, oneshot: bool) {
        if oneshot {
            self.tcc.ctrlbset.write(|w| w.oneshot().set_bit())
        } else {
            self.tcc.ctrlbclr.write(|w| w.oneshot().set_bit())
        }
    }

    /// Get the current value of the counter.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// 
    /// # Synchronisation
    /// 
    /// This operation requires synchronisation to read the value, which will block until complete.
    pub fn get_count(&self) -> u32 {
        self.tcc.ctrlbset.write(|w| {
            w.cmd().readsync()
        });

        while self.tcc.syncbusy.read().count().bit_is_set() {}

        tcc_reg_res!(self.tcc, count, read, count, bits)
    }

    /// Set the value of the timer counter.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    pub fn set_count(&mut self, value: u32) {
        tcc_reg_res!(self.tcc, count, modify, |_, w| unsafe { w.count().bits(value) });

        while self.tcc.syncbusy.read().count().bit_is_set() {}
    }

    /// Set the wave generation mode of the timer.
    pub fn set_wave_gen(&mut self, value: WaveformGen) {
        self.tcc.wave.modify(|_, w| unsafe {
            w.wavegen().bits(value as u8)
        });

        while self.tcc.syncbusy.read().wave().bit_is_set() {}
    }

    /// Set the channel polarity using the given channel bitfield.
    pub fn set_channel_polarity(&mut self, polarity: Channels) {
        self.tcc.wave.modify(|r, w| unsafe {
            w.bits((r.bits() & 0xffe0ffff) + ((polarity.bits() as u32) << 16))
        });

        while self.tcc.syncbusy.read().wave().bit_is_set() {}
    }

    /// Set the direction of counting for the timer.
    pub fn set_direction(&mut self, value: Direction) {
        match value {
            Direction::Up => self.tcc.ctrlbclr.modify(|_, w| w.dir().set_bit()),
            Direction::Down => self.tcc.ctrlbset.modify(|_, w| w.dir().set_bit())
        }

        while self.tcc.syncbusy.read().ctrlb().bit_is_set() {}
    }

    /// Reverse the counting direction of the timer.
    pub fn toggle_direction(&mut self) {
        self.set_direction(self.get_direction().reverse());
    }

    /// Get the counting direction of the timer.
    pub fn get_direction(&self) -> Direction {
        if self.tcc.ctrlbset.read().dir().bit_is_set() {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    /// Set the capture/compare register value for a given channel.
    /// 
    /// Depending on whether the TCC is in 24-bit or 16-bit mode, the value
    /// will be truncated to the correct length.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// Use `set_cc_dither` to set the dither value.
    pub fn set_cc(&mut self, index: usize, value: u32) {
        tcc_reg_res!(self.tcc, cc, index, modify, |_, w| unsafe { w.cc().bits(value) });

        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask {}
    }

    /// Get the capture/compare register value for a given channel.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value after reading from the register.
    /// Use `get_cc_dither` to get the dither value.
    pub fn get_cc(&self, index: usize) -> u32 {
        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask {}

        tcc_reg_res!(self.tcc, cc, index, read, cc, bits)
    }

    /// Set the capture/compare register dither for a given channel.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// Use `set_cc` to set the CC value.
    pub fn set_cc_dither(&mut self, index: usize, value: u8) {
        tcc_reg_res!(dither, self.tcc, cc, index, modify, |_, w| unsafe { w.dither().bits(value) });

        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask {}
    }

    /// Get the capture/compare register value for a given channel.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value after reading from the register.
    /// Use `get_cc` to get the CC value.
    pub fn get_cc_dither(&self, index: usize) -> u8 {
        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask {}

        tcc_reg_res!(dither, self.tcc, cc, index, read, dither, bits)
    }

    /// Set the capture/compare buffer register value for a given channel.
    /// 
    /// Depending on whether the TCC is in 24-bit or 16-bit mode, the value
    /// will be truncated to the correct length.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// Use `set_cc_buffer_dither` to set the dither value.
    pub fn set_cc_buffer(&mut self, index: usize, value: u32) {
        tcc_reg_res!(self.tcc, ccbuf, index, modify, |_, w| unsafe { w.ccbuf().bits(value) });
        
        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask {}
    }

    /// Get the capture/compare register buffer value for a given channel.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value after reading from the register.
    /// Use `get_cc_buffer_dither` to get the dither value.
    pub fn get_cc_buffer(&self, index: usize) -> u32 {
        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask { }

        tcc_reg_res!(self.tcc, ccbuf, index, read, ccbuf, bits)
    }

    /// Set the capture/compare buffer register dither for a given channel.
    /// 
    /// Depending on whether the TCC is in 24-bit or 16-bit mode, the value
    /// will be truncated to the correct length.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// Use `set_cc_buffer_dither` to set the dither value.
    pub fn set_cc_buffer_dither(&mut self, index: usize, value: u8) {
        tcc_reg_res!(dither, self.tcc, ccbuf, index, modify, |_, w| unsafe { w.ditherbuf().bits(value) });
        
        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask {}
    }

    /// Get the capture/compare register buffer dither for a given channel.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value after reading from the register.
    /// Use `get_cc_buffer_dither` to get the dither value.
    pub fn get_cc_buffer_dither(&self, index: usize) -> u8 {
        let index_bitmask = 0x1 << (index + 8);
        while self.tcc.syncbusy.read().bits() & index_bitmask == index_bitmask { }

        tcc_reg_res!(dither, self.tcc, ccbuf, index, read, ditherbuf, bits)
    }

    /// Set the period value of the timer.
    /// 
    /// Depending on whether the TCC is in 24-bit or 16-bit mode, the value
    /// will be truncated to the correct length.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// Use `set_period_dither` to set the dither value.
    pub fn set_period(&mut self, value: u32) {
        tcc_reg_res!(self.tcc, per, modify, |_, w| unsafe { w.per().bits(value) });

        while self.tcc.syncbusy.read().per().bit_is_set() { }
    }

    /// Get the period value of the timer.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value after reading from the register.
    /// Use `get_period_dither` to get the dither value.
    pub fn get_period(&self) -> u32 {
        tcc_reg_res!(self.tcc, per, read, per, bits)
    }

    /// Set the period buffer value of the timer.
    /// 
    /// Depending on whether the TCC is in 24-bit or 16-bit mode, the value
    /// will be truncated to the correct length.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// Use `set_period_buffer_dither` to set the dither value.
    pub fn set_period_buffer(&mut self, value: u32) {
        tcc_reg_res!(self.tcc, perbuf, modify, |_, w| unsafe { w.perbuf().bits(value) });
        
        while self.tcc.syncbusy.read().per().bit_is_set() {}
    }

    /// Get the period buffer value of the timer.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value after reading from the register.
    /// Use `get_period_buffer_dither` to get the dither value.
    pub fn get_period_buffer(&self) -> u32 {
        tcc_reg_res!(self.tcc, perbuf, read, perbuf, bits)
    }

    /// Set the period buffer dither value of the timer.
    /// 
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value before being written to the register.
    /// Use `set_period_buffer` to set the period buffer value.
    pub fn set_period_buffer_dither(&mut self, value: u8) {
        tcc_reg_res!(dither, self.tcc, perbuf, modify, |_, w| unsafe { w.ditherbuf().bits(value) });
    }

    /// Get the period buffer dither value of the timer.
    /// 
    /// # Dithering
    /// 
    /// Depending on the configured dithering behaviour, the appropriate shifts
    /// will be performed on the value after reading from the register.
    /// Use `get_period_buffer` to get the period value.
    pub fn get_period_buffer_dither(&self) -> u8 {
        tcc_reg_res!(dither, self.tcc, perbuf, read, ditherbuf, bits)
    }

    /// Set the pattern value of each waveform output as specified by the
    /// provided bitfield.
    pub fn set_pattern_value(&mut self, value: WaveformSelect) {
        self.tcc.patt.modify(|r, w| unsafe {
            w.bits(((value.bits() as u16) << 8) + (r.bits() & 0xFF))
        });

        while self.tcc.syncbusy.read().patt().bit_is_set() {}
    }

    /// Get the pattern value of each waveform output as a bitfield.
    pub fn get_pattern_value(&self) -> WaveformSelect {
        while self.tcc.syncbusy.read().patt().bit_is_set() {}

        WaveformSelect::from_bits_truncate((self.tcc.patt.read().bits() >> 8).try_into().unwrap())
    }

    /// Set the enable pattern for each waveform output using the
    /// provided bitfield.
    pub fn set_pattern_enable(&mut self, enable: WaveformSelect) {
        self.tcc.patt.modify(|r, w| unsafe {
            w.bits((r.bits() & 0xFF00) + (enable.bits() as u16))
        });
        
        while self.tcc.syncbusy.read().patt().bit_is_set() {}
    }

    /// Get the enable pattern for each waveform output as a bitfield.
    pub fn get_pattern_enable(&self) -> WaveformSelect {
        while self.tcc.syncbusy.read().patt().bit_is_set() {}

        WaveformSelect::from_bits_truncate((self.tcc.patt.read().bits() & 0x00FF).try_into().unwrap())
    }

    /// Set the pattern value buffer of each waveform output as specified by
    /// the provided bitfield.
    pub fn set_pattern_value_buffer(&mut self, value: WaveformSelect) {
        self.tcc.pattbuf.modify(|r, w| unsafe {
            w.bits(((value.bits() as u16) << 8) + (r.bits() & 0xFF))
        });

        while self.tcc.syncbusy.read().patt().bit_is_set() {}
    }

    /// Get the pattern value buffer of each waveform output as a bitfield.
    pub fn get_pattern_value_buffer(&self) -> WaveformSelect {
        while self.tcc.syncbusy.read().patt().bit_is_set() {}

        WaveformSelect::from_bits_truncate((self.tcc.pattbuf.read().bits() >> 8).try_into().unwrap())
    }

    /// Set the pattern enable buffer of each waveform output as specified by
    /// the provided bitfield.
    pub fn set_pattern_enable_buffer(&mut self, enable: WaveformSelect) {
        self.tcc.pattbuf.modify(|r, w| unsafe {
            w.bits((r.bits() & 0xFF00) + (enable.bits() as u16))
        });

        while self.tcc.syncbusy.read().patt().bit_is_set() {}
    }

    /// Get the pattern enable buffer of each waveform output as a bitfield.
    pub fn get_pattern_enable_buffer(&self) -> WaveformSelect {
        while self.tcc.syncbusy.read().patt().bit_is_set() {}

        WaveformSelect::from_bits_truncate((self.tcc.pattbuf.read().bits() & 0x00FF).try_into().unwrap())
    }
}