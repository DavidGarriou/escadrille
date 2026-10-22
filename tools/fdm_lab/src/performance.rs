//! Performance of an aircraft from steady flights (M1 plan, step 6): maximum level
//! speed, best rate of climb, time to climb, ceiling and stall speed, the quantities of
//! the period's official trials (`essais.ron`).

use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::environment::Environment;

use crate::trim::{
    AT_ALPHA, Flight, PATH_AT_SPEED, SPEED_AT_THROTTLE, Trim, TrimError, solve_robust,
};

/// Golden-section search tolerance on the speed of best climb, m/s.
const SPEED_TOLERANCE: f64 = 0.1;

/// Altitude step of the time-to-climb integration, m.
const CLIMB_STEP: f64 = 250.0;

/// Tolerance of the ceiling search, m.
const CEILING_TOLERANCE: f64 = 2.0;

#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceError {
    Trim(TrimError),
    /// The aircraft cannot climb at this altitude.
    NoClimb {
        altitude: f64,
    },
    /// No stall found in the swept range of angle of attack.
    NoStall,
}

impl From<TrimError> for PerformanceError {
    fn from(e: TrimError) -> Self {
        Self::Trim(e)
    }
}

/// Maximum level speed at full throttle (the high-speed solution).
pub fn max_level_speed(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
) -> Result<Trim, PerformanceError> {
    // Start fast, so as to land on the high-speed side of the power curve.
    let guess = Flight::guess(altitude, 80.0, 1.0);
    Ok(solve_robust(aircraft, env, guess, SPEED_AT_THROTTLE)?)
}

/// Steady climb at full throttle at a given speed.
pub fn climb(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    speed: f64,
) -> Result<Trim, PerformanceError> {
    let mut guess = Flight::guess(altitude, speed, 1.0);
    guess.flight_path = 0.05;
    Ok(solve_robust(aircraft, env, guess, PATH_AT_SPEED)?)
}

/// Best rate of climb at full throttle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BestClimb {
    pub speed: f64,
    /// m/s.
    pub rate: f64,
    pub trim: Trim,
}

/// Best rate of climb at an altitude: golden-section search on the speed, between 45 %
/// and 95 % of the maximum level speed.
pub fn best_climb(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
) -> Result<BestClimb, PerformanceError> {
    let vmax = max_level_speed(aircraft, env, altitude)?.flight.speed;
    let rate =
        |speed: f64| climb(aircraft, env, altitude, speed).map(|t| (t.flight.climb_rate(), t));
    let ratio = 0.5 * (5.0_f64.sqrt() - 1.0);
    let (mut lo, mut hi) = (0.45 * vmax, 0.95 * vmax);
    let mut x1 = hi - ratio * (hi - lo);
    let mut x2 = lo + ratio * (hi - lo);
    let (mut f1, mut f2) = (rate(x1)?, rate(x2)?);
    while hi - lo > SPEED_TOLERANCE {
        if f1.0 > f2.0 {
            hi = x2;
            (x2, f2) = (x1, f1);
            x1 = hi - ratio * (hi - lo);
            f1 = rate(x1)?;
        } else {
            lo = x1;
            (x1, f1) = (x2, f2);
            x2 = lo + ratio * (hi - lo);
            f2 = rate(x2)?;
        }
    }
    let (speed, (rate, trim)) = if f1.0 > f2.0 { (x1, f1) } else { (x2, f2) };
    Ok(BestClimb { speed, rate, trim })
}

/// Time to climb from `from` to `to` at the best rate of climb, s (trapezoids on 1 / rate).
pub fn time_to_climb(
    aircraft: &Aircraft,
    env: &dyn Environment,
    from: f64,
    to: f64,
) -> Result<f64, PerformanceError> {
    let steps = ((to - from) / CLIMB_STEP).ceil().max(1.0) as u32;
    let dh = (to - from) / f64::from(steps);
    let inverse_rate = |h: f64| {
        let rate = best_climb(aircraft, env, h)?.rate;
        if rate > 0.0 {
            Ok(1.0 / rate)
        } else {
            Err(PerformanceError::NoClimb { altitude: h })
        }
    };
    let mut previous = inverse_rate(from)?;
    let mut time = 0.0;
    for i in 1..=steps {
        let next = inverse_rate(from + f64::from(i) * dh)?;
        time += 0.5 * (previous + next) * dh;
        previous = next;
    }
    Ok(time)
}

/// Altitude at which the best rate of climb falls to `rate` (m/s): 100 ft/min for the
/// service ceiling of the period, 0 for the absolute ceiling.
pub fn ceiling(
    aircraft: &Aircraft,
    env: &dyn Environment,
    rate: f64,
) -> Result<f64, PerformanceError> {
    let excess = |h: f64| best_climb(aircraft, env, h).map_or(-1.0, |b| b.rate - rate);
    let (mut lo, mut hi) = (0.0, 12_000.0);
    if excess(lo) <= 0.0 {
        return Err(PerformanceError::NoClimb { altitude: lo });
    }
    while hi - lo > CEILING_TOLERANCE {
        let mid = 0.5 * (lo + hi);
        if excess(mid) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Ok(0.5 * (lo + hi))
}

/// Lowest steady speed, engine at idle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stall {
    pub speed: f64,
    pub alpha: f64,
    /// The elevator ran out of travel before the wing stalled.
    pub elevator_limited: bool,
    pub trim: Trim,
}

/// Stall speed at idle: the lowest speed of a steady glide, sweeping the angle of attack
/// up to the stall (or to the end of the elevator travel).
pub fn stall_speed(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
) -> Result<Stall, PerformanceError> {
    let mut guess = Flight::guess(altitude, 40.0, 0.0);
    guess.flight_path = -0.1;
    let mut best: Option<Stall> = None;
    let mut alpha = 0.05;
    while alpha < 0.45 {
        guess.alpha = alpha;
        let Ok(trim) = solve_robust(aircraft, env, guess, AT_ALPHA) else {
            break;
        };
        let elevator_limited = trim.commands(aircraft).pitch.abs() > 1.0;
        let speed = trim.flight.speed;
        match best {
            Some(b) if speed >= b.speed => break,
            _ => {}
        }
        best = Some(Stall {
            speed,
            alpha,
            elevator_limited,
            trim,
        });
        if elevator_limited {
            break;
        }
        guess = trim.flight;
        alpha += 0.005;
    }
    best.ok_or(PerformanceError::NoStall)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};

    #[test]
    fn maximum_speed_falls_with_altitude() {
        let a = biplane();
        let low = max_level_speed(&a, &STILL_AIR, 0.0).unwrap().flight.speed;
        let high = max_level_speed(&a, &STILL_AIR, 3_000.0)
            .unwrap()
            .flight
            .speed;
        assert!(low > high && high > 0.0, "{low} {high}");
    }

    #[test]
    fn best_climb_is_a_maximum_and_falls_with_altitude() {
        let a = biplane();
        let best = best_climb(&a, &STILL_AIR, 500.0).unwrap();
        assert!(best.rate > 0.0, "{best:?}");
        for dv in [-3.0, 3.0] {
            let other = climb(&a, &STILL_AIR, 500.0, best.speed + dv).unwrap();
            assert!(other.flight.climb_rate() < best.rate);
        }
        let higher = best_climb(&a, &STILL_AIR, 2_500.0).unwrap();
        assert!(higher.rate < best.rate);
    }

    #[test]
    fn climb_times_add_up_and_the_ceiling_is_where_the_rate_runs_out() {
        let a = biplane();
        let t1 = time_to_climb(&a, &STILL_AIR, 0.0, 1_000.0).unwrap();
        let t2 = time_to_climb(&a, &STILL_AIR, 1_000.0, 2_000.0).unwrap();
        let t = time_to_climb(&a, &STILL_AIR, 0.0, 2_000.0).unwrap();
        assert!(t1 > 0.0 && t2 > t1, "{t1} {t2}");
        assert!((t - (t1 + t2)).abs() < 1e-6 * t);

        let target = 0.508; // 100 ft/min
        let h = ceiling(&a, &STILL_AIR, target).unwrap();
        let rate = best_climb(&a, &STILL_AIR, h).unwrap().rate;
        assert!((rate - target).abs() < 0.01, "ceiling {h} m, rate {rate}");
    }

    #[test]
    fn stall_speed_is_below_the_maximum_speed() {
        let a = biplane();
        let stall = stall_speed(&a, &STILL_AIR, 500.0).unwrap();
        let vmax = max_level_speed(&a, &STILL_AIR, 500.0).unwrap().flight.speed;
        assert!(stall.speed > 10.0 && stall.speed < 0.7 * vmax, "{stall:?}");
        assert!(stall.trim.flight.climb_rate() < 0.0, "a glide at idle");
    }
}
