# SAMD Timer

> High-level API for the TCC/TC peripherals found on the Microchip SAMD and SAME MCUs.

[![Crates.io](https://img.shields.io/crates/v/samd-timer)](https://crates.io/crates/samd-timer)
![Crates.io](https://img.shields.io/crates/l/samd-timer)
![Crates.io](https://img.shields.io/crates/d/samd-timer)

**This library is usable (as in I use it), but could contain bugs. USE AT YOUR OWN RISK.**

This library provides the `Timer` and `ControlTimer` types for working safely with `TC` and `TCC` peripherals respectively. Full support for waveform generation capabilities, double buffering, and capture modes.

Provides finer grain control over your timers than the abstraction used by [`atsamd-hal`](https://github.com/atsamd-rs/atsamd/). Simply drop in the library; it works with the `atsamd-hal` clock system.

## Usage

Add the following line to your `Cargo.toml`.

```toml
samd-timer = "0.2.0"
```

The following feature flags control which MCU variant you are targetting.

| Name | # TC | # TCC | Boards |
|:------|:----|:----|:-----------------------|
| [samd21g18a](https://docs.rs/atsamd21g18a/) | 3 | 3 | Circuit Playground Express, Feather M0, Metro M0, MKR ZERO, SAMD21 Mini, SODAQ ONE |
| [samd21e18a](https://docs.rs/atsamd21e18a/) | 3 | 3 | Gemma M0, Trinket M0, Serpente |
| [samd21j18a](https://docs.rs/atsamd21j18a/) | 5 | 3 | SODAQ SARA AFF |
| [samd51j19a](https://docs.rs/atsamd51j19a/) | 6 | 5 | EdgeBadge, Feather M4, Metro M4 |
| [samd51j20a](https://docs.rs/atsamd51j20a/) | 6 | 5 | PyPortal |
| [samd51g19a](https://docs.rs/atsamd51g19a/) | 4 | 3 | ItsyBitsy M4, Trellis M4 |
| [same54p20a](https://docs.rs/atsame54p20a/) | 8 | 5 | PathfinderZA Proto1 |

## Example

```rust
use atsamd_hal::target_device::Peripherals;
use atsamd_hal::clock::GenericClockController;
use samd_timer::{TimerConfig, Timer, Prescaler, Synchronization, TimerWaveGen};

fn main() {
    let mut peri = Peripherals.take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peri.GCLCK,
        &mut peri.MCLK,
        &mut peri.OSC32KCTRL,
        &mut peri.OSCCTRL,
        &mut peri.NVMCTRL
    );

    let timer_clock = clocks.gclk1();
    let tc_clock = clocks.tc0_tc1(&timer_clock).unwrap();

    /// Create a 8-bit width timer configuration.
    let mut config = TimerConfig::count8();
    config.prescaler(Prescaler::Div64)
        .sync(Synchronization::Prescaler)
        .wave_gen(TimerWaveGen::NPWM);

    /// Initialise the timer instance.
    let mut timer = config.tc0(&mut peri.MCLK, &tc_clock, peri.TC0);

    // Output a PWM wave with a period of 2 secs and duty cycle of 50%
    timer.set_period(999);
    timer.set_cc0(499);
    timer.enable();
}
```

## Features

### TCC

- [x] One-Shot Operation
- [x] Interrupt Control
- [x] Direction changing
- [x] Waveform Generation
- [x] Waveform Output Inversion
- [x] Dithering
- [x] Dead-time Insertion Generator
- [x] Output Matrixing
- [x] Pattern Generation
- [ ] Fault Control
- [ ] Event Control
- [ ] Master-Slave Operation
- [x] Double Buffering

### TC

- [x] 8-bit mode (incl. PER register)
- [x] 16-bit mode
- [ ] 32-bit mode
- [x] Interrupt Control
- [x] One-Shot Operation
- [x] Direction changing
- [x] Waveform Generation
- [x] Waveform Output Inversion
- [x] Capture Channels
- [ ] Event Control
- [x] Double Buffering (SAMx5x)
- [x] Capture Modes (SAMx5x)
- [x] Capture on Pin (SAMx5x)

## License

`samd-timer` is distributed under the MIT license. See the LICENSE file for the full text of the license.