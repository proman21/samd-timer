/// Counting directions for timers.
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn reverse(self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
        }
    }
}

/// Dither widths for PWM dithering.
#[derive(SmartDefault, Clone, Copy)]
pub enum Dither {
    #[default]
    None = 0,
    Dith4,
    Dith5,
    Dith6,
}

/// Set of clock prescalers.
#[derive(SmartDefault, Clone, Copy)]
pub enum Prescaler {
    #[default]
    Div1 = 0,
    Div2,
    Div4,
    Div8,
    Div16,
    Div64,
    Div256,
    Div1024,
}

/// Prescaler and counter synchronisation options.
#[derive(SmartDefault, Clone, Copy)]
pub enum Synchronization {
    #[default]
    Clock = 0,
    Prescaler,
    Resync,
}

/// Ramp operation modes.
#[derive(SmartDefault)]
pub enum Ramp {
    #[default]
    Ramp1 = 0,
    Ramp2Alternative,
    Ramp2,
    Ramp2Critical,
}

/// Capture modes for Timer.
#[derive(SmartDefault, Clone, Copy)]
pub enum CaptureMode {
    #[default]
    Default = 0,
    MinCapture,
    MaxCapture
}

/// Capture triggers for Timer.
#[derive(SmartDefault)]
pub enum CaptureTrigger {
    #[default]
    Event = 0,
    Pin
}

impl CaptureTrigger {
    pub(crate) fn to_bit(&self) -> bool {
        match self {
            CaptureTrigger::Event => false,
            CaptureTrigger::Pin => true
        }
    }
}

bitflags! {
    /// Bitfield for channel polarity.
    #[derive(Default)]
    pub struct Channels: u8 {
        const CHAN_0 = 0x1;
        const CHAN_1 = 0x2;
        const CHAN_2 = 0x4;
        const CHAN_3 = 0x8;
        const CHAN_4 = 0x10;
        const CHAN_5 = 0x20;
    }
}

bitflags! {
    #[derive(Default)]
    /// Dead-time insertion generator bitfield.
    pub struct DTIEnable: u8 {
        const GEN_0 = 0x1;
        const GEN_1 = 0x2;
        const GEN_2 = 0x4;
        const GEN_3 = 0x8;
    }
}

bitflags! {
    /// Bitfield used to select waveform outputs.
    #[derive(Default)]
    pub struct WaveformSelect: u8 {
        const WO_0 = 0x1;
        const WO_1 = 0x2;
        const WO_2 = 0x4;
        const WO_3 = 0x8;
        const WO_4 = 0x10;
        const WO_5 = 0x20;
        const WO_6 = 0x40;
        const WO_7 = 0x80;
    }
}

/// Output matrix configuration options.
#[derive(SmartDefault, Clone, Copy)]
pub enum OutputMatrixConfig {
    #[default]
    Default = 0,
    Modulo,
    AllZero,
    OneZeroRestOne,
}

/// Bit-width modes for `Timer`.
#[derive(SmartDefault)]
pub enum TimerMode {
    #[default]
    Count16 = 0,
    Count8,
    Count32,
}

#[derive(Clone, Copy, SmartDefault)]
/// Wave generation modes for `Timer`.
pub enum TimerWaveGen {
    /// Normal Frequency PWM (Default)
    #[default]
    NFRQ = 0,
    /// Match Frequency PWM
    MFRQ,
    /// Normal PWM
    NPWM,
    /// Match PWM
    MPWM,
}