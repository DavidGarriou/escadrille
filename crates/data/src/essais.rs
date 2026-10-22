//! Flight test files (`assets/aircraft/<id>/essais.ron`, schema in `format-avion.md`):
//! historical targets with their source and confidence, run by `fdm_lab`.

use std::path::Path;

use serde::Deserialize;

use crate::units::{Angle, Duration, Length, RotationSpeed, Speed};
use crate::{DataError, load_checked};

/// Confidence of a target (skill `recherche-historique`): A primary measurement,
/// B concordant secondary sources, C deduced from comparable aircraft, D gameplay estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Confidence {
    A,
    B,
    C,
    D,
}

impl Confidence {
    /// Targets of confidence C and D are reported but do not block the validation,
    /// except where the player decided otherwise (M1 validated at confidence C).
    pub fn blocks_validation(self, accepted: Confidence) -> bool {
        self <= accepted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PowerSetting {
    Idle,
    Full,
}

/// Engine speed measured with a level speed: checks the propeller and engine together.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct RpmTarget {
    pub target: RotationSpeed,
    pub tolerance_pct: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Quantitative {
    MaxLevelSpeed {
        altitude: Length,
        target: Speed,
        tolerance_pct: f64,
        /// Engine speed in that level flight, when the source gives it.
        #[serde(default)]
        rpm: Option<RpmTarget>,
        source: String,
        confidence: Confidence,
    },
    ClimbTime {
        from: Length,
        to: Length,
        target: Duration,
        tolerance_pct: f64,
        source: String,
        confidence: Confidence,
    },
    StallSpeed {
        power: PowerSetting,
        target: Speed,
        tolerance_pct: f64,
        source: String,
        confidence: Confidence,
    },
    /// Altitude where the best rate of climb falls to `climb_rate` (100 ft/min for the
    /// service ceiling of the period).
    Ceiling {
        climb_rate: Speed,
        target: Length,
        tolerance_pct: f64,
        source: String,
        confidence: Confidence,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Robustness {
    ZeroAirspeed,
    TailSlide,
    SpinEntryRecovery { turns: u32 },
    Inverted { seconds: f64 },
    MaxDive,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Conditions {
    pub fuel_fraction: f64,
}

/// A manoeuvre flown by the `fdm_lab` pilot to show a behaviour from the testimonies.
/// What the aircraft does is reported against the scenario's description, not judged.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Manoeuvre {
    /// From level flight at `speed` (default: the maximum level speed) at `altitude`,
    /// full throttle, a loop pulled at `load_factor` without exceeding the stall
    /// incidence, wings held in the plane.
    Loop {
        altitude: Length,
        #[serde(default)]
        speed: Option<Speed>,
        load_factor: f64,
    },
    /// From level flight at 1.3 times the idle stall speed, engine at idle, the speed
    /// reduced by a knot per second until the stall: speed at the break, nose and wing
    /// drop with the controls held, height lost in the recovery.
    PowerOffStall { altitude: Length },
    /// Best glide ratio, engine at idle.
    Glide { altitude: Length },
    /// From the maximum level speed at `altitude`, a dive held at `angle` (−90° to 0°)
    /// at full throttle for `seconds`, the pitching moment with the elevator neutral
    /// recorded; from there, the elevator released to neutral for a few seconds, and
    /// separately a pull-out at `load_factor`.
    Dive {
        altitude: Length,
        angle: Angle,
        seconds: f64,
        load_factor: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub source: String,
    /// `None` for a behaviour not flown by `fdm_lab` (ground handling, cockpit...).
    #[serde(default)]
    pub manoeuvre: Option<Manoeuvre>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Essais")]
pub struct EssaisFile {
    pub aircraft: String,
    pub conditions: Conditions,
    pub quantitative: Vec<Quantitative>,
    /// Behaviours from the pilots' testimonies (fiche section 6), scripted in `fdm_lab`.
    #[serde(default)]
    pub qualitative: Vec<Scenario>,
    #[serde(default)]
    pub robustness: Vec<Robustness>,
}

/// Shortest held dive, s: its first two seconds are the entry.
pub const MIN_DIVE_SECONDS: f64 = 5.0;

impl Manoeuvre {
    /// Why the manoeuvre cannot be flown, if it cannot.
    pub fn check(&self) -> Result<(), String> {
        let altitude = |a: &Length| {
            if a.si() >= 0.0 {
                Ok(())
            } else {
                Err(format!("altitude négative : {:.0} m", a.si()))
            }
        };
        let pull = |n: f64| {
            if n > 1.0 {
                Ok(())
            } else {
                Err(format!(
                    "facteur de charge de ressource {n} : il faut plus de 1"
                ))
            }
        };
        match self {
            Self::Loop {
                altitude: a,
                speed,
                load_factor,
            } => {
                altitude(a)?;
                pull(*load_factor)?;
                match speed {
                    Some(v) if v.si() <= 0.0 => {
                        Err("vitesse d'entrée nulle ou négative".to_owned())
                    }
                    _ => Ok(()),
                }
            }
            Self::PowerOffStall { altitude: a } | Self::Glide { altitude: a } => altitude(a),
            Self::Dive {
                altitude: a,
                angle,
                seconds,
                load_factor,
            } => {
                altitude(a)?;
                pull(*load_factor)?;
                let degrees = angle.si().to_degrees();
                if !(-90.0..0.0).contains(&degrees) {
                    return Err(format!("angle de piqué {degrees:.0}° : entre −90° et 0°"));
                }
                if *seconds < MIN_DIVE_SECONDS {
                    return Err(format!(
                        "piqué de {seconds} s : au moins {MIN_DIVE_SECONDS} s"
                    ));
                }
                Ok(())
            }
        }
    }
}

/// Loads `assets/aircraft/<id>/essais.ron`, checking that every number has an origin and
/// that every manoeuvre can be flown.
pub fn load_essais(assets: &Path, id: &str) -> Result<EssaisFile, DataError> {
    let path = assets.join("aircraft").join(id).join("essais.ron");
    let file: EssaisFile = load_checked(&path)?;
    for scenario in &file.qualitative {
        if let Some(m) = &scenario.manoeuvre {
            m.check().map_err(|message| DataError::Invalid {
                origin: path.display().to_string(),
                message: format!("scénario {} : {message}", scenario.name),
            })?;
        }
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::from_ron_str;

    #[test]
    fn parses_targets_with_period_units() {
        let text = r#"Essais(
            aircraft: "test",
            conditions: (fuel_fraction: 1.0),
            quantitative: [
                MaxLevelSpeed(altitude: M(2000.0), target: Kmh(190.0), tolerance_pct: 3.0, rpm: Some((target: Rpm(1720.0), tolerance_pct: 3.0)), source: "S1", confidence: C),
                ClimbTime(from: M(0.0), to: M(3000.0), target: MinS(11, 30), tolerance_pct: 5.0, source: "S1", confidence: B),
                Ceiling(climb_rate: FtMin(100.0), target: M(5500.0), tolerance_pct: 5.0, source: "S2", confidence: C),
            ],
            robustness: [TailSlide, SpinEntryRecovery(turns: 3)],
        )"#;
        let file: EssaisFile = from_ron_str(text, "test").unwrap();
        assert_eq!(file.quantitative.len(), 3);
        let Quantitative::ClimbTime { target, .. } = &file.quantitative[1] else {
            panic!("{file:?}");
        };
        assert_eq!(target.si(), 690.0);
        let Quantitative::MaxLevelSpeed { rpm: Some(rpm), .. } = &file.quantitative[0] else {
            panic!("{file:?}");
        };
        assert!((rpm.target.si() - 1720.0 * std::f64::consts::PI / 30.0).abs() < 1e-12);
        assert!(Confidence::C.blocks_validation(Confidence::C));
        assert!(!Confidence::D.blocks_validation(Confidence::C));
    }

    #[test]
    fn parses_manoeuvres_and_unflown_behaviours() {
        let text = r#"Essais(
            aircraft: "test",
            conditions: (fuel_fraction: 1.0),
            quantitative: [],
            qualitative: [
                (name: "loop", description: "easy loop", source: "T1", manoeuvre: Some(Loop(altitude: M(2000.0), speed: Some(Kmh(150.0)), load_factor: 3.5))),
                (name: "dive", description: "holds a dive", source: "T2", manoeuvre: Some(Dive(altitude: M(3000.0), angle: Deg(-80.0), seconds: 20.0, load_factor: 4.0))),
                (name: "ground", description: "swings left", source: "T8"),
            ],
        )"#;
        let file: EssaisFile = from_ron_str(text, "test").unwrap();
        assert_eq!(file.qualitative.len(), 3);
        let Some(Manoeuvre::Dive { angle, .. }) = &file.qualitative[1].manoeuvre else {
            panic!("{file:?}");
        };
        assert!((angle.si() + 80f64.to_radians()).abs() < 1e-12);
        assert!(file.qualitative[2].manoeuvre.is_none());
        assert!(
            file.qualitative
                .iter()
                .flat_map(|q| &q.manoeuvre)
                .all(|m| m.check().is_ok())
        );
    }

    #[test]
    fn manoeuvres_that_cannot_be_flown_are_refused() {
        let refused = [
            "Loop(altitude: M(1000.0), load_factor: 0.9)",
            "Loop(altitude: M(1000.0), speed: Some(Kmh(0.0)), load_factor: 3.0)",
            "Glide(altitude: M(-10.0))",
            "Dive(altitude: M(3000.0), angle: Deg(-95.0), seconds: 20.0, load_factor: 4.0)",
            "Dive(altitude: M(3000.0), angle: Deg(10.0), seconds: 20.0, load_factor: 4.0)",
            "Dive(altitude: M(3000.0), angle: Deg(-80.0), seconds: 1.0, load_factor: 4.0)",
        ];
        for text in refused {
            let m: Manoeuvre = from_ron_str(text, "test").unwrap();
            assert!(m.check().is_err(), "{text}");
        }
        let vertical: Manoeuvre = from_ron_str(
            "Dive(altitude: M(3000.0), angle: Deg(-90.0), seconds: 20.0, load_factor: 4.0)",
            "test",
        )
        .unwrap();
        assert!(vertical.check().is_ok());
    }
}
