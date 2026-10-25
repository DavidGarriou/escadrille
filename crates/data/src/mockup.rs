//! Provisional 3D model files (`assets/aircraft/<id>/maquette.ron`): the geometry that a
//! box-and-plate model needs and `aircraft.ron` does not give (fuselage sections, wheels,
//! tail skid, pilot's eye, guns, struts). `tools/mockup` builds the model from both files
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
        assert_eq!(m.fuselage.len(), 6);
        assert!((m.wheels.diameter.si() - 0.70).abs() < 1e-12);
        assert_eq!(m.gun_muzzles_m.len(), 1);
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
            .replace("(x_m: 0.70,", "(x_m: -0.70,");
        std::fs::write(aircraft.join("maquette.ron"), text).unwrap();
        let error = load_mockup(&dir, "backwards").unwrap_err();
        assert!(error.to_string().contains("nose to the tail"), "{error}");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
