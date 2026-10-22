//! Inline piston engine with a fixed crankcase, naturally aspirated (skill
//! `simulation-vol`, `modele-aero.md`, section « Moteurs »).
//!
//! Two sea-level curves against the shaft speed ω: full-throttle brake power P₀(ω) and
//! friction power P_f(ω) (friction and pumping, measured by motoring the engine). The
//! indicated power at full throttle, P₀ + P_f, scales with the density ratio σ; friction
//! does not. At full throttle, P_b = σ (P₀ + P_f) − P_f: with P_f = k P₀ this is the
//! Gagg-Farrar law P_b / P₀ = σ − k (1 − σ), k = 1/7.55 in its usual form; the research
//! sheet of each engine gives its friction (Hispano-Suiza 8A: measured, NACA Report 46,
//! `docs/recherche/hispano-suiza-8a.md`).
//!
//! Throttle: with the throttle closed, the carburettor limits the air flow, so the
//! indicated power is about constant and the torque falls as 1/ω, which gives a stable
//! idle and engine braking in a dive (aerodynamicist review of 2026-10-21). Indicated
//! power P_i = σ min(P₀(ω) + P_f(ω), θ P_i,max), with θ = idle + (1 − idle) × lever
//! (E:, linear) and P_i,max the largest full-throttle indicated power of the curves. The
//! corner of the min is rounded (C¹, ±5 %, E:) so that the trim can be solved; above the
//! corner the throttle has no effect, which is real (manifold at ambient pressure).
//!
//! Outside the measured curves (E:), the torque is held constant below the first point and
//! the power above the last one, so that full throttle stays the largest power. Below the stall
//! speed the engine stops firing (it restarts when windmilled fast enough), with a C¹
//! ramp up to twice the stall speed; friction fades linearly to zero at rest, which
//! avoids a torque jump.

/// One point of a sea-level power curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnginePoint {
    /// Shaft speed relative to the crankcase, rad/s.
    pub speed: f64,
    /// Power, W.
    pub power: f64,
}

/// Why an engine was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineError {
    TooFewPoints,
    NonFinite,
    NotStrictlyIncreasing,
    NonPositive,
    /// Friction curve: fewer than two points, not increasing in speed, or negative.
    BadFriction,
    BadIdleThrottle,
    /// The stall speed must be positive and the engine fully firing (twice the stall
    /// speed) before the start of the curve.
    BadStallSpeed,
}

/// Engine torques on the crankshaft, N·m: the `engine` and `friction` terms of
/// [`crate::rigid_body::RotorTorques`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EngineTorques {
    /// Indicated (combustion) torque.
    pub combustion: f64,
    /// Friction and pumping torque, positive when it opposes a positive rotation.
    pub friction: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineEngine {
    curve: Vec<EnginePoint>,
    friction: Vec<EnginePoint>,
    idle_throttle: f64,
    stall_speed: f64,
    /// Largest full-throttle indicated power at sea level, W.
    max_indicated: f64,
}

/// Torque P(ω) / ω of a power curve: linear in power between points, constant torque
/// below the first point, constant power above the last one.
fn torque_from(points: &[EnginePoint], speed: f64) -> f64 {
    let last = points.len() - 1;
    let s = speed.clamp(points[0].speed, points[last].speed);
    let i = points.partition_point(|p| p.speed <= s).clamp(1, last);
    let (a, b) = (points[i - 1], points[i]);
    let t = (s - a.speed) / (b.speed - a.speed);
    let power = a.power + t * (b.power - a.power);
    if speed > points[last].speed {
        power / speed
    } else {
        power / s
    }
}

impl InlineEngine {
    pub fn new(
        curve: Vec<EnginePoint>,
        friction: Vec<EnginePoint>,
        idle_throttle: f64,
        stall_speed: f64,
    ) -> Result<Self, EngineError> {
        let finite = |p: &EnginePoint| p.speed.is_finite() && p.power.is_finite();
        if curve.len() < 2 {
            return Err(EngineError::TooFewPoints);
        }
        if !curve.iter().all(finite) {
            return Err(EngineError::NonFinite);
        }
        if curve.windows(2).any(|w| w[1].speed <= w[0].speed) {
            return Err(EngineError::NotStrictlyIncreasing);
        }
        if curve.iter().any(|p| p.speed <= 0.0 || p.power <= 0.0) {
            return Err(EngineError::NonPositive);
        }
        if friction.len() < 2
            || !friction.iter().all(finite)
            || friction.windows(2).any(|w| w[1].speed <= w[0].speed)
            || friction.iter().any(|p| p.speed <= 0.0 || p.power < 0.0)
        {
            return Err(EngineError::BadFriction);
        }
        if !(0.0..1.0).contains(&idle_throttle) {
            return Err(EngineError::BadIdleThrottle);
        }
        if !(stall_speed > 0.0 && 2.0 * stall_speed <= curve[0].speed) {
            return Err(EngineError::BadStallSpeed);
        }
        let max_indicated = curve
            .iter()
            .map(|p| p.power + torque_from(&friction, p.speed) * p.speed)
            .fold(0.0, f64::max);
        Ok(Self {
            curve,
            friction,
            idle_throttle,
            stall_speed,
            max_indicated,
        })
    }

    /// Torques at a shaft speed relative to the crankcase (rad/s), throttle lever
    /// position (0 to 1) and density ratio σ = ρ / ρ₀.
    pub fn torques(&self, shaft_speed: f64, throttle: f64, density_ratio: f64) -> EngineTorques {
        let speed = shaft_speed.abs();
        let brake = torque_from(&self.curve, speed);
        let friction = torque_from(&self.friction, speed);
        let theta = self.idle_throttle + (1.0 - self.idle_throttle) * throttle.clamp(0.0, 1.0);
        let firing = smoothstep(shaft_speed / self.stall_speed - 1.0);
        let combustion = if firing > 0.0 {
            let full = (brake + friction) * speed;
            let available = theta * (1.0 + THROTTLE_BLEND) * self.max_indicated;
            let indicated = density_ratio.max(0.0) * full * smooth_min_ratio(available / full);
            indicated / speed * firing
        } else {
            0.0
        };
        EngineTorques {
            combustion,
            friction: friction * (shaft_speed / self.stall_speed).clamp(-1.0, 1.0),
        }
    }

    /// Brake power delivered to the propeller, W.
    pub fn brake_power(&self, shaft_speed: f64, throttle: f64, density_ratio: f64) -> f64 {
        let t = self.torques(shaft_speed, throttle, density_ratio);
        (t.combustion - t.friction) * shaft_speed
    }
}

/// Half-width of the C¹ blend between the throttle-limited and the full-throttle
/// regimes, as a fraction of the power (E:). The throttle's available power is raised by
/// as much, so that full throttle still gives exactly the full-throttle curve.
const THROTTLE_BLEND: f64 = 0.05;

/// min(x, 1) with its corner rounded over 1 ± [`THROTTLE_BLEND`] (C¹): the trim solver
/// of `fdm_lab` cannot work across a corner.
fn smooth_min_ratio(x: f64) -> f64 {
    let w = THROTTLE_BLEND;
    if x <= 1.0 - w {
        x
    } else if x >= 1.0 + w {
        1.0
    } else {
        x - (x - (1.0 - w)).powi(2) / (4.0 * w)
    }
}

/// 0 below 0, 1 above 1, C¹ in between.
fn smoothstep(x: f64) -> f64 {
    let s = x.clamp(0.0, 1.0);
    s * s * (3.0 - 2.0 * s)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Synthetic engine (not historical data): about 110 kW near 2 100 rpm, friction a
    /// tenth of the brake power.
    pub(crate) fn engine() -> InlineEngine {
        let curve = [(120.0, 75e3), (160.0, 95e3), (200.0, 108e3), (230.0, 112e3)]
            .map(|(speed, power)| EnginePoint { speed, power })
            .to_vec();
        InlineEngine::new(curve.clone(), scaled(&curve, 0.1), 0.08, 20.0).unwrap()
    }

    pub(crate) fn scaled(curve: &[EnginePoint], k: f64) -> Vec<EnginePoint> {
        curve
            .iter()
            .map(|p| EnginePoint {
                speed: p.speed,
                power: k * p.power,
            })
            .collect()
    }

    fn close(a: f64, b: f64, rel: f64) -> bool {
        (a - b).abs() <= rel * a.abs().max(b.abs()).max(1e-12)
    }

    #[test]
    fn full_throttle_at_sea_level_follows_the_power_curve() {
        let e = engine();
        for (speed, power) in [(120.0, 75e3), (140.0, 85e3), (200.0, 108e3), (230.0, 112e3)] {
            assert!(
                close(e.brake_power(speed, 1.0, 1.0), power, 1e-12),
                "{speed}"
            );
        }
        // Constant torque below the curve, constant power above it.
        assert!(close(
            e.brake_power(100.0, 1.0, 1.0),
            75e3 * 100.0 / 120.0,
            1e-12
        ));
        assert!(close(e.brake_power(260.0, 1.0, 1.0), 112e3, 1e-12));
    }

    #[test]
    fn altitude_law_is_gagg_farrar_at_full_throttle() {
        let curve = vec![
            EnginePoint {
                speed: 150.0,
                power: 90e3,
            },
            EnginePoint {
                speed: 220.0,
                power: 110e3,
            },
        ];
        let e = InlineEngine::new(curve.clone(), scaled(&curve, 1.0 / 7.55), 0.1, 30.0).unwrap();
        for sigma in [1.0, 0.9, 0.7, 0.5, 0.3] {
            let ratio = e.brake_power(200.0, 1.0, sigma) / e.brake_power(200.0, 1.0, 1.0);
            assert!(close(ratio, sigma - (1.0 - sigma) / 7.55, 1e-12), "{sigma}");
        }
    }

    #[test]
    fn throttle_scales_indicated_power_and_idle_brakes_the_shaft() {
        let e = engine();
        let full = e.torques(200.0, 1.0, 1.0);
        let half = e.torques(200.0, 0.5, 1.0);
        // θ = 0.08 + 0.92 × 0.5 of the largest indicated power, 1.1 × 112 kW.
        let throttled = 0.54 * 1.05 * 1.1 * 112e3 / 200.0;
        assert!(close(half.combustion, throttled, 1e-12));
        assert!(close(full.combustion, 1.1 * 108e3 / 200.0, 1e-12));
        assert_eq!(half.friction, full.friction);
        // Throttled, the indicated power is constant: the torque falls as 1/ω.
        let slow = e.torques(150.0, 0.5, 1.0);
        assert!(close(
            slow.combustion * 150.0,
            half.combustion * 200.0,
            1e-12
        ));
        // Idle: 0.08 × 123.2 kW indicated, less than the 10.8 kW of friction at 200 rad/s.
        assert!(e.brake_power(200.0, 0.0, 1.0) < 0.0);
        assert!(e.brake_power(200.0, -1.0, 1.0) == e.brake_power(200.0, 0.0, 1.0));
    }

    #[test]
    fn engine_stops_firing_below_its_stall_speed() {
        let e = engine();
        for speed in [-50.0, -1.0, 0.0, 10.0, 20.0] {
            assert_eq!(e.torques(speed, 1.0, 1.0).combustion, 0.0, "{speed}");
        }
        assert!(e.torques(30.0, 1.0, 1.0).combustion > 0.0);
        assert_eq!(e.torques(0.0, 1.0, 1.0).friction, 0.0);
        // Friction always opposes the rotation.
        assert!(e.torques(10.0, 1.0, 1.0).friction > 0.0);
        assert!(e.torques(-10.0, 1.0, 1.0).friction < 0.0);
        // Stratosphere-like density ratio, out-of-range inputs: finite.
        let t = e.torques(400.0, 2.0, -0.5);
        assert!(t.combustion == 0.0 && t.friction.is_finite());
    }

    #[test]
    fn torques_are_continuous_in_shaft_speed() {
        let e = engine();
        let mut previous = e.torques(-100.0, 1.0, 1.0);
        for i in 1..=40_000 {
            let t = e.torques(-100.0 + f64::from(i) * 0.01, 1.0, 1.0);
            // Steepest at the firing ramp: 1.5 / ω_stall × 1.1 × 625 N·m per rad/s.
            assert!((t.combustion - previous.combustion).abs() < 0.6);
            assert!((t.friction - previous.friction).abs() < 0.5);
            previous = t;
        }
    }

    #[test]
    fn rejects_unphysical_engines() {
        let point = |speed, power| EnginePoint { speed, power };
        let good = vec![point(120.0, 75e3), point(200.0, 108e3)];
        let new = |c: Vec<EnginePoint>, k, idle, stall| {
            let friction = scaled(&c, k);
            InlineEngine::new(c, friction, idle, stall)
        };
        assert_eq!(
            new(good[..1].to_vec(), 0.1, 0.1, 30.0),
            Err(EngineError::TooFewPoints)
        );
        assert_eq!(
            new(
                vec![point(120.0, f64::NAN), point(200.0, 1.0)],
                0.1,
                0.1,
                30.0
            ),
            Err(EngineError::NonFinite)
        );
        assert_eq!(
            new(vec![point(200.0, 1.0), point(120.0, 1.0)], 0.1, 0.1, 30.0),
            Err(EngineError::NotStrictlyIncreasing)
        );
        assert_eq!(
            new(vec![point(120.0, 0.0), point(200.0, 1.0)], 0.1, 0.1, 30.0),
            Err(EngineError::NonPositive)
        );
        assert_eq!(
            new(good.clone(), -0.1, 0.1, 30.0),
            Err(EngineError::BadFriction)
        );
        assert_eq!(
            InlineEngine::new(good.clone(), good[..1].to_vec(), 0.1, 30.0),
            Err(EngineError::BadFriction)
        );
        assert_eq!(
            new(good.clone(), 0.1, 1.0, 30.0),
            Err(EngineError::BadIdleThrottle)
        );
        assert_eq!(
            new(good.clone(), 0.1, 0.1, 61.0),
            Err(EngineError::BadStallSpeed)
        );
        assert_eq!(new(good, 0.1, 0.1, 0.0), Err(EngineError::BadStallSpeed));
    }
}
