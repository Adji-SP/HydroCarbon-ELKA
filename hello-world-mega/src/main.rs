//! NDIR gas-sensor firmware — Arduino Mega (ATmega2560)
//!
//! Direct port of `hello-world` (Teensy 4.1 / RTIC / Rust) to the
//! Arduino Mega platform using `arduino-hal`.
//!
//! Signal chain — identical to the Teensy version:
//!   ADC raw → [Median5] → [EMA] (ChannelFilter)
//!          ↓
//!   NdirState::capture_on / capture_off_and_process
//!          ↓
//!   ProcessedFilter → FilteredProcessed
//!
//! Log format (UART0 / USB-Serial, 115 200 baud) — identical to log/data2_raw.csv and log/data2_proc.csv:
//!
//!   RAW:  type,timestamp,phase,ch1_raw,ch2_raw,ch3_raw,ch1_filt,ch2_filt,ch3_filt,ema1,ema2,ema3,flag
//!   PROC: type,timestamp,gas_flag,raw_val1,raw_val2,ratio,ratio_filt,baseline,delta1,delta2,response1,response2,ema_out
//!
//! Hardware mapping (Arduino Mega):
//!   IR lamp MOSFET gate → Digital pin D2   (change IR_DRIVE if re-wired)
//!   LED heartbeat       → Digital pin D13  (built-in)
//!   ACT  photodiode     → Analog  pin A0   (change if re-routed)
//!   REF  photodiode     → Analog  pin A1
//!   TEMP thermistor     → Analog  pin A2
//!   Serial (TX/RX)      → D1/D0  (USART0) at 115 200 baud
//!
//! Build (requires Rust nightly + avr-llvm):
//!   cargo +nightly build -Z build-std=core --release
//! Flash (requires ravedude):
//!   cargo +nightly run   -Z build-std=core --release

#![no_std]
#![no_main]

use panic_halt as _;

mod filter;
mod sensor;

use crate::filter::{
    absorbance, ChannelFilter, FilteredProcessed, ProcessedFilter,
    near_rail, RAW_EMA_ALPHA, PROC_EMA_ALPHA,
};
use crate::sensor::NdirState;

// ── Mode switches ──────────────────────────────────────────────────────────────
// AMBIENT_TEST_MODE = true  → lamp pin held LOW; pseudo-ON/OFF for bench test.
// AMBIENT_TEST_MODE = false → real IR lamp toggled at HALF_PERIOD_MS.
const AMBIENT_TEST_MODE:     bool = false;

// USE_FILTERED_FOR_PROC = true  → sensor pair uses EMA-filtered ADC values.
//   WARNING: EMA/Median5 buffers mix ON and OFF phase samples, collapsing
//   ref_diff to near-zero and causing every pair to be skipped on AVR.
//   Keep false for AVR — use raw ADC for pair math; filtering is still logged.
// USE_FILTERED_FOR_PROC = false → sensor pair uses true-raw ADC values.
const USE_FILTERED_FOR_PROC: bool = false;

// ── Filter enable/disable ──────────────────────────────────────────────────────
const USE_MEDIAN_FILTER: bool = true;
const USE_RAW_EMA:       bool = true;
const USE_PROC_EMA:      bool = true;

// ── Logging switches ───────────────────────────────────────────────────────────
const LOG_RAW:  bool = true;
const LOG_PROC: bool = true;

// ── Timing ────────────────────────────────────────────────────────────────────
/// 4 Hz lamp, 50 % duty → 125 ms half-period.
const HALF_PERIOD_MS: u32 = 125;

/// ON/OFF pairs to average for the ambient baseline.  80 × 250 ms ≈ 20 s.
const BASELINE_CYCLES: u32 = 80;

// ── Clock speed ───────────────────────────────────────────────────────────────
// Arduino Mega 2560 spec: 16 MHz crystal (ATmega2560 datasheet §8).
// The `arduino-mega2560` feature sets DefaultClock = MHz16 in arduino-hal,
// which calibrates delay_ms/delay_us busy-loops to exactly 16 000 000 Hz.
// Declaring the alias here makes it compile-time-visible and guards against
// accidentally using a wrong board feature (e.g. MHz8 for a 3.3 V variant).
#[allow(dead_code)]
type BoardClock = arduino_hal::DefaultClock;  // MHz16 = 16_000_000 Hz
/// Explicit Hz constant — used only for documentation; delay_ms uses BoardClock.
#[allow(dead_code)]
const MCU_CLOCK_HZ: u32 = 16_000_000;

// ── Float-to-serial helpers ───────────────────────────────────────────────────
//
// `ufmt` has no built-in f32 support and `core::fmt` is too large for AVR.
// We do fixed-decimal formatting with integer arithmetic only.

/// Write `val` to `w` with exactly `decimals` fractional digits.
fn write_f32<W: ufmt::uWrite>(w: &mut W, val: f32, decimals: u8) {
    // Handle negative values.
    if val < 0.0_f32 {
        let _ = ufmt::uwrite!(w, "-");
        write_f32(w, -val, decimals);
        return;
    }

    // Compute scale factor (10^decimals).
    let mut scale: u32 = 1;
    for _ in 0..decimals {
        scale *= 10;
    }

    let int_part  = val as u32;
    let remainder = val - (int_part as f32);
    // Round frac part — guard overflow from rounding.
    let frac_raw  = (remainder * scale as f32 + 0.5_f32) as u32;
    let (int_part, frac_part) = if frac_raw >= scale {
        (int_part + 1, 0u32)
    } else {
        (int_part, frac_raw)
    };

    let _ = ufmt::uwrite!(w, "{}.", int_part);

    // Print fractional part with leading zeros.
    let mut lead = scale / 10;
    while lead >= 1 {
        if frac_part < lead {
            let _ = ufmt::uwrite!(w, "0");
        }
        if lead == 1 { break; }
        lead /= 10;
    }
    let _ = ufmt::uwrite!(w, "{}", frac_part);
}

/// Like `write_f32` but always prints a leading '+' or '-' sign.
fn write_f32_signed<W: ufmt::uWrite>(w: &mut W, val: f32, decimals: u8) {
    if val >= 0.0_f32 {
        let _ = ufmt::uwrite!(w, "+");
    }
    write_f32(w, val, decimals);
}

/// Write elapsed time as "seconds.milliseconds" (e.g. "12.005").
fn write_time<W: ufmt::uWrite>(w: &mut W, elapsed_ms: u32) {
    let s  = elapsed_ms / 1000;
    let ms = elapsed_ms % 1000;
    let _ = ufmt::uwrite!(w, "{}.", s);
    if ms < 100 { let _ = ufmt::uwrite!(w, "0"); }
    if ms < 10  { let _ = ufmt::uwrite!(w, "0"); }
    let _ = ufmt::uwrite!(w, "{}", ms);
}

// ── Entry point ───────────────────────────────────────────────────────────────
#[arduino_hal::entry]
fn main() -> ! {
    let dp   = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // UART0 (D0 RX / D1 TX) — connect USB-Serial adapter or use the Mega's
    // built-in USB-to-serial (CH340 / FT232) chip.
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    // ADC — 10-bit, DEFAULT (AVCC = 5 V) reference.
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // IR lamp MOSFET gate (Digital pin 2). Change to another pin if re-wired.
    let mut ir_drive = pins.d2.into_output();
    ir_drive.set_low(); // lamp starts OFF

    // Built-in LED (Digital pin 13) — heartbeat blink per processed pair.
    let mut led = pins.d13.into_output();
    led.set_low();

    // Analog inputs: A0 = ACT, A1 = REF, A2 = TEMP.
    // Change pin labels below if you re-route.
    let act_pin  = pins.a0.into_analog_input(&mut adc);
    let ref_pin  = pins.a1.into_analog_input(&mut adc);
    let temp_pin = pins.a2.into_analog_input(&mut adc);

    // ── Filter / sensor state ─────────────────────────────────────────────────
    let mut act_cf    = ChannelFilter::new(RAW_EMA_ALPHA);
    let mut ref_cf    = ChannelFilter::new(RAW_EMA_ALPHA);
    let mut temp_cf   = ChannelFilter::new(RAW_EMA_ALPHA);
    let mut proc_filt = ProcessedFilter::new(PROC_EMA_ALPHA);
    let mut ndir      = NdirState::new(BASELINE_CYCLES);

    let mut tick_ms: u32 = 0;
    let mut led_state    = false;

    // ── Startup messages ──────────────────────────────────────────────────────
    if AMBIENT_TEST_MODE {
        let _ = ufmt::uwriteln!(&mut serial, "*** AMBIENT TEST MODE - lamp held LOW ***");
    } else {
        let _ = ufmt::uwriteln!(
            &mut serial,
            "*** IR LAMP MODE - {}ms half-period ***",
            HALF_PERIOD_MS
        );
        // Kick off the first ON half-cycle.
        ir_drive.set_high();
        ndir.set_lamp_on(true);
    }
    let _ = ufmt::uwriteln!(&mut serial, "Baseline: {} pairs", BASELINE_CYCLES);
    // Print exact CSV column headers matching data2_raw.csv / data2_proc.csv.
    // Capture serial output and split lines by type field to recreate the CSV files.
    if LOG_RAW {
        let _ = ufmt::uwriteln!(
            &mut serial,
            "type,timestamp,phase,ch1_raw,ch2_raw,ch3_raw,ch1_filt,ch2_filt,ch3_filt,ema1,ema2,ema3,flag"
        );
    }
    if LOG_PROC {
        let _ = ufmt::uwriteln!(
            &mut serial,
            "type,timestamp,gas_flag,raw_val1,raw_val2,ratio,ratio_filt,baseline,delta1,delta2,response1,response2,ema_out"
        );
    }

    // ── Main loop ─────────────────────────────────────────────────────────────
    loop {
        // Blocking half-period delay — no drift compensation needed at 125 ms
        // on AVR (ADC read time ≈ 100 µs × 3 channels ≈ negligible).
        arduino_hal::delay_ms(HALF_PERIOD_MS);

        // ── Elapsed time ──────────────────────────────────────────────────────
        tick_ms = tick_ms.wrapping_add(HALF_PERIOD_MS);
        let elapsed_ms = tick_ms;

        // ── ADC reads (10-bit, 0–1023) ────────────────────────────────────────
        let act_raw:  u16 = adc.read_blocking(&act_pin);
        let ref_raw:  u16 = adc.read_blocking(&ref_pin);
        let temp_raw: u16 = adc.read_blocking(&temp_pin);

        // ── Channel filters (Median5 → EMA) ───────────────────────────────────
        let (act_med_full,  act_ema_full)  = act_cf.update(act_raw);
        let (ref_med_full,  ref_ema_full)  = ref_cf.update(ref_raw);
        let (temp_med_full, temp_ema_full) = temp_cf.update(temp_raw);

        // Median stage: real output or pass-through raw.
        let act_med  = if USE_MEDIAN_FILTER { act_med_full  } else { act_raw };
        let ref_med  = if USE_MEDIAN_FILTER { ref_med_full  } else { ref_raw };
        let temp_med = if USE_MEDIAN_FILTER { temp_med_full } else { temp_raw };

        // EMA stage: real output or pass-through median.
        let act_ema  = if USE_RAW_EMA { act_ema_full  } else { act_med  as f32 };
        let ref_ema  = if USE_RAW_EMA { ref_ema_full  } else { ref_med  as f32 };
        let temp_ema = if USE_RAW_EMA { temp_ema_full } else { temp_med as f32 };

        // Rounded filtered u16 — feed to sensor when USE_FILTERED_FOR_PROC.
        let act_f  = if USE_RAW_EMA { act_cf.filt_u16()  } else { act_med  };
        let ref_f  = if USE_RAW_EMA { ref_cf.filt_u16()  } else { ref_med  };
        let temp_f = if USE_RAW_EMA { temp_cf.filt_u16() } else { temp_med };

        // ── Rail detection ────────────────────────────────────────────────────
        let rail = near_rail(act_raw, ref_raw, temp_raw);

        // ── Choose sensor inputs ──────────────────────────────────────────────
        let (sa, sr, st) = if USE_FILTERED_FOR_PROC {
            (act_f, ref_f, temp_f)
        } else {
            (act_raw, ref_raw, temp_raw)
        };

        let phase_on = ndir.lamp_on();

        // ── RAW log line ──────────────────────────────────────────────────────
        // Format: RAW,<s.ms>,<ph>,<act_r>,<ref_r>,<tmp_r>,
        //              <act_m>,<ref_m>,<tmp_m>,<act_f:1>,<ref_f:1>,<tmp_f:1>,<rail>
        if LOG_RAW {
            let _ = ufmt::uwrite!(&mut serial, "RAW,");
            write_time(&mut serial, elapsed_ms);
            let _ = ufmt::uwrite!(
                &mut serial,
                ",{},{},{},{},{},{},{},",
                phase_on as u8,
                act_raw, ref_raw, temp_raw,
                act_med, ref_med, temp_med,
            );
            write_f32(&mut serial, act_ema,  1);
            let _ = ufmt::uwrite!(&mut serial, ",");
            write_f32(&mut serial, ref_ema,  1);
            let _ = ufmt::uwrite!(&mut serial, ",");
            write_f32(&mut serial, temp_ema, 1);
            let _ = ufmt::uwriteln!(&mut serial, ",{}", rail as u8);
        }

        // ── State machine ─────────────────────────────────────────────────────
        if phase_on {
            // ON half-cycle: store sample, turn lamp off.
            ndir.capture_on(sa, sr, st);
            if !AMBIENT_TEST_MODE { ir_drive.set_low(); }
            ndir.set_lamp_on(false);
        } else {
            // OFF half-cycle: process pair.
            match ndir.capture_off_and_process(sa, sr, st) {
                Some(s) => {
                    // Heartbeat blink per valid pair.
                    led_state = !led_state;
                    if led_state { led.set_high(); } else { led.set_low(); }

                    // ProcessedFilter (EMA on ratio / response / absorbance).
                    let fp: FilteredProcessed = if USE_PROC_EMA {
                        proc_filt.update(s.ratio, s.response_pct, s.baseline_ratio)
                    } else {
                        FilteredProcessed {
                            ratio_filt:    s.ratio,
                            response_filt: s.response_pct,
                            abs_raw:       absorbance(s.ratio, s.baseline_ratio),
                            abs_filt:      absorbance(s.ratio, s.baseline_ratio),
                        }
                    };

                    // ── PROC log line ──────────────────────────────────────────
                    // Format: PROC,<s.ms>,<bl>,<adiff>,<rdiff>,
                    //               <ratio_r:4>,<ratio_f:4>,<base:4>,
                    //               <rsp_r:+2>,<rsp_f:+2>,<abs_r:4>,<abs_f:4>,<tavg>
                    if LOG_PROC {
                        let _ = ufmt::uwrite!(&mut serial, "PROC,");
                        write_time(&mut serial, elapsed_ms);
                        let _ = ufmt::uwrite!(
                            &mut serial,
                            ",{},{},{},",
                            s.baseline_ready as u8,
                            s.act_diff,
                            s.ref_diff,
                        );
                        write_f32(&mut serial, s.ratio,          4);
                        let _ = ufmt::uwrite!(&mut serial, ",");
                        write_f32(&mut serial, fp.ratio_filt,    4);
                        let _ = ufmt::uwrite!(&mut serial, ",");
                        write_f32(&mut serial, s.baseline_ratio, 4);
                        let _ = ufmt::uwrite!(&mut serial, ",");
                        write_f32_signed(&mut serial, s.response_pct,   2);
                        let _ = ufmt::uwrite!(&mut serial, ",");
                        write_f32_signed(&mut serial, fp.response_filt, 2);
                        let _ = ufmt::uwrite!(&mut serial, ",");
                        write_f32(&mut serial, fp.abs_raw,       4);
                        let _ = ufmt::uwrite!(&mut serial, ",");
                        write_f32(&mut serial, fp.abs_filt,      4);
                        let _ = ufmt::uwriteln!(&mut serial, ",{}", s.temp_avg_raw);
                    }
                }
                None => {
                    // Skipped: ref_diff < REF_DIFF_MIN
                    if AMBIENT_TEST_MODE {
                        let _ = ufmt::uwriteln!(
                            &mut serial,
                            "DBG pair skip: ref_diff<REF_DIFF_MIN (normal in ambient test)"
                        );
                    } else {
                        let _ = ufmt::uwriteln!(
                            &mut serial,
                            "WRN pair skip: ref_diff<REF_DIFF_MIN - lamp switching?"
                        );
                    }
                }
            }

            // Restart lamp for next ON half-cycle.
            if !AMBIENT_TEST_MODE {
                ir_drive.set_high();
                ndir.set_lamp_on(true);
            }
        }
    }
}
