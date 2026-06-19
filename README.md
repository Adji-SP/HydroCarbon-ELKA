# HydroCarbon-ELKA

Embedded firmware for an **NDIR (Non-Dispersive Infrared) hydrocarbon gas sensor** running on a **Teensy 4.1** microcontroller.  
Written in Rust using the RTIC v2 async framework. Zero heap allocations — fully `no_std`.

---

## Table of Contents

1. [What is NDIR?](#what-is-ndir)
2. [Hardware](#hardware)
3. [Project Structure](#project-structure)
4. [Signal Chain](#signal-chain)
5. [Firmware Architecture](#firmware-architecture)
6. [Configuration Reference](#configuration-reference)
7. [Log Format](#log-format)
8. [Building & Flashing — Teensy 4.1](#building--flashing)
9. [Flashing the Mega (hello-world-mega)](#flashing-the-mega-hello-world-mega)
10. [Data Files](#data-files)

---

## What is NDIR?

An NDIR sensor works by shining an **infrared lamp** at a gas sample and measuring how much IR light reaches two photodetectors:

| Channel | Purpose |
|---------|---------|
| **ACT** (active) | Measures at the wavelength the target gas absorbs |
| **REF** (reference) | Measures at a wavelength the gas does *not* absorb — tracks lamp drift |
| **TEMP** | Thermistor / temperature channel for compensation |

By **toggling the lamp ON and OFF** and computing the difference in each channel, the firmware cancels out ambient light and DC offset, leaving only the IR signal caused by the lamp.  
The **ratio `ACT_diff / REF_diff`** normalises out lamp intensity drift over time, giving a stable, lamp-independent gas measurement.

---

## Hardware

| Item | Detail |
|------|--------|
| MCU | Teensy 4.1 (i.MX RT1062, Cortex-M7 @ 600 MHz) |
| Sensor | SGX IR1 / IR12EM (or compatible NDIR module) |
| ADC | Internal 10-bit ADC1 (configurable to 12-bit) |
| IR lamp drive | MOSFET gate → **Teensy pin 2** (GPIO4 / GPIO_EMC_04) |
| Active channel | **Teensy pin 14** (A0, ADC1) |
| Reference channel | **Teensy pin 15** (A1, ADC1) |
| Temperature channel | **Teensy pin 16** (A2, ADC1) |
| USB | Full-speed USB CDC — used for serial log output |
| LED | Onboard LED (pin 13) — heartbeat blink per ON/OFF pair |

> **Re-wiring:** All pin assignments are `type` aliases at the top of `main.rs`. Change `P2`/`P14`/`P15`/`P16` and the matching `gpio4`/`pins.pN` calls in `init` if you route differently.

---

## Project Structure

```
hello-world/
├── .cargo/
│   └── config.toml        # Build target: thumbv7em-none-eabihf + linker flags
├── src/
│   ├── main.rs            # RTIC app — init, sensor_loop task, all config consts
│   ├── filter.rs          # Signal filtering: Median-of-5, EMA, rail detection
│   └── sensor.rs          # NDIR state machine — baseline accumulation, ratio math
├── Cargo.toml             # Dependencies
└── firmware.hex           # Pre-built flash image (Intel HEX)
```

---

## Signal Chain

Each ADC channel goes through up to three stages before being used in gas math.  
Every stage can be independently **enabled or disabled** via `const bool` flags in `main.rs`.

```
ADC raw (u16)
    │
    ▼  [USE_MEDIAN_FILTER]
 Median-of-5 sliding window
    │  Removes short impulse spikes (EMI, lamp transients).
    │  Returns the median of the last 5 samples.
    │
    ▼  [USE_RAW_EMA]
 Exponential Moving Average (EMA)
    │  y[n] = α·x[n] + (1-α)·y[n-1]
    │  Smooths residual noise. Alpha tuned via RAW_EMA_ALPHA.
    │
    ▼
 act_f / ref_f / temp_f  (filtered u16, fed to sensor math)
    │
    ▼  [USE_FILTERED_FOR_PROC]
 NdirState (sensor.rs)
    │  capture_on(act, ref, temp)      — stores ON-phase sample
    │  capture_off_and_process(...)    — computes ProcessedSample
    │
    │  ProcessedSample:
    │    act_diff    = |ACT_on  − ACT_off|
    │    ref_diff    = |REF_on  − REF_off|
    │    ratio       = act_diff / ref_diff
    │    response%   = (ratio − baseline) / baseline × 100
    │
    ▼  [USE_PROC_EMA]
 ProcessedFilter (EMA on processed outputs)
    │  ratio_filt, response_filt, absorbance_filt
    │  Alpha tuned via PROC_EMA_ALPHA.
    │
    ▼
 PROC log line → USB serial
```

### ref_diff guard

If `ref_diff < REF_DIFF_MIN` the whole pair is **silently dropped** (`None` returned).  
This protects against:
- Division by near-zero when the lamp barely toggles
- Noisy ratio spikes during lamp warm-up or hardware faults

Tune `REF_DIFF_MIN` in `filter.rs`.

### Baseline accumulation

For the first `BASELINE_CYCLES` ON/OFF pairs, `NdirState` **accumulates a running mean of `ratio`**.  
Once enough pairs are collected the mean is locked as `baseline_ratio`.  
`response_pct` and absorbance are **only meaningful after the baseline is locked** (`baseline_ready = true`).  
Default: `BASELINE_CYCLES = 80` pairs × 250 ms = **~20 seconds warm-up**.

### Beer-Lambert absorbance

```
A = -ln(ratio / baseline_ratio)
```

Implemented as a fast Padé-1 `ln` approximation (no `libm` dependency).  
Accurate to ~1% for `0.5 < x < 2.0` — the normal range for NDIR ratios.  
Replace with `libm::logf(x)` for full float precision if needed.

---

## Firmware Architecture

The firmware uses **RTIC v2** (Real-Time Interrupt-driven Concurrency).

### Tasks

| Task | Type | Period |
|------|------|--------|
| `sensor_loop` | Async software task | Every `HALF_PERIOD_MS` (125 ms) |
| `log_over_usb` | Hardware interrupt task | On USB_OTG1 interrupt |

### `sensor_loop` — what happens every 125 ms

```
1. Wait HALF_PERIOD_MS via Systick delay
2. Read ADC (act_raw, ref_raw, temp_raw)
3. Run ChannelFilter on all three channels (Median5 → EMA)
4. Rail detection (flag if any channel is near 0 or full-scale)
5. Emit RAW log line (if LOG_RAW = true)
6. ON tick:
     → store sample via ndir.capture_on()
     → toggle lamp OFF
7. OFF tick:
     → compute ProcessedSample via ndir.capture_off_and_process()
     → run ProcessedFilter (EMA on ratio/response/absorbance)
     → emit PROC log line (if LOG_PROC = true)
     → toggle lamp ON for next pair
     → blink LED (heartbeat)
```

### Modules

#### `src/filter.rs`
All signal filtering primitives. Zero heap, `const fn` constructors.

| Type | Description |
|------|-------------|
| `Ema` | Single-channel exponential moving average |
| `Median5` | Circular buffer median-of-5 (insertion-sort on 5 elements) |
| `ChannelFilter` | Combined `Median5 → Ema` for one ADC channel |
| `ProcessedFilter` | Three `Ema` instances for ratio, response, absorbance |
| `FilteredProcessed` | Output struct of `ProcessedFilter::update()` |
| `near_rail()` | Returns `true` if any channel is ≤ `RAIL_LOW` or ≥ `RAIL_HIGH` |
| `absorbance()` | Beer-Lambert: `-ln(ratio / baseline_ratio)` |

#### `src/sensor.rs`
Pure Rust state machine. No HAL dependencies — portable and unit-testable.

| Type | Description |
|------|-------------|
| `HalfSample` | One half-cycle of ADC reads (act, ref, temp) |
| `ProcessedSample` | Output of a complete ON/OFF pair after all math |
| `NdirState` | Stateful machine: stores ON sample, accumulates baseline, computes ratio |

---

## Configuration Reference

All config lives at the top of `main.rs` as `const bool` or `const u32` — the compiler eliminates disabled branches with zero flash cost.

### Mode

| Constant | Default | Description |
|----------|---------|-------------|
| `AMBIENT_TEST_MODE` | `true` | `true` = lamp held LOW, pseudo ON/OFF cycles for bench testing. `false` = real IR lamp toggled. |

### Filter enable / disable

| Constant | Default | Description |
|----------|---------|-------------|
| `USE_MEDIAN_FILTER` | `true` | Enable Median-of-5 on raw ADC channels. `false` = raw passed directly to EMA. |
| `USE_RAW_EMA` | `true` | Enable EMA on each ADC channel. `false` = median (or raw) used as-is. |
| `USE_PROC_EMA` | `true` | Enable EMA on processed outputs (ratio, response, absorbance). `false` = `_filt` columns equal raw values. |
| `USE_FILTERED_FOR_PROC` | `true` | Feed EMA-filtered ADC into sensor math. `false` = raw ADC into sensor math. Both paths always log all columns. |

### Logging

| Constant | Default | Description |
|----------|---------|-------------|
| `LOG_RAW` | `true` | Emit `RAW,...` line every half-period tick. |
| `LOG_PROC` | `true` | Emit `PROC,...` line every OFF tick (once per ON/OFF pair). |

### Timing

| Constant | Default | Description |
|----------|---------|-------------|
| `HALF_PERIOD_MS` | `125` | Half-period of lamp toggle. 125 ms → 4 Hz lamp, 50% duty cycle. |
| `BASELINE_CYCLES` | `80` | ON/OFF pairs to average for baseline. 80 × 250 ms ≈ 20 s warm-up. |

### Filter tuning (in `filter.rs`)

| Constant | Default | Description |
|----------|---------|-------------|
| `RAW_EMA_ALPHA` | `0.25` | EMA weight for ADC channels. Range `(0, 1]`. Higher = faster/noisier. |
| `PROC_EMA_ALPHA` | `0.2` | EMA weight for processed outputs. Slightly heavier smoothing than raw. |
| `REF_DIFF_MIN` | `40` | Minimum `ref_diff` (ADC counts) for a valid pair. ≈4% full-scale on 10-bit ADC. |
| `RAIL_LOW` | `8` | ADC counts at/below which a reading is flagged as railed low. |
| `RAIL_HIGH` | `1015` | ADC counts at/above which a reading is flagged as railed high. Change to `4080` for 12-bit. |

---

## Log Format

Output is plain text over **USB CDC serial** (115200 baud or USB native speed).  
Each line is a CSV row prefixed with `RAW` or `PROC`.

### RAW line — every 125 ms

```
RAW,<time>,<ph>,<act_r>,<ref_r>,<tmp_r>,<act_m>,<ref_m>,<tmp_m>,<act_f>,<ref_f>,<tmp_f>,<rail>
```

| Field | Type | Description |
|-------|------|-------------|
| `time` | `s.ms` | Elapsed time (e.g. `1.125`) |
| `ph` | `0/1` | Lamp phase: `1` = ON, `0` = OFF |
| `act_r` | u16 | Raw ACT ADC count |
| `ref_r` | u16 | Raw REF ADC count |
| `tmp_r` | u16 | Raw TEMP ADC count |
| `act_m` | u16 | Median-of-5 ACT |
| `ref_m` | u16 | Median-of-5 REF |
| `tmp_m` | u16 | Median-of-5 TEMP |
| `act_f` | f32 (1 dp) | EMA-filtered ACT |
| `ref_f` | f32 (1 dp) | EMA-filtered REF |
| `tmp_f` | f32 (1 dp) | EMA-filtered TEMP |
| `rail` | `0/1` | `1` if any channel is near supply rail |

### PROC line — every 250 ms (OFF tick only)

```
PROC,<time>,<bl>,<adiff>,<rdiff>,<ratio_r>,<ratio_f>,<base>,<rsp_r>,<rsp_f>,<abs_r>,<abs_f>,<tavg>
```

| Field | Type | Description |
|-------|------|-------------|
| `time` | `s.ms` | Elapsed time |
| `bl` | `0/1` | `0` = still accumulating baseline, `1` = baseline locked (measurement valid) |
| `adiff` | u16 | `\|ACT_on − ACT_off\|` |
| `rdiff` | u16 | `\|REF_on − REF_off\|` |
| `ratio_r` | f32 | Raw ratio = `adiff / rdiff` |
| `ratio_f` | f32 | EMA-filtered ratio |
| `base` | f32 | Locked baseline ratio (ambient air) |
| `rsp_r` | f32 | Raw response % = `(ratio − base) / base × 100` |
| `rsp_f` | f32 | EMA-filtered response % |
| `abs_r` | f32 | Raw absorbance = `-ln(ratio/base)` |
| `abs_f` | f32 | EMA-filtered absorbance |
| `tavg` | u16 | Mean TEMP ADC count across ON and OFF half-cycles |

> **During baseline phase** (`bl=0`): `rsp_r`, `rsp_f`, `abs_r`, `abs_f` are `0.0` — not valid measurements yet.

---

## Building & Flashing

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the Cortex-M7 target
rustup target add thumbv7em-none-eabihf

# Install cargo-binutils for HEX generation
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### Build

```bash
cd hello-world
cargo build --release
```

### Generate HEX file

```bash
cargo objcopy --release -- -O ihex firmware.hex
```

### Flash to Teensy 4.1

Use the **Teensy Loader** application (from PJRC):  
1. Open `firmware.hex` in Teensy Loader  
2. Press the button on the Teensy  
3. Click **Upload**

Or use `teensy_loader_cli`:
```bash
teensy_loader_cli --mcu=TEENSY41 -w -v firmware.hex
```

### Monitor serial output

Any serial terminal at the USB CDC port works:

```bash
# Linux / macOS
screen /dev/ttyACM0 115200

# Windows — use PuTTY, Tera Term, or:
mode COM3 BAUD=115200
```

Data files `data_raw.csv` and `data_proc.csv` in the repo root contain example captures from an ambient-air bench test.

---

## Data Files

| File | Description |
|------|-------------|
| `data_raw.csv` | Example RAW log — raw, median, and EMA columns per channel |
| `data_proc.csv` | Example PROC log — ratio, response, absorbance (raw and filtered) |
| `data.txt` | Combined raw serial capture from a bench test session |
| `hello-world/firmware.hex` | Pre-built Intel HEX ready to flash (Teensy 4.1) |
| `hello-world-mega/firmware.hex` | Pre-built Intel HEX ready to flash (Arduino Mega) |

---

## Flashing the Mega (hello-world-mega)

Firmware for the **Arduino Mega 2560** (ATmega2560, AVR 8-bit). Same signal chain and CSV log format as the Teensy version.

> **Note:** AVR Rust requires **Rust Nightly** and the AVR LLVM backend.

### Prerequisites

```powershell
# Rust nightly + AVR components
rustup install nightly
rustup component add llvm-tools-preview rust-src --toolchain nightly

# cargo-binutils (for .hex generation)
cargo install cargo-binutils

# ravedude (for one-command flash via cargo run)
cargo install ravedude
```

The AVR linker (`avr-gcc`) is provided by the **Arduino IDE** install already on this machine:

```powershell
# Temporarily add avr-gcc to PATH (already handled by build.ps1 automatically):
$env:PATH += ";$env:LOCALAPPDATA\Arduino15\packages\arduino\tools\avr-gcc\7.3.0-atmel3.6.1-arduino7\bin"
avr-gcc --version   # verify
```

### Build (manual)

```powershell
cd hello-world-mega
cargo +nightly build -Zjson-target-spec --release
```

### Generate HEX (manual)

```powershell
cargo +nightly objcopy -Zjson-target-spec --release -- -O ihex firmware.hex
```

### One-liner: build.ps1

The `build.ps1` script in `hello-world-mega/` handles PATH setup, build, and flash in a single command:

```powershell
cd hello-world-mega

# Build only — produces firmware.hex
.\build.ps1

# Build + flash on default port COM14
.\build.ps1 -Flash

# Build + flash on a different port
.\build.ps1 -Flash -Port COM7
```

### Flash Options

#### Option A — ravedude (recommended, one command)

1. Uncomment the `runner` line in `hello-world-mega/.cargo/config.toml`:
   ```toml
   runner = "ravedude mega2560 -cb 115200"
   ```
2. Then:
   ```powershell
   cd hello-world-mega
   cargo +nightly run -Z build-std=core --release
   ```
   `ravedude` auto-detects the COM port, builds, and flashes.

#### Option B — avrdude manually

```powershell
# Replace COM3 / COM14 with your actual port (check Device Manager)
avrdude -p atmega2560 -c wiring -P COM14 -b 115200 -D `
  -U flash:w:hello-world-mega\firmware.hex:i
```

Or using the `avrdude` bundled with Arduino IDE (already on this machine):

```powershell
$AVRDUDE     = "$env:LOCALAPPDATA\Arduino15\packages\arduino\tools\avrdude\6.3.0-arduino17\bin\avrdude.exe"
$AVRDUDE_CONF = "$env:LOCALAPPDATA\Arduino15\packages\arduino\tools\avrdude\6.3.0-arduino17\etc\avrdude.conf"

& $AVRDUDE -C $AVRDUDE_CONF `
    -p atmega2560 -c wiring -P COM14 -b 115200 -D `
    -U "flash:w:hello-world-mega\firmware.hex:i"
```

#### Option C — Arduino IDE

1. Build the `.hex` as above (or use the pre-built `hello-world-mega/firmware.hex`).
2. In Arduino IDE: **Sketch → Upload Using Programmer**, then select the `.hex`.

### Monitor Serial Output

The Mega firmware outputs over **UART0** (USB CH340/FT232 bridge) at **115 200 baud**:

```powershell
# Python miniterm
python -m serial.tools.miniterm COM14 115200

# Or use Arduino IDE Serial Monitor at 115200 baud
```

Log format is identical to the Teensy version — `RAW,...` and `PROC,...` CSV lines.

> See [`hello-world-mega/README.md`](hello-world-mega/README.md) for the full code manipulation guide (pin rewiring, filter tuning, ADC reference, etc.).

---

*Built with [teensy4-bsp](https://github.com/mciantyre/teensy4-rs), [RTIC v2](https://rtic.rs), and [imxrt-log](https://github.com/imxrt-rs/imxrt-log).*
