// src/sensor.rs
//
// Pure signal-processing state machine for the SGX IR1 / IR12EM NDIR sensor.
// No HAL dependencies — only plain Rust math, no_std compatible.
//
// Signal chain (inputs should be pre-filtered by filter.rs):
//   capture_on(act, ref, temp)          ← ON half-cycle
//   capture_off_and_process(act, ref, temp) ← OFF half-cycle → ProcessedSample
//
// ref_diff guard: if ref_diff < crate::filter::REF_DIFF_MIN the pair is dropped.
// Tune REF_DIFF_MIN in src/filter.rs.

#![allow(dead_code)]

use crate::filter::REF_DIFF_MIN;

// ─── Half-cycle raw sample ────────────────────────────────────────────────────

#[derive(Clone, Copy, Default)]
pub struct HalfSample {
    pub act:       u16,
    pub reference: u16,
    pub temp:      u16,
}

// ─── Processed pair output ────────────────────────────────────────────────────

/// Values produced after one complete ON/OFF lamp pair.
/// These are unfiltered processed values; apply `ProcessedFilter` afterwards.
#[derive(Clone, Copy, Default)]
pub struct ProcessedSample {
    /// |ACT_on − ACT_off|
    pub act_diff: u16,
    /// |REF_on − REF_off|
    pub ref_diff: u16,
    /// act_diff / ref_diff  (unfiltered ratio)
    pub ratio: f32,
    /// Current baseline (running mean during accumulation, fixed afterwards)
    pub baseline_ratio: f32,
    /// ((ratio - baseline) / baseline) * 100   — 0 during baseline phase
    pub response_pct: f32,
    /// true once baseline accumulation is complete
    pub baseline_ready: bool,
    /// Mean TEMP ADC count across ON and OFF half-cycles
    pub temp_avg_raw: u16,
}

// ─── NDIR state machine ───────────────────────────────────────────────────────

pub struct NdirState {
    lamp_on:         bool,
    on_sample:       Option<HalfSample>,
    baseline_sum:    f32,
    baseline_count:  u32,
    baseline_target: u32,
    baseline_ratio:  f32,
    baseline_ready:  bool,
}

impl NdirState {
    /// `baseline_target` — ON/OFF pairs to average for the ambient baseline.
    pub const fn new(baseline_target: u32) -> Self {
        Self {
            lamp_on:         false,
            on_sample:       None,
            baseline_sum:    0.0,
            baseline_count:  0,
            baseline_target,
            baseline_ratio:  1.0,
            baseline_ready:  false,
        }
    }

    pub fn lamp_on(&self) -> bool { self.lamp_on }
    pub fn set_lamp_on(&mut self, on: bool) { self.lamp_on = on; }
    pub fn baseline_ready(&self) -> bool { self.baseline_ready }

    // ── Data capture ──────────────────────────────────────────────────────────

    /// Store the ON-phase sample.
    /// Pass pre-filtered values from `ChannelFilter::filt_u16()` when
    /// `USE_FILTERED_FOR_PROC = true`, or raw ADC values otherwise.
    pub fn capture_on(&mut self, act: u16, reference: u16, temp: u16) {
        self.on_sample = Some(HalfSample { act, reference, temp });
    }

    /// Store OFF-phase sample, compute and return the processed pair.
    ///
    /// Returns `None` when:
    ///  - no ON sample was previously captured, or
    ///  - `ref_diff < REF_DIFF_MIN` (tune in filter.rs — protects against
    ///    divide-by-zero and noisy ratio spikes when the lamp barely switches)
    pub fn capture_off_and_process(
        &mut self,
        act_off:  u16,
        ref_off:  u16,
        temp_off: u16,
    ) -> Option<ProcessedSample> {
        let on = self.on_sample.take()?;

        let act_diff    = on.act.abs_diff(act_off);
        let ref_diff    = on.reference.abs_diff(ref_off);
        let temp_avg_raw = ((on.temp as u32 + temp_off as u32) / 2) as u16;

        // ── REF_DIFF_MIN guard ────────────────────────────────────────────────
        // Tune REF_DIFF_MIN in src/filter.rs.
        if ref_diff < REF_DIFF_MIN {
            return None;
        }

        let ratio = act_diff as f32 / ref_diff as f32;

        // ── Baseline accumulation ─────────────────────────────────────────────
        if !self.baseline_ready {
            self.baseline_sum   += ratio;
            self.baseline_count += 1;

            if self.baseline_count >= self.baseline_target {
                self.baseline_ratio = self.baseline_sum / self.baseline_count as f32;
                self.baseline_ready = true;
            }

            return Some(ProcessedSample {
                act_diff,
                ref_diff,
                ratio,
                baseline_ratio: self.baseline_ratio,
                response_pct:   0.0,
                baseline_ready: self.baseline_ready,
                temp_avg_raw,
            });
        }

        // ── Measurement phase ─────────────────────────────────────────────────
        let response_pct =
            ((ratio - self.baseline_ratio) / self.baseline_ratio) * 100.0;

        Some(ProcessedSample {
            act_diff,
            ref_diff,
            ratio,
            baseline_ratio: self.baseline_ratio,
            response_pct,
            baseline_ready: true,
            temp_avg_raw,
        })
    }
}
