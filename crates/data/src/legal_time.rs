//! The French legal time of the war (`docs/recherche/heure-legale-1917.md`): the time the
//! reports and the flight logs give, and the scenes' `time`. Greenwich time since the law
//! of 9 March 1911, and the summer time, one hour ahead, between the dates of each year's
//! decree.

use crate::scene::TimeSpec;

/// One year's summer time: when the clocks go forward and back, as (month, day, legal
/// hour before the change).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SummerTime {
    pub year: i32,
    pub forward: (u32, u32, f64),
    pub back: (u32, u32, f64),
}

/// The French summer times of the war (`docs/recherche/heure-legale-1917.md` § 1.3, 1.4:
/// the IANA time-zone base after the Bureau des longitudes, and the British orders that
/// followed the French; confidence B, 1918's end C). The clocks go forward at 23 h,
/// legal winter time, and back at 24 h, legal summer time: 23 h UTC both.
pub const FRENCH_SUMMER_TIME: [SummerTime; 3] = [
    SummerTime {
        year: 1916,
        forward: (6, 14, 23.0),
        back: (10, 1, 24.0),
    },
    SummerTime {
        year: 1917,
        forward: (3, 24, 23.0),
        back: (10, 7, 24.0),
    },
    SummerTime {
        year: 1918,
        forward: (3, 9, 23.0),
        back: (10, 6, 24.0),
    },
];

/// Hours ahead of Greenwich at a French legal time.
pub fn utc_offset_hours(time: TimeSpec) -> f64 {
    let Some(summer) = FRENCH_SUMMER_TIME.iter().find(|s| s.year == time.year) else {
        return 0.0;
    };
    let at = (
        time.month,
        time.day,
        time.hour as f64 + time.minute as f64 / 60.0,
    );
    let before =
        |(m, d, h): (u32, u32, f64)| (at.0, at.1) < (m, d) || ((at.0, at.1) == (m, d) && at.2 < h);
    if !before(summer.forward) && before(summer.back) {
        1.0
    } else {
        0.0
    }
}

/// Whether a legal time is read once on the clocks: not in the hour skipped when they go
/// forward (23 to 24 h), nor in the hour read twice when they go back (23 to 24 h).
pub fn is_unambiguous(time: TimeSpec) -> bool {
    let Some(summer) = FRENCH_SUMMER_TIME.iter().find(|s| s.year == time.year) else {
        return true;
    };
    let in_last_hour = |(m, d, h): (u32, u32, f64)| {
        (time.month, time.day) == (m, d) && (time.hour as f64) >= h - 1.0 && (time.hour as f64) < h
    };
    let skipped = (summer.forward.0, summer.forward.1, summer.forward.2 + 1.0);
    !(in_last_hour(skipped) || in_last_hour(summer.back))
}

/// A French legal time as universal time: (year, month, day, hours), the hours possibly
/// below 0 when the day starts earlier at Greenwich.
pub fn to_utc(time: TimeSpec) -> (i32, u32, u32, f64) {
    let hours = time.hour as f64 + time.minute as f64 / 60.0 - utc_offset_hours(time);
    (time.year, time.month, time.day, hours)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(month: u32, day: u32, hour: u32) -> TimeSpec {
        TimeSpec {
            year: 1917,
            month,
            day,
            hour,
            minute: 0,
        }
    }

    #[test]
    fn summer_time_is_an_hour_ahead_of_greenwich() {
        assert_eq!(utc_offset_hours(at(1, 15, 12)), 0.0);
        assert_eq!(utc_offset_hours(at(7, 31, 5)), 1.0);
        assert_eq!(utc_offset_hours(at(12, 15, 12)), 0.0);
        // The night of the change forward, either side of 23 h.
        assert_eq!(utc_offset_hours(at(3, 24, 22)), 0.0);
        assert_eq!(utc_offset_hours(at(3, 25, 1)), 1.0);
        assert_eq!(to_utc(at(7, 31, 5)), (1917, 7, 31, 4.0));
    }

    #[test]
    fn the_skipped_hour_and_the_doubled_one_are_refused() {
        assert!(!is_unambiguous(at(3, 24, 23)));
        assert!(!is_unambiguous(at(10, 7, 23)));
        assert!(is_unambiguous(at(3, 24, 22)));
        assert!(is_unambiguous(at(10, 7, 22)));
        assert!(is_unambiguous(at(10, 8, 0)));
    }
}
