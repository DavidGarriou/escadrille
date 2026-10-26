//! Provisional 3D model files (`assets/aircraft/<id>/maquette.ron`): the geometry that a
//! box-and-plate model needs and `aircraft.ron` does not give (fuselage sections, wheels,
//! tail skid, pilot's eye, guns, struts), and the checks of `tools/asset_check` that it
//! fails for a known, accepted reason. `tools/mockup` builds the model from both files
//! (plan `docs/plans/modele-3d-provisoire.md`). Positions are from the datum of
//! `aircraft.ron`, body axes (x forwards, y right, z down), metres.

use std::path::Path;

use serde::Deserialize;

use crate::units::Length;
use crate::{DataError, load_checked};

/// A cross-section of the fuselage, a rectangle centred on the plane of symmetry.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct SectionSpec {
    pub x_m: f64,
    pub width: Length,
    /// Top and bottom of the section (z, down positive).
    pub top_m: f64,
    pub bottom_m: f64,
}

/// The main wheels, mirrored about the plane of symmetry.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct WheelsSpec {
    /// Centre of the right wheel.
    pub centre_m: [f64; 3],
    pub diameter: Length,
    pub width: Length,
}

/// A pair of interplane struts, mirrored: from the lower plane to the upper one.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct StrutSpec {
    pub x_m: f64,
    /// Right-hand strut.
    pub y_m: f64,
}

/// The drawn outline of a lifting surface, for the realistic model (Blender, ADR 0007):
/// leading and trailing edge x along the span, where `aircraft.ron` may describe an
/// aerodynamic equivalent. Rows `(s, x_le, x_te)`, `s` being y on the right half for a
/// horizontal surface (mirrored for the left), z for a vertical one, metres.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OutlineSpec {
    pub surface: String,
    pub rows: Vec<[f64; 3]>,
}

/// A named table of numbers (metres, or fractions where the script says so) for the
/// realistic model's details that `aircraft.ron` does not hold: nose, exhausts, cut-out,
/// struts, gear, cockpit. The Blender script (`art/<id>/build.py`) names and reads them;
/// each carries its origin in a comment, as any data.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FeatureSpec {
    pub name: String,
    pub rows: Vec<Vec<f64>>,
}

/// A check of `tools/asset_check` that the provisional model fails for a known reason,
/// accepted with the player's agreement: it passes while the model measures `measured`
/// (a change of the data that moves it must be looked at again).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AcceptedDeviation {
    /// Name of the check, as `asset_check` prints it (`span`, `length`, `height`).
    pub check: String,
    pub measured: Length,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Mockup")]
pub struct MockupFile {
    /// From the nose to the tail post, x decreasing.
    pub fuselage: Vec<SectionSpec>,
    pub wheels: WheelsSpec,
    /// Point of the tail skid that touches the ground.
    pub skid_m: [f64; 3],
    pub struts: Vec<StrutSpec>,
    pub pilot_eye_m: [f64; 3],
    /// Muzzle of each gun, firing along +x.
    pub gun_muzzles_m: Vec<[f64; 3]>,
    #[serde(default)]
    pub accepted_deviations: Vec<AcceptedDeviation>,
    #[serde(default)]
    pub outlines: Vec<OutlineSpec>,
    #[serde(default)]
    pub features: Vec<FeatureSpec>,
}

/// The provisional model file of an aircraft, if it has one.
pub fn load_mockup(assets: &Path, id: &str) -> Result<Option<MockupFile>, DataError> {
    let path = assets.join("aircraft").join(id).join("maquette.ron");
    if !path.exists() {
        return Ok(None);
    }
    let file: MockupFile = load_checked(&path)?;
    let origin = path.display().to_string();
    let invalid = |message: &str| DataError::Invalid {
        origin: origin.clone(),
        message: message.to_owned(),
    };
    if file.fuselage.len() < 2 {
        return Err(invalid("the fuselage needs at least two sections"));
    }
    if !file.fuselage.windows(2).all(|w| w[1].x_m < w[0].x_m) {
        return Err(invalid(
            "fuselage sections must run from the nose to the tail",
        ));
    }
    let positive = |l: Length| l.si() > 0.0;
    if !file
        .fuselage
        .iter()
        .all(|s| s.width.si() >= 0.0 && s.bottom_m >= s.top_m)
        || !positive(file.wheels.diameter)
        || !positive(file.wheels.width)
    {
        return Err(invalid("negative width, height or wheel size"));
    }
    if file.gun_muzzles_m.is_empty() {
        return Err(invalid("at least one gun muzzle"));
    }
    if file
        .accepted_deviations
        .iter()
        .any(|d| !positive(d.measured) || d.reason.trim().is_empty())
    {
        return Err(invalid(
            "an accepted deviation needs a measured value and a reason",
        ));
    }
    let mut names: Vec<&str> = file.features.iter().map(|f| f.name.as_str()).collect();
    names.sort_unstable();
    if names.windows(2).any(|w| w[0] == w[1])
        || file
            .features
            .iter()
            .any(|f| f.rows.is_empty() || f.rows.iter().flatten().any(|v| !v.is_finite()))
    {
        return Err(invalid(
            "features need a unique name and at least one row of finite numbers",
        ));
    }
    for outline in &file.outlines {
        let monotonic = outline.rows.windows(2).all(|w| w[1][0] != w[0][0])
            && (outline.rows.windows(2).all(|w| w[1][0] > w[0][0])
                || outline.rows.windows(2).all(|w| w[1][0] < w[0][0]));
        if outline.rows.len() < 2 || !monotonic || outline.rows.iter().any(|r| r[1] < r[2]) {
            return Err(invalid(&format!(
                "outline of {}: at least two rows, monotonic along the span, leading edge ahead of the trailing edge",
                outline.surface
            )));
        }
    }
    Ok(Some(file))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn assets() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
    }

    #[test]
    fn the_spad_has_a_provisional_model_file() {
        let m = load_mockup(&assets(), "spad7").unwrap().unwrap();
        assert_eq!(m.fuselage.len(), 52);
        assert!((m.wheels.diameter.si() - 0.61).abs() < 1e-12);
        assert_eq!(m.gun_muzzles_m.len(), 1);
        let deviations: Vec<&str> = m
            .accepted_deviations
            .iter()
            .map(|d| d.check.as_str())
            .collect();
        assert_eq!(deviations, ["height"]);
        let outlines: Vec<&str> = m.outlines.iter().map(|o| o.surface.as_str()).collect();
        assert_eq!(outlines, ["tailplane", "fin"]);
        let radiator = m.features.iter().find(|f| f.name == "radiator").unwrap();
        assert_eq!(radiator.rows, [[0.66, -0.43, 0.67]]);
        assert!(
            load_mockup(&assets(), "no_such_aircraft")
                .unwrap()
                .is_none()
        );
        // The model is checked against the overall dimensions of aircraft.ron.
        let d = crate::aircraft::load_aircraft(&assets(), "spad7")
            .unwrap()
            .file
            .dimensions
            .unwrap();
        assert_eq!(
            (d.span.si(), d.length.si(), d.height.si()),
            (7.82, 6.08, 2.20)
        );
    }

    #[test]
    fn a_fuselage_must_run_from_nose_to_tail() {
        let dir = std::env::temp_dir().join(format!("escadrille-mockup-{}", std::process::id()));
        let aircraft = dir.join("aircraft").join("backwards");
        std::fs::create_dir_all(&aircraft).unwrap();
        let text = std::fs::read_to_string(assets().join("aircraft/spad7/maquette.ron"))
            .unwrap()
            .replace("(x_m: 0.65,", "(x_m: -0.65,");
        std::fs::write(aircraft.join("maquette.ron"), text).unwrap();
        let error = load_mockup(&dir, "backwards").unwrap_err();
        assert!(error.to_string().contains("nose to the tail"), "{error}");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn features_need_unique_names() {
        let dir = std::env::temp_dir().join(format!("escadrille-features-{}", std::process::id()));
        let aircraft = dir.join("aircraft").join("twice");
        std::fs::create_dir_all(&aircraft).unwrap();
        let text = std::fs::read_to_string(assets().join("aircraft/spad7/maquette.ron"))
            .unwrap()
            .replace("(name: \"cowl_ring\",", "(name: \"radiator\",");
        std::fs::write(aircraft.join("maquette.ron"), text).unwrap();
        let error = load_mockup(&dir, "twice").unwrap_err();
        assert!(error.to_string().contains("unique name"), "{error}");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
