//! Inline piston engine with a fixed crankcase, naturally aspirated (skill
//! `simulation-vol`, `modele-aero.md`, section « Moteurs »).
//!
//! Two sea-level curves against the shaft speed ω: full-throttle brake power P₀(ω) and
//! friction power P_f(ω) (friction and pumping, measured by motoring the engine). The
//! indicated power at full throttle, P₀ + P_f, scales with the charge ratio c of the
//! [`AltitudeLaw`] (the density ratio σ, or a power of the pressure ratio); friction does
//! not. At full throttle, P_b = c (P₀ + P_f) − P_f: with c = σ and P_f = k P₀ this is the
//! Gagg-Farrar law P_b / P₀ = σ − k (1 − σ), k = 1/7.55 in its usual form; the research
//! sheet of each engine gives its friction and its altitude law (Hispano-Suiza 8A:
//! NACA Report 46 and R&M 462, `docs/recherche/hispano-suiza-8a.md`).
//!
//! Throttle: the throttle is an orifice in series with a volumetric engine. Nearly closed,
//! the flow through it is choked: the indicated power is about constant and the torque
//! falls as 1/ω, which gives a stable idle and engine braking in a dive. Opening it, the
//! gain decreases continuously and vanishes only at full throttle (aerodynamicist reviews
//! of 2026-10-21 and 22; an earlier min() left a dead zone of 22 % of the lever travel).
//! With P_pg(ω) = P₀ + P_f the full-throttle indicated power, s = θ P_i,max / P_pg(ω)
//! and s₁ its value at full throttle: P_i = c P_pg(ω) g(s) / g(s₁), g(s) = s / (1 + sⁿ)^(1/n),
//! n = 2 (E:); θ = idle + (1 − idle) × lever (E:) and P_i,max the largest full-throttle
//! indicated power of the curves. Full throttle gives exactly the curves.
//!
//! Installed power: the curves are bench (dynamometer) figures, and an installed engine
//! seems to deliver a few per cent less in flight (NACA Report 301: flight power
//! coefficients 3 to 5 % above the full-scale tunnel's on a Wright-Hispano; cause not
//! identified by the source). The indicated power is multiplied by an installed fraction,
//! like a smaller charge, friction unchanged.
//!
//! Outside the measured curves (E:), the torque is held constant below the first point and
//! the power above the last one, so that full throttle stays the largest power. Below the stall
//! speed the engine stops firing (it restarts when windmilled fast enough), with a C¹
//! ramp up to twice the stall speed; friction fades linearly to zero at rest, which
//! avoids a torque jump.

use crate::atmosphere::{AtmosphereState, standard_temperature_at_pressure};
use crate::constants::{P0, RHO0};

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
    /// The exponent of a pressure law must be positive.
    BadAltitudeLaw,
    BadIdleThrottle,
    /// The stall speed must be positive and the engine fully firing (twice the stall
    /// speed) before the start of the curve.
    BadStallSpeed,
    /// The installed fraction of the indicated power must be in (0, 1].
    BadInstalledPower,
}

/// How the indicated power falls with altitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AltitudeLaw {
    /// Proportional to the density ratio σ = ρ / ρ₀ (with friction: Gagg-Farrar).
    Density,
    /// Proportional to (p / p₀)ⁿ in the standard atmosphere. The exponent applies to the
    /// indicated power, friction being taken off afterwards: a measured law of the
    /// *brake* power must be converted with the engine's friction curve (R&M 462 measures
    /// a brake power in (p / p₀)^1.04 on a 150 hp Hispano-Suiza; with the friction of
    /// NACA Report 46, the indicated exponent is about 0.93, aerodynamicist's review).
    /// On a day warmer or colder than the standard atmosphere, the charge also varies as
    /// (T_std / T)^[`TEMPERATURE_EXPONENT`], T_std being the standard temperature at the
    /// same pressure: 1 in the standard atmosphere.
    Pressure(f64),
}

/// Exponent of the temperature at constant pressure in [`AltitudeLaw::Pressure`]: on the
/// dynamometer, with carburettor intake air at about 10 and 26 °C, the brake power of a
/// Wright-Hispano « varied closely as 1/√T » at constant speed and pressure (NACA Report
/// 220, Durand and Lesley, 1926, p. 7; C: applied to the indicated power).
pub const TEMPERATURE_EXPONENT: f64 = 0.5;

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
    altitude_law: AltitudeLaw,
    /// Fraction of the bench indicated power delivered in flight.
    installed: f64,
    /// Largest full-throttle indicated power at sea level, W.
    max_indicated: f64,
}

/// Exponent n of the throttle saturation g(s) = s / (1 + sⁿ)^(1/n) (E:).
const THROTTLE_SATURATION: f64 = 2.0;

fn saturation(s: f64) -> f64 {
    s / (1.0 + s.powf(THROTTLE_SATURATION)).powf(1.0 / THROTTLE_SATURATION)
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
        altitude_law: AltitudeLaw,
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
        if let AltitudeLaw::Pressure(n) = altitude_law
            && !(n.is_finite() && n > 0.0)
        {
            return Err(EngineError::BadAltitudeLaw);
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
            altitude_law,
            installed: 1.0,
            max_indicated,
        })
    }

    /// The same engine delivering `fraction` of its bench indicated power in flight.
    pub fn with_installed_power(self, fraction: f64) -> Result<Self, EngineError> {
        if !(fraction > 0.0 && fraction <= 1.0) {
            return Err(EngineError::BadInstalledPower);
        }
        Ok(Self {
            installed: fraction,
            ..self
        })
    }

    /// Charge ratio c for the ambient air: the altitude law times the installed fraction.
    pub fn charge_ratio(&self, atmosphere: &AtmosphereState) -> f64 {
        let law = match self.altitude_law {
            AltitudeLaw::Density => atmosphere.density / RHO0,
            AltitudeLaw::Pressure(n) => {
                let temperature =
                    standard_temperature_at_pressure(atmosphere.pressure) / atmosphere.temperature;
                (atmosphere.pressure / P0).max(0.0).powf(n) * temperature.powf(TEMPERATURE_EXPONENT)
            }
        };
        self.installed * law
    }

    /// Torques at a shaft speed relative to the crankcase (rad/s), throttle lever
    /// position (0 to 1) and charge ratio ([`Self::charge_ratio`]).
    pub fn torques(&self, shaft_speed: f64, throttle: f64, charge_ratio: f64) -> EngineTorques {
        let speed = shaft_speed.abs();
        let brake = torque_from(&self.curve, speed);
        let friction = torque_from(&self.friction, speed);
        let theta = self.idle_throttle + (1.0 - self.idle_throttle) * throttle.clamp(0.0, 1.0);
        let firing = smoothstep(shaft_speed / self.stall_speed - 1.0);
        let combustion = if firing > 0.0 {
            let full = (brake + friction) * speed;
            let s = theta * self.max_indicated / full;
            let s_full = self.max_indicated / full;
            let indicated = charge_ratio.max(0.0) * full * saturation(s) / saturation(s_full);
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
    pub fn brake_power(&self, shaft_speed: f64, throttle: f64, charge_ratio: f64) -> f64 {
        let t = self.torques(shaft_speed, throttle, charge_ratio);
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

    /// Synthetic engine (not historical data): about 110 kW near 2 100 rpm, friction a
    /// tenth of the brake power.
    pub(crate) fn engine() -> InlineEngine {
        let curve = [(120.0, 75e3), (160.0, 95e3), (200.0, 108e3), (230.0, 112e3)]
            .map(|(speed, power)| EnginePoint { speed, power })
            .to_vec();
        InlineEngine::new(
            curve.clone(),
            scaled(&curve, 0.1),
            0.05,
            20.0,
            AltitudeLaw::Density,
        )
        .unwrap()
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
        let e = InlineEngine::new(
            curve.clone(),
            scaled(&curve, 1.0 / 7.55),
            0.1,
            30.0,
            AltitudeLaw::Density,
        )
        .unwrap();
        for sigma in [1.0, 0.9, 0.7, 0.5, 0.3] {
            let ratio = e.brake_power(200.0, 1.0, sigma) / e.brake_power(200.0, 1.0, 1.0);
            assert!(close(ratio, sigma - (1.0 - sigma) / 7.55, 1e-12), "{sigma}");
        }
    }

    #[test]
    fn throttle_gain_never_vanishes_and_idle_brakes_the_shaft() {
        let e = engine();
        let g = |s: f64| s / (1.0 + s * s).sqrt();
        let full = e.torques(200.0, 1.0, 1.0);
        assert!(close(full.combustion, 1.1 * 108e3 / 200.0, 1e-12));
        // Half lever: θ = 0.05 + 0.95 × 0.5 of the largest indicated power, 1.1 × 112 kW.
        let (p_full, p_max) = (1.1 * 108e3, 1.1 * 112e3);
        let s = 0.525 * p_max / p_full;
        let half = e.torques(200.0, 0.5, 1.0);
        let expected = p_full * g(s) / g(p_max / p_full) / 200.0;
        assert!(close(half.combustion, expected, 1e-12));
        assert_eq!(half.friction, full.friction);
        // No dead zone: the power rises with the lever all the way, at any speed.
        for speed in [80.0, 148.0, 200.0, 230.0] {
            let mut previous = 0.0;
            for i in 0..=20 {
                let power = e.brake_power(speed, f64::from(i) / 20.0, 1.0);
                if i > 0 {
                    assert!(power > previous, "dead zone at {speed} rad/s, lever {i}/20");
                }
                previous = power;
            }
        }
        // Nearly closed, the power hardly depends on the speed: the torque falls with it.
        let (slow, fast) = (e.torques(150.0, 0.0, 1.0), e.torques(200.0, 0.0, 1.0));
        assert!(slow.combustion > fast.combustion);
        // Idle gives less than the 10.8 kW of friction at 200 rad/s: engine braking.
        assert!(e.brake_power(200.0, 0.0, 1.0) < 0.0);
        assert!(e.brake_power(200.0, -1.0, 1.0) == e.brake_power(200.0, 0.0, 1.0));
    }

    #[test]
    fn altitude_laws_follow_density_or_a_power_of_pressure() {
        let curve = [(150.0, 90e3), (220.0, 110e3)]
            .map(|(speed, power)| EnginePoint { speed, power })
            .to_vec();
        let pressure = InlineEngine::new(
            curve.clone(),
            scaled(&curve, 0.1),
            0.1,
            30.0,
            AltitudeLaw::Pressure(1.04),
        )
        .unwrap();
        let density = InlineEngine::new(
            curve.clone(),
            scaled(&curve, 0.1),
            0.1,
            30.0,
            AltitudeLaw::Density,
        )
        .unwrap();
        for altitude in [0.0, 2_000.0, 5_000.0] {
            let air = crate::atmosphere::isa(altitude);
            assert!(close(density.charge_ratio(&air), air.density / RHO0, 1e-12));
            assert!(close(
                pressure.charge_ratio(&air),
                (air.pressure / P0).powf(1.04),
                1e-12
            ));
        }
        // R&M 462's law is the more pessimistic one at altitude (about 9 % at 5 000 m).
        let air = crate::atmosphere::isa(5_000.0);
        assert!(pressure.charge_ratio(&air) < density.charge_ratio(&air));
        let bad = InlineEngine::new(
            curve.clone(),
            scaled(&curve, 0.1),
            0.1,
            30.0,
            AltitudeLaw::Pressure(-1.0),
        );
        assert_eq!(bad, Err(EngineError::BadAltitudeLaw));
    }

    #[test]
    fn a_hot_day_lowers_the_charge_at_the_same_pressure() {
        let curve = [(150.0, 90e3), (220.0, 110e3)]
            .map(|(speed, power)| EnginePoint { speed, power })
            .to_vec();
        let engine = |law| InlineEngine::new(curve.clone(), scaled(&curve, 0.1), 0.1, 30.0, law);
        let pressure = engine(AltitudeLaw::Pressure(0.93)).unwrap();
        let density = engine(AltitudeLaw::Density).unwrap();
        let standard = crate::atmosphere::isa(2_000.0);
        let hot = AtmosphereState {
            temperature: standard.temperature + 20.0,
            density: standard.density * standard.temperature / (standard.temperature + 20.0),
            ..standard
        };
        let ratio = standard.temperature / hot.temperature;
        assert!(close(
            pressure.charge_ratio(&hot),
            pressure.charge_ratio(&standard) * ratio.sqrt(),
            1e-12
        ));
        // The density law already holds the temperature, as 1/T.
        assert!(close(
            density.charge_ratio(&hot),
            density.charge_ratio(&standard) * ratio,
            1e-12
        ));
    }

    #[test]
    fn an_installed_engine_loses_indicated_power_not_friction() {
        let bench = engine();
        let installed = engine().with_installed_power(0.96).unwrap();
        for altitude in [0.0, 3_000.0] {
            let air = crate::atmosphere::isa(altitude);
            let c = bench.charge_ratio(&air);
            assert!(close(installed.charge_ratio(&air), 0.96 * c, 1e-12));
            for (speed, throttle) in [(200.0, 1.0), (180.0, 0.5), (150.0, 0.0)] {
                let (b, i) = (
                    bench.torques(speed, throttle, c),
                    installed.torques(speed, throttle, 0.96 * c),
                );
                assert!(close(i.combustion, 0.96 * b.combustion, 1e-12));
                assert_eq!(i.friction, b.friction);
            }
        }
        // At full throttle: 0.96 c (P₀ + P_f) − P_f, here P_f = 0.1 P₀.
        let p0 = bench.brake_power(200.0, 1.0, 1.0);
        let air = crate::atmosphere::isa(2_000.0);
        let c = bench.charge_ratio(&air);
        let p = installed.brake_power(200.0, 1.0, installed.charge_ratio(&air));
        assert!(close(p, 0.96 * c * 1.1 * p0 - 0.1 * p0, 1e-9));
        for bad in [0.0, -0.1, 1.01, f64::NAN] {
            assert_eq!(
                engine().with_installed_power(bad),
                Err(EngineError::BadInstalledPower)
            );
        }
        assert_eq!(engine().with_installed_power(1.0), Ok(engine()));
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
            InlineEngine::new(c, friction, idle, stall, AltitudeLaw::Density)
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
            InlineEngine::new(
                good.clone(),
                good[..1].to_vec(),
                0.1,
                30.0,
                AltitudeLaw::Density
            ),
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
