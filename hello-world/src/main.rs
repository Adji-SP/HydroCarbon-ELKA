//! NDIR gas-sensor firmware — Teensy 4.1
//!
//! Signal chain (each stage independently enable/disable via const flags):
//!   ADC raw → [Median5] → [EMA] (ChannelFilter)  ← USE_MEDIAN_FILTER / USE_RAW_EMA
//!          ↓ filtered u16 values
//!   NdirState::capture_on / capture_off_and_process
//!          ↓ ProcessedSample { ratio, response_pct, … }
//!   [ProcessedFilter::update] → FilteredProcessed { ratio_filt, response_filt, absorbance… }
//!                             ← USE_PROC_EMA
//!
//! Log formats (USB, one line each):
//!   RAW,<t>,<ph>,<act_r>,<ref_r>,<tmp_r>,<act_m>,<ref_m>,<tmp_m>,<act_f>,<ref_f>,<tmp_f>,<rail>
//!   PROC,<t>,<bl>,<adiff>,<rdiff>,<ratio_r>,<ratio_f>,<base>,<rsp_r>,<rsp_f>,<abs_r>,<abs_f>,<tavg>
//!
//! Where: t=time_s.ms  ph=1(ON)/0(OFF)  bl=0(baseline)/1(ready)  rail=0/1
//!        _r=raw  _m=median  _f=filtered(EMA)

#![no_std]
#![no_main]

use teensy4_panic as _;

mod filter;
mod sensor;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;

    use bsp::hal::adc;
    use imxrt_log as logging;

    use board::t41 as my_board;

    use rtic_monotonics::systick::{Systick, *};

    use crate::sensor::NdirState;
    use crate::filter::{
        ChannelFilter, ProcessedFilter,
        near_rail,
        RAW_EMA_ALPHA, PROC_EMA_ALPHA,
    };

    // ── Mode switches ─────────────────────────────────────────────────────────
    //
    // AMBIENT_TEST_MODE = true  → lamp pin held LOW; pseudo-ON/OFF cycles for
    //                             bench testing without a real lamp.
    // AMBIENT_TEST_MODE = false → real IR lamp toggled at HALF_PERIOD_MS.
    const AMBIENT_TEST_MODE: bool = true;

    // USE_FILTERED_FOR_PROC = true  → sensor.rs receives EMA-filtered ADC values.
    // USE_FILTERED_FOR_PROC = false → sensor.rs receives true-raw ADC values.
    // Either way both raw and filtered are always logged in the RAW line.
    const USE_FILTERED_FOR_PROC: bool = true;

    // ── Filter enable/disable ─────────────────────────────────────────────────
    //
    // Each stage of the signal chain can be disabled independently.
    // Disabling a stage passes the previous value through unchanged:
    //
    //   USE_MEDIAN_FILTER  true  → Median-of-5 denoising before EMA.
    //                     false  → raw ADC value is fed directly into EMA
    //                              (or directly to sensor if USE_RAW_EMA also false).
    //
    //   USE_RAW_EMA        true  → EMA smoothing on each ADC channel.
    //                     false  → median output (or raw if USE_MEDIAN_FILTER false)
    //                              is used as-is; _f columns equal _m columns in log.
    //
    //   USE_PROC_EMA       true  → EMA smoothing on ratio / response / absorbance.
    //                     false  → filtered columns equal raw columns in PROC log.
    const USE_MEDIAN_FILTER: bool = true;
    const USE_RAW_EMA:       bool = true;
    const USE_PROC_EMA:      bool = true;

    // ── Logging switches ──────────────────────────────────────────────────────
    // false = that log line is completely suppressed (zero flash cost).
    const LOG_RAW:  bool = true;  // emit RAW,... every half-period tick
    const LOG_PROC: bool = true;  // emit PROC,... every OFF tick (per pair)

    // ── Timing ────────────────────────────────────────────────────────────────
    /// 4 Hz lamp, 50% duty → 125 ms half-period.
    const HALF_PERIOD_MS: u32 = 125;

    /// Pairs to average for ambient baseline.  80 × 250 ms ≈ 20 s.
    const BASELINE_CYCLES: u32 = 80;

    // ── Pin / ADC type aliases ────────────────────────────────────────────────
    // Lamp drive: Teensy pin 2 → GPIO_EMC_04 → GPIO4.
    // Change P2 + gpio4 below if you re-wire the MOSFET gate.
    type IrDrivePin = bsp::hal::gpio::Output<bsp::pins::common::P2>;

    // ACT/REF/TEMP on Teensy pins 14/15/16 (A0/A1/A2), all on ADC1.
    // Change P14/P15/P16 and the pins.pN below if you re-route.
    type ActPin  = adc::AnalogInput<bsp::pins::common::P14, 1>;
    type RefPin  = adc::AnalogInput<bsp::pins::common::P15, 1>;
    type TempPin = adc::AnalogInput<bsp::pins::common::P16, 1>;
    type Adc1    = adc::Adc<1>;

    // ── RTIC resources ────────────────────────────────────────────────────────
    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led:       board::Led,
        poller:    logging::Poller,
        ir_drive:  IrDrivePin,
        adc1:      Adc1,
        act:       ActPin,
        reference: RefPin,
        temp:      TempPin,
        ndir:      NdirState,
        /// Ms since boot; incremented HALF_PERIOD_MS per loop tick.
        tick_ms:   u32,
        // Per-channel filters (Median-of-5 → EMA).
        // Tune alpha via RAW_EMA_ALPHA in src/filter.rs.
        act_cf:    ChannelFilter,
        ref_cf:    ChannelFilter,
        temp_cf:   ChannelFilter,
        // EMA on processed outputs (ratio, response_pct, absorbance).
        // Tune alpha via PROC_EMA_ALPHA in src/filter.rs.
        proc_filt: ProcessedFilter,
    }

    // ── init ──────────────────────────────────────────────────────────────────
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut gpio2,  // Port<2> — LED on pin 13
            mut gpio4,  // Port<4> — lamp pin 2 lives here
            pins,
            usb,
            adc1,
            ..
        } = my_board(cx.device);

        let led    = board::led(&mut gpio2, pins.p13);
        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        Systick::start(
            cx.core.SYST,
            board::ARM_FREQUENCY,
            rtic_monotonics::create_systick_token!(),
        );

        // Lamp pin: change gpio4 / pins.p2 / IrDrivePin if you re-wire.
        let ir_drive: IrDrivePin = gpio4.output(pins.p2);
        ir_drive.clear(); // ensure lamp starts OFF

        // ADC inputs: change pins.pN / type aliases above if you re-route.
        let act:       ActPin  = adc::AnalogInput::new(pins.p14);
        let reference: RefPin  = adc::AnalogInput::new(pins.p15);
        let temp:      TempPin = adc::AnalogInput::new(pins.p16);

        // Optional: bump to 12-bit and re-calibrate (update RAIL_HIGH too).
        // adc1.set_resolution(adc::ResolutionBits::Bits12);
        // adc1.calibrate();

        let ndir      = NdirState::new(BASELINE_CYCLES);
        let act_cf    = ChannelFilter::new(RAW_EMA_ALPHA);
        let ref_cf    = ChannelFilter::new(RAW_EMA_ALPHA);
        let temp_cf   = ChannelFilter::new(RAW_EMA_ALPHA);
        let proc_filt = ProcessedFilter::new(PROC_EMA_ALPHA);

        sensor_loop::spawn().unwrap();

        (
            Shared {},
            Local {
                led, poller, ir_drive,
                adc1, act, reference, temp,
                ndir, tick_ms: 0,
                act_cf, ref_cf, temp_cf, proc_filt,
            },
        )
    }

    // ── Sensor task ───────────────────────────────────────────────────────────
    //
    // Every HALF_PERIOD_MS (125 ms):
    //   1. Read raw ADC
    //   2. Run ChannelFilter (Median5 → EMA) per channel
    //   3. Check rail
    //   4. Emit RAW log line (if LOG_RAW)
    //   5. Feed (filtered or raw) values into NdirState
    //   6. ON tick  → store, toggle lamp off
    //      OFF tick → compute ProcessedSample, filter, emit PROC log
    #[task(local = [
        led, ir_drive, adc1, act, reference, temp,
        ndir, tick_ms,
        act_cf, ref_cf, temp_cf, proc_filt,
    ])]
    async fn sensor_loop(cx: sensor_loop::Context) {
        if AMBIENT_TEST_MODE {
            cx.local.ir_drive.clear();
            log::info!("*** AMBIENT TEST MODE — lamp held LOW ***");
        } else {
            cx.local.ir_drive.set();
            cx.local.ndir.set_lamp_on(true);
            log::info!("*** IR LAMP MODE — {}ms half-period ***", HALF_PERIOD_MS);
        }
        log::info!("Baseline: {} pairs", BASELINE_CYCLES);
        log::info!("RAW hdr: time,ph,act_r,ref_r,tmp_r,act_m,ref_m,tmp_m,act_f,ref_f,tmp_f,rail");
        log::info!("PROC hdr: time,bl,adiff,rdiff,ratio_r,ratio_f,base,rsp_r,rsp_f,abs_r,abs_f,tavg");

        loop {
            Systick::delay(HALF_PERIOD_MS.millis()).await;

            // ── elapsed time ──────────────────────────────────────────────────
            *cx.local.tick_ms = cx.local.tick_ms.wrapping_add(HALF_PERIOD_MS);
            let elapsed_ms = *cx.local.tick_ms;
            let time_s  = elapsed_ms / 1000;
            let time_ms = elapsed_ms % 1000;

            // ── ADC reads ─────────────────────────────────────────────────────
            let act_raw  = cx.local.adc1.read_blocking(cx.local.act);
            let ref_raw  = cx.local.adc1.read_blocking(cx.local.reference);
            let temp_raw = cx.local.adc1.read_blocking(cx.local.temp);

            // ── Channel filters (Median5 → EMA) ───────────────────────────────
            // Stages are independently gated by USE_MEDIAN_FILTER / USE_RAW_EMA.
            // The raw update() call is always made so the circular buffers keep
            // ticking, but the *output* is substituted when a stage is disabled.
            let (act_med_full,  act_ema_full)  = cx.local.act_cf.update(act_raw);
            let (ref_med_full,  ref_ema_full)  = cx.local.ref_cf.update(ref_raw);
            let (temp_med_full, temp_ema_full) = cx.local.temp_cf.update(temp_raw);

            // Median output: real median or pass-through raw.
            let act_med  = if USE_MEDIAN_FILTER { act_med_full  } else { act_raw };
            let ref_med  = if USE_MEDIAN_FILTER { ref_med_full  } else { ref_raw };
            let temp_med = if USE_MEDIAN_FILTER { temp_med_full } else { temp_raw };

            // EMA output: real EMA or pass-through median.
            let act_ema  = if USE_RAW_EMA { act_ema_full  } else { act_med  as f32 };
            let ref_ema  = if USE_RAW_EMA { ref_ema_full  } else { ref_med  as f32 };
            let temp_ema = if USE_RAW_EMA { temp_ema_full } else { temp_med as f32 };

            // Rounded filtered u16 values — feed to sensor when USE_FILTERED_FOR_PROC.
            let act_f  = if USE_RAW_EMA { cx.local.act_cf.filt_u16()  } else { act_med  };
            let ref_f  = if USE_RAW_EMA { cx.local.ref_cf.filt_u16()  } else { ref_med  };
            let temp_f = if USE_RAW_EMA { cx.local.temp_cf.filt_u16() } else { temp_med };

            // ── Rail detection ────────────────────────────────────────────────
            // Tune RAIL_LOW / RAIL_HIGH in src/filter.rs.
            let rail = near_rail(act_raw, ref_raw, temp_raw);

            // ── Determine what feeds sensor math ──────────────────────────────
            let (sa, sr, st) = if USE_FILTERED_FOR_PROC {
                (act_f, ref_f, temp_f)
            } else {
                (act_raw, ref_raw, temp_raw)
            };

            let phase_on = cx.local.ndir.lamp_on();

            // ── RAW log line (every tick) ─────────────────────────────────────
            // Columns: time, phase, act_raw, ref_raw, temp_raw,
            //          act_med, ref_med, temp_med, act_ema, ref_ema, temp_ema, rail
            // EMA values logged as one-decimal floats.
            if LOG_RAW {
                log::info!(
                    "RAW,{}.{:03},{},{},{},{},{},{},{},{:.1},{:.1},{:.1},{}",
                    time_s, time_ms,
                    phase_on as u8,
                    act_raw, ref_raw, temp_raw,
                    act_med, ref_med, temp_med,
                    act_ema, ref_ema, temp_ema,
                    rail as u8,
                );
            }

            // ── State machine ─────────────────────────────────────────────────
            if phase_on {
                // ON half-cycle: store sample, toggle lamp off.
                cx.local.ndir.capture_on(sa, sr, st);
                if !AMBIENT_TEST_MODE { cx.local.ir_drive.clear(); }
                cx.local.ndir.set_lamp_on(false);
            } else {
                // OFF half-cycle: process pair.
                let result = cx.local.ndir.capture_off_and_process(sa, sr, st);

                match result {
                    Some(s) => {
                        cx.local.led.toggle(); // heartbeat blink per pair

                        // ── ProcessedFilter (EMA on ratio / response / absorbance) ──
                        // Gated by USE_PROC_EMA; when false the _filt columns equal
                        // the raw values so the PROC log format stays identical.
                        // Tune PROC_EMA_ALPHA in src/filter.rs.
                        let fp = if USE_PROC_EMA {
                            cx.local.proc_filt.update(
                                s.ratio,
                                s.response_pct,
                                s.baseline_ratio,
                            )
                        } else {
                            use crate::filter::{FilteredProcessed, absorbance};
                            FilteredProcessed {
                                ratio_filt:    s.ratio,
                                response_filt: s.response_pct,
                                abs_raw:       absorbance(s.ratio, s.baseline_ratio),
                                abs_filt:      absorbance(s.ratio, s.baseline_ratio),
                            }
                        };

                        // ── PROC log line ──────────────────────────────────────
                        // Columns: time, bl(0=baseline/1=ready),
                        //          act_diff, ref_diff,
                        //          ratio_raw, ratio_filt,
                        //          baseline,
                        //          response_raw, response_filt,
                        //          absorbance_raw, absorbance_filt,
                        //          temp_avg
                        if LOG_PROC {
                            log::info!(
                                "PROC,{}.{:03},{},{},{},{:.4},{:.4},{:.4},{:+.2},{:+.2},{:.4},{:.4},{}",
                                time_s, time_ms,
                                s.baseline_ready as u8,
                                s.act_diff, s.ref_diff,
                                s.ratio,       fp.ratio_filt,
                                s.baseline_ratio,
                                s.response_pct, fp.response_filt,
                                fp.abs_raw,     fp.abs_filt,
                                s.temp_avg_raw,
                            );
                        }
                    }
                    None => {
                        // Skipped: ref_diff < REF_DIFF_MIN (tune in filter.rs).
                        if AMBIENT_TEST_MODE {
                            log::debug!("pair skip: ref_diff<REF_DIFF_MIN (normal in ambient test)");
                        } else {
                            log::warn!("pair skip: ref_diff<REF_DIFF_MIN — lamp switching?");
                        }
                    }
                }

                // Restart lamp for next pair.
                if !AMBIENT_TEST_MODE { cx.local.ir_drive.set(); }
                cx.local.ndir.set_lamp_on(true);
            }
        }
    }

    // ── USB logging interrupt ──────────────────────────────────────────────────
    #[task(binds = USB_OTG1, local = [poller])]
    fn log_over_usb(cx: log_over_usb::Context) {
        cx.local.poller.poll();
    }
}
