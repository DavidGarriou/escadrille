//! Flight-test campaign: the historical targets of `essais.ron` against the flight
//! model (skill `essai-en-vol`). Each target gives an [`Outcome`]; a target blocks the
//! validation when its confidence is at least the accepted one (M1: C, player's decision
//! of 2026-10-20).

use escadrille_data::essais::{Confidence, EssaisFile, PowerSetting, Quantitative};
use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::environment::Environment;

use crate::performance::{ceiling, max_level_speed, stall_speed, time_to_climb};

/// Altitude of the stall-speed test, m (low, clear of the ground effect, E:).
const STALL_TEST_ALTITUDE: f64 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Speed,
    Duration,
    Altitude,
}

impl Unit {
    /// Display value and symbol: km/h, min, m.
    pub fn display(self, si: f64) -> (f64, &'static str) {
        match self {
            Self::Speed => (si * 3.6, "km/h"),
            Self::Duration => (si / 60.0, "min"),
            Self::Altitude => (si, "m"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    /// What was tested, in French (report language).
    pub label: String,
    pub unit: Unit,
    /// SI.
    pub target: f64,
    /// SI; `None` when the model could not compute it.
    pub computed: Option<f64>,
    pub tolerance_pct: f64,
    pub confidence: Confidence,
    pub source: String,
    /// Why nothing was computed.
    pub note: Option<String>,
    pub blocking: bool,
}

impl Outcome {
    /// Signed relative error, %.
    pub fn error_pct(&self) -> Option<f64> {
        self.computed
            .map(|c| 100.0 * (c - self.target) / self.target)
    }

    pub fn passed(&self) -> bool {
        self.error_pct()
            .is_some_and(|e| e.abs() <= self.tolerance_pct)
    }
}

/// Runs the quantitative targets of a test file.
pub fn run(
    aircraft: &Aircraft,
    env: &dyn Environment,
    essais: &EssaisFile,
    accepted: Confidence,
) -> Vec<Outcome> {
    essais
        .quantitative
        .iter()
        .map(|q| {
            let (label, unit, target, tolerance_pct, source, confidence, computed) = match q {
                Quantitative::MaxLevelSpeed {
                    altitude,
                    target,
                    tolerance_pct,
                    source,
                    confidence,
                } => (
                    format!("Vitesse maximale à {:.0} m", altitude.si()),
                    Unit::Speed,
                    target.si(),
                    *tolerance_pct,
                    source,
                    *confidence,
                    max_level_speed(aircraft, env, altitude.si())
                        .map(|t| t.flight.speed)
                        .map_err(|e| format!("{e:?}")),
                ),
                Quantitative::ClimbTime {
                    from,
                    to,
                    target,
                    tolerance_pct,
                    source,
                    confidence,
                } => (
                    format!("Montée de {:.0} à {:.0} m", from.si(), to.si()),
                    Unit::Duration,
                    target.si(),
                    *tolerance_pct,
                    source,
                    *confidence,
                    time_to_climb(aircraft, env, from.si(), to.si()).map_err(|e| format!("{e:?}")),
                ),
                Quantitative::StallSpeed {
                    power,
                    target,
                    tolerance_pct,
                    source,
                    confidence,
                } => (
                    format!(
                        "Vitesse de décrochage ({})",
                        match power {
                            PowerSetting::Idle => "moteur au ralenti",
                            PowerSetting::Full => "pleins gaz",
                        }
                    ),
                    Unit::Speed,
                    target.si(),
                    *tolerance_pct,
                    source,
                    *confidence,
                    match power {
                        PowerSetting::Idle => stall_speed(aircraft, env, STALL_TEST_ALTITUDE)
                            .map(|s| s.speed)
                            .map_err(|e| format!("{e:?}")),
                        PowerSetting::Full => {
                            Err("décrochage pleins gaz : pas encore calculé".into())
                        }
                    },
                ),
                Quantitative::Ceiling {
                    climb_rate,
                    target,
                    tolerance_pct,
                    source,
                    confidence,
                } => (
                    format!("Plafond (montée de {:.2} m/s)", climb_rate.si()),
                    Unit::Altitude,
                    target.si(),
                    *tolerance_pct,
                    source,
                    *confidence,
                    ceiling(aircraft, env, climb_rate.si()).map_err(|e| format!("{e:?}")),
                ),
            };
            let (computed, note) = match computed {
                Ok(value) => (Some(value), None),
                Err(message) => (None, Some(message)),
            };
            Outcome {
                label,
                unit,
                target,
                computed,
                tolerance_pct,
                confidence,
                source: source.clone(),
                note,
                blocking: confidence.blocks_validation(accepted),
            }
        })
        .collect()
}

/// Whether every blocking target passed.
pub fn validated(outcomes: &[Outcome]) -> bool {
    outcomes.iter().all(|o| !o.blocking || o.passed())
}

#[cfg(test)]
mod tests {
    use escadrille_data::from_ron_str;

    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};

    #[test]
    fn targets_are_compared_within_their_tolerance() {
        // Synthetic targets (not historical): the first near the model, the second far
        // off but only confidence D, the third far off and blocking.
        let text = r#"Essais(
            aircraft: "test_biplane",
            conditions: (fuel_fraction: 1.0),
            quantitative: [
                MaxLevelSpeed(altitude: M(1000.0), target: Kmh(168.0), tolerance_pct: 3.0, source: "test", confidence: B),
                MaxLevelSpeed(altitude: M(1000.0), target: Kmh(250.0), tolerance_pct: 3.0, source: "test", confidence: D),
                StallSpeed(power: Full, target: Kmh(80.0), tolerance_pct: 5.0, source: "test", confidence: C),
            ],
        )"#;
        let essais: EssaisFile = from_ron_str(text, "test").unwrap();
        let outcomes = run(&biplane(), &STILL_AIR, &essais, Confidence::C);
        assert_eq!(outcomes.len(), 3);
        assert!(
            outcomes[0].passed() && outcomes[0].blocking,
            "{:?}",
            outcomes[0]
        );
        assert!(!outcomes[1].passed() && !outcomes[1].blocking);
        assert!(outcomes[2].computed.is_none() && outcomes[2].note.is_some());
        assert!(!validated(&outcomes), "an uncomputed blocking target fails");
        assert!(validated(&outcomes[..2]));
    }
}
