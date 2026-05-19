# hello-world — NDIR Firmware for Teensy 4.1

Rust `no_std` firmware for the **SGX IR1 / IR12EM NDIR gas sensor** running on a **Teensy 4.1** (NXP i.MX RT1062, ARM Cortex-M7). Uses [RTIC v2](https://rtic.rs) for real-time task scheduling and [teensy4-bsp](https://github.com/mciantyre/teensy4-rs) for hardware access.

---

## Directory Structure

```
hello-world/
├── .cargo/
│   └── config.toml   ← build target + linker flags
├── Cargo.toml        ← dependencies and build profiles
├── firmware.hex      ← last compiled binary (ready to flash)
└── src/
    ├── main.rs       ← RTIC app: init, sensor_loop task, USB log task
    ├── filter.rs     ← Median5, EMA, ChannelFilter, ProcessedFilter
    └── sensor.rs     ← NdirState machine (pure math, no HAL)
```

---

## Prerequisites

### 1. Rust toolchain

```powershell
# Install stable + the ARM Cortex-M7 target
rustup target add thumbv7em-none-eabihf
```

### 2. `cargo-binutils` (for .hex conversion)

```powershell
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### 3. Teensy Loader
Download **Teensy Loader CLI** or the GUI loader from https://www.pjrc.com/teensy/loader.html  
The GUI loader (`teensy_loader.exe`) is the easiest option on Windows.

---

## Building

```powershell
# Debug build (larger, includes debug info)
cargo build

# Release build (recommended for deployment — optimised)
cargo build --release
```

### Convert ELF → HEX (required for Teensy Loader)

```powershell
cargo objcopy --release -- -O ihex firmware.hex
```

The resulting `firmware.hex` is placed in the project root.

---

## Flashing to Teensy 4.1

1. Open **Teensy Loader GUI**.
2. Click **File → Open HEX File** → select `firmware.hex`.
3. Press the **physical button** on the Teensy board to enter bootloader mode.
4. Teensy Loader will detect the board and flash automatically.

> **Tip:** Holding the button for >5 s enters the bootloader without needing a firmware already running.

---

## Reading Serial Output

The firmware logs over **USB CDC** (virtual COM port).

```powershell
# Replace COM3 with your actual port (check Device Manager)
# Any serial terminal works — example with PuTTY or screen:
python -m serial.tools.miniterm COM3 115200
```

Log format:
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

Set any of these to `false` to bypass that stage. The log format stays identical — disabled stages pass the previous value through unchanged.

### Bench-testing without a lamp

```rust
const AMBIENT_TEST_MODE: bool = true;
```

The lamp pin is held LOW. The firmware still cycles ON/OFF timing so you can verify serial output and filter logic on a bench without connecting hardware.

### Enabling only one log type

```rust
const LOG_RAW:  bool = true;   // RAW,... line every half-period tick
const LOG_PROC: bool = false;  // suppress PROC,... lines
```

### Tuning EMA smoothing

In `src/filter.rs`:

```rust
// Higher α = faster response, more noise.
// Lower  α = slower response, smoother.
pub const RAW_EMA_ALPHA:  f32 = 0.25;  // per ADC channel
pub const PROC_EMA_ALPHA: f32 = 0.20;  // ratio / response / absorbance
```

### Tuning the ref_diff guard

```rust
// Pairs with ref_diff below this are silently dropped.
// Increase if you see noisy spikes; decrease for weak lamp drive.
pub const REF_DIFF_MIN: u16 = 40;
```

### Rewiring pins

| Signal         | Default pin | Constant / type alias      |
|----------------|-------------|----------------------------|
| IR lamp gate   | D2          | `IrDrivePin` + `gpio4 / pins.p2` in `init` |
| LED heartbeat  | D13         | `board::led(&mut gpio2, pins.p13)` |
| ACT photodiode | A0 (pin 14) | `ActPin = P14`, `pins.p14` |
| REF photodiode | A1 (pin 15) | `RefPin = P15`, `pins.p15` |
| TEMP sensor    | A2 (pin 16) | `TempPin = P16`, `pins.p16` |

Change both the **type alias** at the top of `main.rs` and the **`pins.pN`** call inside `init`.

### Switching to 12-bit ADC

Uncomment in `init`:
```rust
adc1.set_resolution(adc::ResolutionBits::Bits12);
adc1.calibrate();
```
Then update `filter.rs`:
```rust
pub const RAIL_HIGH: u16 = 4080;  // was 1015 for 10-bit
pub const REF_DIFF_MIN: u16 = 160; // ≈ 4% of 4095 full-scale
```

---

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `teensy4-bsp` | Board support: pins, clock, ADC, LED |
| `rtic` v2 | Real-time interrupt-driven concurrency framework |
| `rtic-monotonics` | `Systick` timer for `async` delays |
| `imxrt-log` | USB CDC logging backend |
| `log` | `log::info!` / `log::warn!` macros |
| `teensy4-panic` | Panic handler that logs the panic message over USB |
