//! Inline piston engine with a fixed crankcase, naturally aspirated (skill
//! `simulation-vol`, `modele-aero.md`, section « Moteurs »).
//!
//! Brake power P_b = (1 + k) P₀(ω) σ θ − k P₀(ω), where P₀(ω) is the sea-level,
//! full-throttle brake power curve, σ the density ratio, θ the effective throttle and
//! k the friction fraction (friction power at full power, as a fraction of brake power).
//! Indicated power scales with the charge density and the throttle, friction does not.
//! At full throttle this is the Gagg-Farrar law, P_b / P₀ = σ − k (1 − σ), with
//! k = 1/7.55 in its usual form; the research sheet of each engine gives k (for the
//! Hispano-Suiza 8A, `docs/recherche/hispano-suiza-8a.md`).
//!
//! Throttle: θ = idle + (1 − idle) × lever (E:, linear). Outside the measured curve the
//! torque P₀ / ω is held constant (E:). Below the stall speed the engine stops firing
//! (it restarts when windmilled fast enough); friction then fades linearly to zero at
//! rest, which avoids a torque jump.

/// One point of the sea-level, full-throttle brake power curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnginePoint {
    /// Shaft speed relative to the crankcase, rad/s.
    pub speed: f64,
    /// Brake power, W.
    pub power: f64,
}

/// Why an engine was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineError {
    TooFewPoints,
    NonFinite,
    NotStrictlyIncreasing,
    NonPositive,
    BadFrictionFraction,
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
    friction_fraction: f64,
    idle_throttle: f64,
    stall_speed: f64,
}

impl InlineEngine {
    pub fn new(
        curve: Vec<EnginePoint>,
        friction_fraction: f64,
        idle_throttle: f64,
        stall_speed: f64,
    ) -> Result<Self, EngineError> {
        if curve.len() < 2 {
            return Err(EngineError::TooFewPoints);
        }
        if curve
            .iter()
            .any(|p| !(p.speed.is_finite() && p.power.is_finite()))
        {
            return Err(EngineError::NonFinite);
        }
        if curve.windows(2).any(|w| w[1].speed <= w[0].speed) {
            return Err(EngineError::NotStrictlyIncreasing);
        }
        if curve.iter().any(|p| p.speed <= 0.0 || p.power <= 0.0) {
            return Err(EngineError::NonPositive);
        }
        if !(friction_fraction.is_finite() && friction_fraction >= 0.0) {
            return Err(EngineError::BadFrictionFraction);
        }
        if !(0.0..1.0).contains(&idle_throttle) {
            return Err(EngineError::BadIdleThrottle);
        }
        if !(stall_speed > 0.0 && 2.0 * stall_speed <= curve[0].speed) {
            return Err(EngineError::BadStallSpeed);
        }
        Ok(Self {
            curve,
            friction_fraction,
            idle_throttle,
            stall_speed,
        })
    }

    /// Sea-level, full-throttle brake torque P₀(ω) / ω, held constant outside the curve.
    fn reference_torque(&self, speed: f64) -> f64 {
        let last = self.curve.len() - 1;
        let s = speed.clamp(self.curve[0].speed, self.curve[last].speed);
        let i = self.curve.partition_point(|p| p.speed <= s).clamp(1, last);
        let (a, b) = (self.curve[i - 1], self.curve[i]);
        let t = (s - a.speed) / (b.speed - a.speed);
        (a.power + t * (b.power - a.power)) / s
    }

    /// Full-throttle brake power at density ratio σ, relative to sea level.
    pub fn altitude_factor(&self, density_ratio: f64) -> f64 {
        let k = self.friction_fraction;
        (1.0 + k) * density_ratio - k
    }

    /// Torques at a shaft speed relative to the crankcase (rad/s), throttle lever
    /// position (0 to 1) and density ratio σ = ρ / ρ₀.
    pub fn torques(&self, shaft_speed: f64, throttle: f64, density_ratio: f64) -> EngineTorques {
        let k = self.friction_fraction;
        let reference = self.reference_torque(shaft_speed.abs());
        let theta = self.idle_throttle + (1.0 - self.idle_throttle) * throttle.clamp(0.0, 1.0);
        let firing = smoothstep(shaft_speed / self.stall_speed - 1.0);
        EngineTorques {
            combustion: (1.0 + k) * reference * density_ratio.max(0.0) * theta * firing,
            friction: k * reference * (shaft_speed / self.stall_speed).clamp(-1.0, 1.0),
        }
    }

    /// Brake power delivered to the propeller, W.
    pub fn brake_power(&self, shaft_speed: f64, throttle: f64, density_ratio: f64) -> f64 {
        let t = self.torques(shaft_speed, throttle, density_ratio);
        (t.combustion - t.friction) * shaft_speed
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

    /// Synthetic engine (not historical data): about 110 kW near 2 100 rpm.
    pub(crate) fn engine() -> InlineEngine {
        let curve = [(120.0, 75e3), (160.0, 95e3), (200.0, 108e3), (230.0, 112e3)]
            .map(|(speed, power)| EnginePoint { speed, power })
            .to_vec();
        InlineEngine::new(curve, 0.1, 0.05, 30.0).unwrap()
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
        // Constant torque outside the curve.
        assert!(close(
            e.brake_power(100.0, 1.0, 1.0),
            75e3 * 100.0 / 120.0,
            1e-12
        ));
        assert!(close(
            e.brake_power(260.0, 1.0, 1.0),
            112e3 * 260.0 / 230.0,
            1e-12
        ));
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
        let e = InlineEngine::new(curve, 1.0 / 7.55, 0.1, 30.0).unwrap();
        for sigma in [1.0, 0.9, 0.7, 0.5, 0.3] {
            let ratio = e.brake_power(200.0, 1.0, sigma) / e.brake_power(200.0, 1.0, 1.0);
            assert!(close(ratio, sigma - (1.0 - sigma) / 7.55, 1e-12), "{sigma}");
            assert!(close(ratio, e.altitude_factor(sigma), 1e-12));
        }
    }

    #[test]
    fn throttle_scales_indicated_power_and_idle_brakes_the_shaft() {
        let e = engine();
        let full = e.torques(200.0, 1.0, 1.0);
        let half = e.torques(200.0, 0.5, 1.0);
        assert!(close(half.combustion / full.combustion, 0.525, 1e-12));
        assert_eq!(half.friction, full.friction);
        // Idle: (1 + k) θ_idle = 1.1 × 0.05 < k = 0.1, the engine brakes the shaft.
        assert!(e.brake_power(200.0, 0.0, 1.0) < 0.0);
        assert!(e.brake_power(200.0, -1.0, 1.0) == e.brake_power(200.0, 0.0, 1.0));
    }

    #[test]
    fn engine_stops_firing_below_its_stall_speed() {
        let e = engine();
        for speed in [-50.0, -1.0, 0.0, 15.0, 30.0] {
            assert_eq!(e.torques(speed, 1.0, 1.0).combustion, 0.0, "{speed}");
        }
        assert!(e.torques(45.0, 1.0, 1.0).combustion > 0.0);
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
            assert!((t.combustion - previous.combustion).abs() < 0.5);
            assert!((t.friction - previous.friction).abs() < 0.5);
            previous = t;
        }
    }

    #[test]
    fn rejects_unphysical_engines() {
        let point = |speed, power| EnginePoint { speed, power };
        let good = vec![point(120.0, 75e3), point(200.0, 108e3)];
        let new = |c: Vec<EnginePoint>, k, idle, stall| InlineEngine::new(c, k, idle, stall);
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
            Err(EngineError::BadFrictionFraction)
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
