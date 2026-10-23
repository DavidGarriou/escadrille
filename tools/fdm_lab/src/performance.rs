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

/// Steady glide at idle at a given speed.
pub fn glide(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    speed: f64,
) -> Result<Trim, PerformanceError> {
    let mut guess = Flight::guess(altitude, speed, 0.0);
    guess.flight_path = -0.12;
    Ok(solve_robust(aircraft, env, guess, PATH_AT_SPEED)?)
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
    /// The optimum lies at the edge of the searched interval (45 to 95 % of the maximum
    /// speed): the true optimum may be outside it.
    pub at_edge: bool,
}

/// Lower and upper bounds of the speed of best climb, fractions of the maximum speed.
const CLIMB_SPEED_RANGE: (f64, f64) = (0.45, 0.95);

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
    let (bottom, top) = (CLIMB_SPEED_RANGE.0 * vmax, CLIMB_SPEED_RANGE.1 * vmax);
    let (mut lo, mut hi) = (bottom, top);
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
    let at_edge = speed - bottom < 2.0 * SPEED_TOLERANCE || top - speed < 2.0 * SPEED_TOLERANCE;
    Ok(BestClimb {
        speed,
        rate,
        trim,
        at_edge,
    })
}

/// ∫ dh / rate(h) from `from` to `to`, trapezoids on 1 / rate with steps of at most
/// `step`; the rate must stay positive.
pub fn integrate_climb(
    rate: impl Fn(f64) -> Result<f64, PerformanceError>,
    from: f64,
    to: f64,
    step: f64,
) -> Result<f64, PerformanceError> {
    let steps = ((to - from) / step).ceil().max(1.0) as u32;
    let dh = (to - from) / f64::from(steps);
    let inverse = |h: f64| {
        let r = rate(h)?;
        if r > 0.0 {
            Ok(1.0 / r)
        } else {
            Err(PerformanceError::NoClimb { altitude: h })
        }
    };
    let mut previous = inverse(from)?;
    let mut time = 0.0;
    for i in 1..=steps {
        let next = inverse(from + f64::from(i) * dh)?;
        time += 0.5 * (previous + next) * dh;
        previous = next;
    }
    Ok(time)
}

/// Time to climb from `from` to `to` at the best rate of climb, s.
pub fn time_to_climb(
    aircraft: &Aircraft,
    env: &dyn Environment,
    from: f64,
    to: f64,
) -> Result<f64, PerformanceError> {
    integrate_climb(
        |h| best_climb(aircraft, env, h).map(|b| b.rate),
        from,
        to,
        CLIMB_STEP,
    )
}

/// A ceiling and how it was found.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ceiling {
    pub altitude: f64,
    /// Found by extrapolating the last rates of climb, the steady climb being no longer
    /// solvable near the ceiling (as the period's "theoretical ceiling").
    pub extrapolated: bool,
}

/// Altitude at which the best rate of climb falls to `rate` (m/s): 100 ft/min for the
/// service ceiling of the period, 0 for the theoretical (absolute) ceiling. Climbing in
/// steps of [`CLIMB_STEP`]; between two solved steps the crossing is found by
/// bisection; if the climb can no longer be solved before the crossing, the last two
/// rates are extrapolated linearly (the period's method), and the result says so.
pub fn ceiling(
    aircraft: &Aircraft,
    env: &dyn Environment,
    rate: f64,
) -> Result<Ceiling, PerformanceError> {
    let excess = |h: f64| best_climb(aircraft, env, h).map(|b| b.rate - rate);
    let mut points: Vec<(f64, f64)> = Vec::new();
    let mut h = 0.0;
    loop {
        match excess(h) {
            Ok(e) if e > 0.0 => points.push((h, e)),
            Ok(_) => {
                let Some(&(mut lo, _)) = points.last() else {
                    return Err(PerformanceError::NoClimb { altitude: h });
                };
                let mut hi = h;
                while hi - lo > CEILING_TOLERANCE {
                    let mid = 0.5 * (lo + hi);
                    match excess(mid) {
                        Ok(e) if e > 0.0 => lo = mid,
                        Ok(_) => hi = mid,
                        Err(_) => break,
                    }
                }
                return Ok(Ceiling {
                    altitude: 0.5 * (lo + hi),
                    extrapolated: false,
                });
            }
            Err(error) => {
                let n = points.len();
                if n < 2 {
                    return Err(error);
                }
                let ((h1, e1), (h2, e2)) = (points[n - 2], points[n - 1]);
                if e2 >= e1 {
                    return Err(error);
                }
                return Ok(Ceiling {
                    altitude: h2 + e2 * (h2 - h1) / (e1 - e2),
                    extrapolated: true,
                });
            }
        }
        h += CLIMB_STEP;
        if h > 15_000.0 {
            return Err(PerformanceError::NoClimb { altitude: h });
        }
    }
}

/// Maximum level speed and best climb at one altitude (for the plots).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvelopePoint {
    pub altitude: f64,
    pub max_speed: Trim,
    pub best_climb: BestClimb,
}

/// The envelope from sea level upwards in steps of `step` metres, as long as both the
/// maximum speed and a positive climb can be solved.
pub fn envelope(aircraft: &Aircraft, env: &dyn Environment, step: f64) -> Vec<EnvelopePoint> {
    let mut points = Vec::new();
    let mut altitude = 0.0;
    while altitude <= 15_000.0 {
        let (Ok(max_speed), Ok(best_climb)) = (
            max_level_speed(aircraft, env, altitude),
            best_climb(aircraft, env, altitude),
        ) else {
            break;
        };
        if best_climb.rate <= 0.0 {
            break;
        }
        points.push(EnvelopePoint {
            altitude,
            max_speed,
            best_climb,
        });
        altitude += step;
    }
    points
}

/// Lowest steady speed at a throttle setting.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stall {
    pub speed: f64,
    pub alpha: f64,
    /// The elevator ran out of travel before the wing stalled: the speed is that of the
    /// last steady glide the pilot can hold.
    pub elevator_limited: bool,
    pub trim: Trim,
}

/// Starting speeds of the stall sweep, m/s.
const STALL_START_SPEEDS: [f64; 5] = [40.0, 30.0, 50.0, 60.0, 25.0];

/// Stall speed at idle: the lowest speed of a steady glide, sweeping the angle of attack
/// up to the stall (or to the end of the elevator travel).
pub fn stall_speed(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
) -> Result<Stall, PerformanceError> {
    stall_speed_at(aircraft, env, altitude, 0.0)
}

/// Stall speed at a throttle setting: the lowest speed of a steady straight flight
/// (a glide at idle, a climb at full throttle), sweeping the angle of attack up to the
/// stall (or to the end of the elevator travel).
pub fn stall_speed_at(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    throttle: f64,
) -> Result<Stall, PerformanceError> {
    let mut guess = Flight::guess(altitude, 40.0, throttle);
    guess.flight_path = if throttle > 0.5 { 0.1 } else { -0.1 };
    let mut best: Option<Stall> = None;
    let mut alpha = 0.05;
    while alpha < 0.45 {
        guess.alpha = alpha;
        // Until a first steady flight is found, try several starting speeds, then the
        // next incidence; past it, each solution starts the next one.
        let trim = if best.is_some() {
            solve_robust(aircraft, env, guess, AT_ALPHA).ok()
        } else {
            STALL_START_SPEEDS.iter().find_map(|&speed| {
                solve_robust(aircraft, env, Flight { speed, ..guess }, AT_ALPHA).ok()
            })
        };
        let Some(trim) = trim else {
            if best.is_some() {
                break;
            }
            alpha += 0.005;
            continue;
        };
        if trim.commands(aircraft).pitch.abs() > 1.0 {
            if let Some(b) = best.as_mut() {
                b.elevator_limited = true;
            }
            break;
        }
        let speed = trim.flight.speed;
        if best.is_some_and(|b| speed >= b.speed) {
            break;
        }
        best = Some(Stall {
            speed,
            alpha,
            elevator_limited: false,
            trim,
        });
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
        let h = ceiling(&a, &STILL_AIR, target).unwrap().altitude;
        let rate = best_climb(&a, &STILL_AIR, h).unwrap().rate;
        assert!((rate - target).abs() < 0.01, "ceiling {h} m, rate {rate}");
    }

    #[test]
    fn power_lowers_the_stall_speed() {
        // Thrust carries part of the weight and the slipstream adds lift.
        let a = biplane();
        let idle = stall_speed(&a, &STILL_AIR, 500.0).unwrap();
        let full = stall_speed_at(&a, &STILL_AIR, 500.0, 1.0).unwrap();
        assert!(full.speed < idle.speed, "{full:?} {idle:?}");
        assert!(
            full.trim.flight.climb_rate() > 0.0,
            "a climb at full throttle"
        );
    }

    #[test]
    fn stall_speed_is_below_the_maximum_speed() {
        let a = biplane();
        let stall = stall_speed(&a, &STILL_AIR, 500.0).unwrap();
        let vmax = max_level_speed(&a, &STILL_AIR, 500.0).unwrap().flight.speed;
        assert!(stall.speed > 10.0 && stall.speed < 0.7 * vmax, "{stall:?}");
        assert!(stall.trim.flight.climb_rate() < 0.0, "a glide at idle");
        // The sweep stops at the minimum: slightly lower or higher angles fly faster.
        if !stall.elevator_limited {
            for da in [-0.005, 0.005] {
                let mut g = stall.trim.flight;
                g.alpha = stall.alpha + da;
                if let Ok(t) = solve_robust(&a, &STILL_AIR, g, AT_ALPHA) {
                    assert!(t.flight.speed >= stall.speed - 1e-9, "{da}: {t:?}");
                }
            }
        }
    }
}

#[cfg(test)]
mod integration {
    use super::*;

    #[test]
    fn climb_time_integration_is_second_order() {
        // Linear rate 6 − 0.001 h m/s: exact time −1000 ln((6 − 0.001 h₁) / (6 − 0.001 h₀)).
        let rate = |h: f64| Ok(6.0 - 0.001 * h);
        let exact = -1_000.0 * ((6.0 - 3.0) / 6.0_f64).ln();
        let trapezoids = integrate_climb(rate, 0.0, 3_000.0, 250.0).unwrap();
        assert!(
            (trapezoids - exact).abs() < 2e-3 * exact,
            "{trapezoids} vs {exact}"
        );
        // Halving the step divides the error by about four.
        let finer = integrate_climb(rate, 0.0, 3_000.0, 125.0).unwrap();
        let ratio = (trapezoids - exact) / (finer - exact);
        assert!((ratio - 4.0).abs() < 0.2, "{ratio}");
        assert!(matches!(
            integrate_climb(|h| Ok(1.0 - 0.001 * h), 0.0, 2_000.0, 250.0),
            Err(PerformanceError::NoClimb { .. })
        ));
    }
}
