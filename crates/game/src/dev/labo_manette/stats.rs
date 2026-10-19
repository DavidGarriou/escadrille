//! Pure statistics of the gamepad lab (no Bevy), so they can be unit-tested.

/// Number of intervals between 0 and 1; values are rounded to the nearest of `BINS + 1` levels.
pub const BINS: usize = 20;

/// Minimum number of non-zero samples before giving a verdict.
const MIN_PRESSED_SAMPLES: u32 = 10;

/// Minimum number of distinct intermediate levels to call a trigger progressive.
const MIN_INTERMEDIATE_LEVELS: usize = 3;

/// Histogram of the raw values reported by a trigger, in `0..=1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerHistogram {
    counts: [u32; BINS + 1],
}

impl Default for TriggerHistogram {
    fn default() -> Self {
        Self {
            counts: [0; BINS + 1],
        }
    }
}

/// What the histogram says about the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerVerdict {
    /// Not enough presses yet.
    NotEnoughData,
    /// Many intermediate values: an analog trigger.
    Progressive,
    /// Only fully released and fully pressed: a digital trigger.
    AllOrNothing,
    /// A few intermediate values: keep pressing slowly to decide.
    Unclear,
}

impl TriggerHistogram {
    pub fn record(&mut self, value: f32) {
        let v = if value.is_finite() {
            value.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let bin = (v * BINS as f32).round() as usize;
        self.counts[bin.min(BINS)] += 1;
    }

    pub fn counts(&self) -> &[u32; BINS + 1] {
        &self.counts
    }

    pub fn total(&self) -> u32 {
        self.counts.iter().sum()
    }

    /// Distinct levels seen strictly between released (0) and fully pressed (1).
    pub fn intermediate_levels(&self) -> usize {
        self.counts[1..BINS].iter().filter(|&&c| c > 0).count()
    }

    pub fn verdict(&self) -> TriggerVerdict {
        let pressed: u32 = self.counts[1..].iter().sum();
        let intermediate = self.intermediate_levels();
        if intermediate >= MIN_INTERMEDIATE_LEVELS {
            TriggerVerdict::Progressive
        } else if pressed < MIN_PRESSED_SAMPLES {
            TriggerVerdict::NotEnoughData
        } else if intermediate == 0 {
            TriggerVerdict::AllOrNothing
        } else {
            TriggerVerdict::Unclear
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_trigger_is_all_or_nothing() {
        let mut h = TriggerHistogram::default();
        for _ in 0..12 {
            h.record(1.0);
            h.record(0.0);
        }
        assert_eq!(h.intermediate_levels(), 0);
        assert_eq!(h.verdict(), TriggerVerdict::AllOrNothing);
    }

    #[test]
    fn slow_analog_pull_is_progressive() {
        let mut h = TriggerHistogram::default();
        for i in 0..=100 {
            h.record(i as f32 / 100.0);
        }
        assert_eq!(h.intermediate_levels(), BINS - 1);
        assert_eq!(h.verdict(), TriggerVerdict::Progressive);
    }

    #[test]
    fn few_presses_give_no_verdict() {
        let mut h = TriggerHistogram::default();
        h.record(1.0);
        h.record(0.0);
        assert_eq!(h.verdict(), TriggerVerdict::NotEnoughData);
    }

    #[test]
    fn out_of_range_values_are_clamped() {
        let mut h = TriggerHistogram::default();
        h.record(-0.5);
        h.record(1.7);
        h.record(f32::NAN);
        assert_eq!(h.counts()[0], 2);
        assert_eq!(h.counts()[BINS], 1);
        assert_eq!(h.total(), 3);
    }
}
