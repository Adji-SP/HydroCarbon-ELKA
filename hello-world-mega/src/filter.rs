// src/filter.rs
//
// Filtering primitives for no_std ADC signal chains.
// All types are zero-heap, stack-only, safe for RTIC local resources.
//
// Signal chain per ADC channel:
//   raw u16 → Median5 → u16 → EMA → f32
//
// Processed-output chain:
//   ratio (f32) → EMA → ratio_filt
//   response_pct (f32) → EMA → response_filt
//   absorbance = -ln(ratio/baseline) → EMA → absorbance_filt

#![allow(dead_code)]

// ── Tuning constants ──────────────────────────────────────────────────────────
// Edit these to change global filter behaviour.

/// EMA alpha for raw ADC channels (ACT / REF / TEMP).
/// Range (0.0, 1.0].  Higher = faster / noisier.  Lower = slower / smoother.
/// 0.25 ≈ 3-sample effective window.
pub const RAW_EMA_ALPHA: f32 = 0.25;

/// EMA alpha for processed outputs (ratio, response_pct, absorbance).
/// Slightly heavier smoothing than raw by default.
pub const PROC_EMA_ALPHA: f32 = 0.2;

/// Minimum ref_diff (ADC counts) for a valid ratio computation.
/// Pairs with smaller ref_diff are silently skipped.
/// Increase if you see noisy spikes; decrease if lamp drive is weak.
/// For 10-bit ADC (1023 full-scale): 40 ≈ 4% full-scale.
pub const REF_DIFF_MIN: u16 = 40;

/// Low-rail threshold (10-bit ADC).  Samples ≤ this are flagged as railed.
pub const RAIL_LOW: u16 = 8;

/// High-rail threshold.  Change to 4080 when using 12-bit resolution.
pub const RAIL_HIGH: u16 = 1015;

// ── Rail detection ────────────────────────────────────────────────────────────

/// Returns `true` if any of the three raw ADC readings is near a supply rail.
#[inline]
pub fn near_rail(act: u16, reference: u16, temp: u16) -> bool {
    let r = |v: u16| v <= RAIL_LOW || v >= RAIL_HIGH;
    r(act) || r(reference) || r(temp)
}

// ── Exponential Moving Average ────────────────────────────────────────────────

/// Single-channel EMA:  `y[n] = α·x[n] + (1-α)·y[n-1]`
///
/// Cold-start: first call returns input unchanged (no ringing).
pub struct Ema {
    alpha:       f32,
    value:       f32,
    initialized: bool,
}

impl Ema {
    pub const fn new(alpha: f32) -> Self {
        Self { alpha, value: 0.0, initialized: false }
    }

    /// Push a new sample; return the current EMA output.
    pub fn update(&mut self, x: f32) -> f32 {
        if !self.initialized {
            self.value = x;
            self.initialized = true;
        } else {
            self.value = self.alpha * x + (1.0 - self.alpha) * self.value;
        }
        self.value
    }

    #[inline] pub fn get(&self) -> f32 { self.value }

    /// EMA output rounded to the nearest u16, saturating.
    #[inline]
    pub fn as_u16(&self) -> u16 {
        self.value.max(0.0).min(u16::MAX as f32) as u16
    }
}

// ── Median-of-5 sliding window ────────────────────────────────────────────────

/// Circular buffer of 5 samples; returns the median on each `update()`.
/// Before the window fills, returns the median of however many samples exist.
pub struct Median5 {
    buf:    [u16; 5],
    idx:    usize,
    filled: u8,
}

impl Median5 {
    pub const fn new() -> Self {
        Self { buf: [0; 5], idx: 0, filled: 0 }
    }

    /// Push a new ADC sample; return the current window median.
    pub fn update(&mut self, x: u16) -> u16 {
        self.buf[self.idx] = x;
        self.idx = (self.idx + 1) % 5;
        if self.filled < 5 { self.filled += 1; }

        let n = self.filled as usize;
        // Sort a scratch copy — O(n²) on 5 elements, trivially fast.
        let mut s = [0u16; 5];
        s[..n].copy_from_slice(&self.buf[..n]); // valid: see note below
        // Note: when filled==5 the buffer is fully occupied; all 5 slots are
        // valid regardless of insertion order — median is order-independent.
        for i in 1..n {
            let key = s[i];
            let mut j = i;
            while j > 0 && s[j - 1] > key { s[j] = s[j - 1]; j -= 1; }
            s[j] = key;
        }
        s[n / 2]
    }
}

// ── Per-channel filter: Median5 → EMA ────────────────────────────────────────

/// Combined filter for a single ADC channel.
/// Call `update(raw)` every sample tick.
pub struct ChannelFilter {
    pub median: Median5,
    pub ema:    Ema,
}

impl ChannelFilter {
    /// `alpha` — EMA weight.  Use `RAW_EMA_ALPHA` as the default.
    pub const fn new(alpha: f32) -> Self {
        Self { median: Median5::new(), ema: Ema::new(alpha) }
    }

    /// Push raw ADC count.  Returns `(median_u16, ema_f32)`.
    pub fn update(&mut self, raw: u16) -> (u16, f32) {
        let med = self.median.update(raw);
        let ema = self.ema.update(med as f32);
        (med, ema)
    }

    /// EMA output rounded to u16 — use this to feed sensor.rs.
    #[inline] pub fn filt_u16(&self) -> u16 { self.ema.as_u16() }
}

// ── Processed-output filter ───────────────────────────────────────────────────

/// EMA filters for ratio, response_pct, and absorbance.
pub struct ProcessedFilter {
    pub ratio_ema:      Ema,
    pub response_ema:   Ema,
    pub absorbance_ema: Ema,
}

impl ProcessedFilter {
    pub const fn new(alpha: f32) -> Self {
        Self {
            ratio_ema:      Ema::new(alpha),
            response_ema:   Ema::new(alpha),
            absorbance_ema: Ema::new(alpha),
        }
    }

    /// Feed raw processed values; return all filtered versions.
    pub fn update(
        &mut self,
        ratio: f32,
        response_pct: f32,
        baseline_ratio: f32,
    ) -> FilteredProcessed {
        let ratio_filt    = self.ratio_ema.update(ratio);
        let response_filt = self.response_ema.update(response_pct);
        let abs_raw       = absorbance(ratio, baseline_ratio);
        let abs_filt      = self.absorbance_ema.update(abs_raw);
        FilteredProcessed { ratio_filt, response_filt, abs_raw, abs_filt }
    }
}

/// Output of `ProcessedFilter::update`.
#[derive(Clone, Copy)]
pub struct FilteredProcessed {
    pub ratio_filt:    f32,
    pub response_filt: f32,
    pub abs_raw:       f32,
    pub abs_filt:      f32,
}

// ── Beer-Lambert absorbance ───────────────────────────────────────────────────

/// `A = -ln(ratio / baseline_ratio)`.  Returns 0 if either value ≤ 0.
pub fn absorbance(ratio: f32, baseline_ratio: f32) -> f32 {
    if ratio <= 0.0 || baseline_ratio <= 0.0 { return 0.0; }
    -ln_approx(ratio / baseline_ratio)
}

/// Padé-1 ln approximation with range-reduction.
/// Accurate to ~1 % for 0.5 < x < 2.0 (normal NDIR ratios).
/// To get full precision, add the `libm` crate and replace with `libm::logf(x)`.
fn ln_approx(x: f32) -> f32 {
    if x <= 0.0 { return 0.0; }
    let mut val = x;
    let mut adj = 0.0_f32;
    // Scale toward [0.5, 2.0) tracking the log of the scale factor.
    while val > 2.0 { val *= 0.5; adj += core::f32::consts::LN_2; }
    while val < 0.5 { val *= 2.0; adj -= core::f32::consts::LN_2; }
    // 1st-order Padé: ln(x) ≈ 2·(x-1)/(x+1)
    let t = val - 1.0;
    2.0 * t / (val + 1.0) + adj
}
