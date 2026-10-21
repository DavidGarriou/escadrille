//! Flight test files (`assets/aircraft/<id>/essais.ron`, schema in `format-avion.md`):
//! historical targets with their source and confidence, run by `fdm_lab`.

use std::path::Path;

use serde::Deserialize;

use crate::units::{Duration, Length, Speed};
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Quantitative {
    MaxLevelSpeed {
        altitude: Length,
        target: Speed,
        tolerance_pct: f64,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub source: String,
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

/// Loads `assets/aircraft/<id>/essais.ron`, checking that every number has an origin.
pub fn load_essais(assets: &Path, id: &str) -> Result<EssaisFile, DataError> {
    load_checked(&assets.join("aircraft").join(id).join("essais.ron"))
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
                MaxLevelSpeed(altitude: M(2000.0), target: Kmh(190.0), tolerance_pct: 3.0, source: "S1", confidence: C),
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
        assert!(Confidence::C.blocks_validation(Confidence::C));
        assert!(!Confidence::D.blocks_validation(Confidence::C));
    }
}
