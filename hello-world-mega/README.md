# hello-world-mega — NDIR Firmware for Arduino Mega (ATmega2560)

Rust `no_std` firmware for the **SGX IR1 / IR12EM NDIR gas sensor** running on an **Arduino Mega 2560** (ATmega2560, AVR 8-bit). Direct port of [`hello-world`](../hello-world) (Teensy 4.1 / RTIC) to the [`arduino-hal`](https://github.com/Rahix/avr-hal) platform. The signal chain, filter logic, and CSV log format are **identical** to the Teensy version.

> **Important:** AVR Rust requires **Rust Nightly** and the AVR LLVM backend. See prerequisites below.

---

## Directory Structure

```
hello-world-mega/
├── .cargo/
│   └── config.toml   ← build target (avr-atmega2560) + opt flags
├── Cargo.toml        ← arduino-hal, panic-halt, ufmt
└── src/
    ├── main.rs       ← entry point: ADC reads, filter, state machine, UART log
    ├── filter.rs     ← Median5, EMA, ChannelFilter, ProcessedFilter (shared with Teensy)
    └── sensor.rs     ← NdirState machine (shared with Teensy, zero changes)
```

---

## Prerequisites

### 1. Rust Nightly + required components

```powershell
# Install nightly toolchain
rustup install nightly

# AVR LLVM backend
rustup component add llvm-tools-preview --toolchain nightly

# Standard library source (required for -Z build-std)
rustup component add rust-src --toolchain nightly

# cargo-binutils for .hex conversion
cargo install cargo-binutils
```

### 2. AVR-GCC (linker — required)

The AVR linker (`avr-gcc`) is **not** bundled with Rust and must be installed separately.

**Option A — WinAVR (simplest on Windows):**
Download and install from https://winavr.sourceforge.net — adds `avr-gcc` to PATH automatically.

**Option B — Arduino IDE (already installed on this machine):**
`avr-gcc` was found at:
```powershell
# Add to PATH before building (or add permanently to your system PATH):
$env:PATH += ";C:\Users\Karma\AppData\Local\Arduino15\packages\arduino\tools\avr-gcc\7.3.0-atmel3.6.1-arduino7\bin"
```

Verify it works:
```powershell
avr-gcc --version   # should print avr-gcc 7.x or similar
```

### 3. ravedude (for flashing via `cargo run`)

```powershell
cargo install ravedude
```

---

## Building

The AVR target uses a **custom JSON spec** (`avr-specs/avr-atmega2560.json`), which requires the
`-Zjson-target-spec` flag. The `build-std = ["core"]` is declared in `.cargo/config.toml` so it
applies automatically.

```powershell
# Release build (recommended — LTO + single codegen unit)
cargo +nightly build -Zjson-target-spec --release
```

> `--release` is strongly recommended on AVR — dev builds can be 2× larger and may not fit in flash.

### Convert ELF → HEX (if not using ravedude)

```powershell
cargo +nightly objcopy -Zjson-target-spec --release -- -O ihex firmware.hex
```

---

## Flashing to Arduino Mega

### Option A — ravedude (recommended)

1. Uncomment the `runner` line in `.cargo/config.toml`:
   ```toml
   runner = "ravedude mega2560 -cb 115200"
   ```
2. Plug in the Mega via USB, then:
   ```powershell
   cargo +nightly run -Z build-std=core --release
   ```
   `ravedude` will detect the COM port, build, and flash in one step.

### Option B — avrdude manually

```powershell
# Replace COM3 with your actual port
avrdude -p atmega2560 -c wiring -P COM3 -b 115200 -D `
  -U flash:w:target\avr-atmega2560\release\hello-world-mega.elf:e
```

### Option C — Arduino IDE

1. Build the `.hex` as shown above.
2. Open Arduino IDE → **Sketch → Upload Using Programmer** → select the `.hex`.

---

## Reading Serial Output

The firmware uses **UART0** (pins D0 RX / D1 TX) which the Mega's on-board CH340/FT232 bridges to USB. Connect at **115 200 baud**:

```powershell
# Python miniterm (pip install pyserial)
python -m serial.tools.miniterm COM3 115200

# Or use Arduino IDE Serial Monitor at 115200 baud
```

Log format (identical to Teensy version):
```
RAW,<s.ms>,<ph>,<act_r>,<ref_r>,<tmp_r>,<act_m>,<ref_m>,<tmp_m>,<act_f>,<ref_f>,<tmp_f>,<rail>
PROC,<s.ms>,<bl>,<adiff>,<rdiff>,<ratio_r>,<ratio_f>,<base>,<rsp_r>,<rsp_f>,<abs_r>,<abs_f>,<tavg>
```

---

## Code Manipulation Guide

### Changing the lamp frequency

In `src/main.rs`:

```rust
// 4 Hz lamp, 50% duty → 125 ms half-period.
const HALF_PERIOD_MS: u32 = 125;
```

| Target freq | Value |
|-------------|-------|
| 2 Hz        | `250` |
| 4 Hz        | `125` |
| 8 Hz        | `62`  |

> On AVR, `delay_ms` takes a `u16`. The cast `HALF_PERIOD_MS as u16` is safe up to 65 535 ms.

### Changing the baseline duration

```rust
// 80 pairs × 250 ms ≈ 20 s baseline window
const BASELINE_CYCLES: u32 = 80;
```

### Enabling / disabling filter stages

```rust
const USE_MEDIAN_FILTER: bool = true;  // Median-of-5 denoising
const USE_RAW_EMA:       bool = true;  // EMA smoothing on ADC channels
const USE_PROC_EMA:      bool = true;  // EMA smoothing on ratio/response
```

Set any to `false` to bypass that stage. The log format stays identical.

### Bench-testing without a lamp

```rust
const AMBIENT_TEST_MODE: bool = true;
```

Lamp pin held LOW. The firmware still cycles timing so you can verify UART output and filter logic without connecting hardware.

### Enabling only one log type

```rust
const LOG_RAW:  bool = true;   // RAW,... line every half-period
const LOG_PROC: bool = false;  // suppress PROC,... lines
```

### Tuning EMA smoothing

In `src/filter.rs`:

```rust
pub const RAW_EMA_ALPHA:  f32 = 0.25;  // per ADC channel (ACT/REF/TEMP)
pub const PROC_EMA_ALPHA: f32 = 0.20;  // ratio / response / absorbance
```

Higher α = faster response, more noise. Lower α = smoother, slower.

### Tuning the ref_diff guard

```rust
// Drop pairs with ref_diff below this (protects against divide-by-zero).
pub const REF_DIFF_MIN: u16 = 40;  // ≈ 4% of 10-bit full-scale (1023)
```

### Rewiring pins

| Signal         | Default pin | Constant in `main.rs`             |
|----------------|-------------|-----------------------------------|
| IR lamp gate   | D2          | `let mut ir_drive = pins.d2.into_output()` |
| LED heartbeat  | D13         | `let mut led = pins.d13.into_output()`     |
| ACT photodiode | A0          | `let act_pin  = pins.a0.into_analog_input(&mut adc)` |
| REF photodiode | A1          | `let ref_pin  = pins.a1.into_analog_input(&mut adc)` |
| TEMP sensor    | A2          | `let temp_pin = pins.a2.into_analog_input(&mut adc)` |

Change both the `pins.dN` / `pins.aN` and, if moving to a different port, the `dp.*` peripheral passed to the HAL.

### Changing ADC reference voltage

```rust
// In main() — default is AVCC (5 V)
let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
```

To use the internal 1.1 V reference (for low-signal sensors):

```rust
use arduino_hal::adc::ReferenceVoltage;
let mut adc = arduino_hal::Adc::new(
    dp.ADC,
    arduino_hal::adc::AdcSettings {
        ref_voltage: ReferenceVoltage::Internal,
        ..Default::default()
    },
);
```

Also update `filter.rs` rail thresholds accordingly.

### Adding a second sensor (dual-channel)

1. Declare two more `ChannelFilter` instances in `main.rs`.
2. Read additional ADC pins inside the loop.
3. Call `ndir.capture_on` / `capture_off_and_process` with the new values.
4. The `sensor.rs` and `filter.rs` modules are stateless per-instance — just create additional objects.

---

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `arduino-hal` | Board support for ATmega2560: pins, ADC, UART, delay |
| `panic-halt`  | Panic handler — halts CPU (safe for production AVR) |
| `ufmt`        | Lightweight `no_std` serial formatting (replaces `core::fmt`) |

> `filter.rs` and `sensor.rs` have **zero external dependencies** — they are pure Rust math and are shared verbatim with the Teensy version.

---

## Differences vs. Teensy (`hello-world`)

| Aspect | Teensy 4.1 | Arduino Mega |
|--------|-----------|-------------|
| CPU | ARM Cortex-M7 @ 600 MHz | AVR ATmega2560 @ 16 MHz |
| Rust target | `thumbv7em-none-eabihf` | `avr-atmega2560` |
| Scheduler | RTIC v2 async tasks | Blocking `delay_ms` loop |
| Serial | USB CDC via `imxrt-log` | UART0 via `ufmt` |
| Float formatting | `log::info!("{:.4}", …)` | `write_f32()` helper (ufmt has no f32) |
| Build command | `cargo build --release` | `cargo +nightly build -Z build-std=core --release` |
| Flash tool | Teensy Loader | ravedude / avrdude |
| `filter.rs` | ✅ shared | ✅ shared (unchanged) |
| `sensor.rs` | ✅ shared | ✅ shared (unchanged) |
