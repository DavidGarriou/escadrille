//! Liveries of an aircraft (`assets/aircraft/<id>/livrees.ron`): the colours of its doped
//! fabric and painted metal, and the markings painted on the fabric, for the realistic
//! model (`art/<id>/build.py`, ADR 0007), which paints them into its textures. The first
//! livery is the one the model is built with; the others are variants on the same
//! texture layout (plan `docs/plans/m3-modele-spad7.md`, B2 and B5). Positions are from
//! the datum of `aircraft.ron`, body axes (x forward, y right, z down), metres.

use std::path::Path;

use serde::Deserialize;

use crate::units::Length;
use crate::{DataError, load_checked};

/// The faces a marking can be painted on: `<surface>.<side>`, the surface being one of
/// `aircraft.ron` or `fuselage`, the side `top` or `bottom` for a horizontal surface,
/// `right` or `left` for a vertical one, any of the four for the fuselage.
pub const SIDES: [&str; 4] = ["top", "bottom", "right", "left"];

/// A named colour, sRGB components from 0 to 1 (as a colour picker gives them).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ColourSpec {
    pub name: String,
    pub srgb: [f64; 3],
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum MarkSpec {
    /// Concentric rings of equal width, outermost first, centred on a point projected
    /// onto a face (`SIDES`).
    Roundel {
        face: String,
        centre_m: [f64; 3],
        diameter: Length,
        rings: Vec<String>,
    },
    /// Bands of equal width across a control surface, from its hinge to its trailing
    /// edge, on both sides: the rudder's tricolour.
    Bands { node: String, colours: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LiverySpec {
    pub name: String,
    /// Colour of the doped fabric.
    pub fabric: String,
    /// Colour of the painted metal panels (cowl).
    pub metal: String,
    pub marks: Vec<MarkSpec>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Liveries")]
pub struct LiveryFile {
    pub colours: Vec<ColourSpec>,
    pub liveries: Vec<LiverySpec>,
}

impl LiveryFile {
    pub fn colour(&self, name: &str) -> Option<[f64; 3]> {
        self.colours.iter().find(|c| c.name == name).map(|c| c.srgb)
    }
}

/// The surface and side of a face name, if it is well formed (`SIDES`).
pub fn split_face(face: &str) -> Option<(&str, &str)> {
    let (surface, side) = face.rsplit_once('.')?;
    (!surface.is_empty() && SIDES.contains(&side)).then_some((surface, side))
}

/// The liveries of an aircraft, if it has any.
pub fn load_liveries(assets: &Path, id: &str) -> Result<Option<LiveryFile>, DataError> {
    let path = assets.join("aircraft").join(id).join("livrees.ron");
    if !path.exists() {
        return Ok(None);
    }
    let file: LiveryFile = load_checked(&path)?;
    let invalid = |message: String| DataError::Invalid {
        origin: path.display().to_string(),
        message,
    };
    if file.liveries.is_empty() {
        return Err(invalid("at least one livery".to_owned()));
    }
    for (i, c) in file.colours.iter().enumerate() {
        if file.colours[..i].iter().any(|o| o.name == c.name) {
            return Err(invalid(format!("colour {} defined twice", c.name)));
        }
        if !c.srgb.iter().all(|v| (0.0..=1.0).contains(v)) {
            return Err(invalid(format!(
                "colour {}: sRGB components from 0 to 1",
                c.name
            )));
        }
    }
    let known = |name: &str| -> Result<(), DataError> {
        match file.colour(name) {
            Some(_) => Ok(()),
            None => Err(invalid(format!("unknown colour {name}"))),
        }
    };
    for (i, livery) in file.liveries.iter().enumerate() {
        if file.liveries[..i].iter().any(|o| o.name == livery.name) {
            return Err(invalid(format!("livery {} defined twice", livery.name)));
        }
        known(&livery.fabric)?;
        known(&livery.metal)?;
        for mark in &livery.marks {
            let colours = match mark {
                MarkSpec::Roundel {
                    face,
                    diameter,
                    rings,
                    centre_m,
                } => {
                    if split_face(face).is_none() {
                        return Err(invalid(format!(
                            "{}: face {face} is not <surface>.<{}>",
                            livery.name,
                            SIDES.join("|")
                        )));
                    }
                    if diameter.si() <= 0.0 || !centre_m.iter().all(|v| v.is_finite()) {
                        return Err(invalid(format!(
                            "{}: a roundel on {face} needs a positive diameter and a finite centre",
                            livery.name
                        )));
                    }
                    rings
                }
                MarkSpec::Bands { colours, .. } => colours,
            };
            if colours.is_empty() {
                return Err(invalid(format!("{}: a mark without colours", livery.name)));
            }
            for c in colours {
                known(c)?;
            }
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
    fn the_spad_is_built_in_its_factory_finish() {
        let file = load_liveries(&assets(), "spad7").unwrap().unwrap();
        let first = &file.liveries[0];
        assert_eq!(first.name, "usine-1917");
        let roundels: Vec<&str> = first
            .marks
            .iter()
            .filter_map(|m| match m {
                MarkSpec::Roundel { face, rings, .. } => {
                    assert_eq!(rings, &["rouge", "blanc", "bleu"]);
                    Some(face.as_str())
                }
                MarkSpec::Bands { .. } => None,
            })
            .collect();
        assert_eq!(
            roundels,
            [
                "upper_wing.top",
                "upper_wing.top",
                "lower_wing.bottom",
                "lower_wing.bottom"
            ]
        );
        // The rudder's tricolour, blue at the hinge.
        assert!(first.marks.iter().any(|m| matches!(
            m,
            MarkSpec::Bands { node, colours } if node == "ctrl_rudder" && colours[0] == "bleu"
        )));
        assert!(file.colour(&first.fabric).is_some());
        assert!(
            load_liveries(&assets(), "no_such_aircraft")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn faces_name_a_surface_and_a_side() {
        assert_eq!(split_face("upper_wing.top"), Some(("upper_wing", "top")));
        assert_eq!(split_face("fuselage.left"), Some(("fuselage", "left")));
        assert_eq!(split_face("upper_wing.under"), None);
        assert_eq!(split_face(".top"), None);
        assert_eq!(split_face("upper_wing"), None);
    }

    #[test]
    fn a_mark_must_use_known_colours() {
        let dir = std::env::temp_dir().join(format!("escadrille-livery-{}", std::process::id()));
        let aircraft = dir.join("aircraft").join("unknown");
        std::fs::create_dir_all(&aircraft).unwrap();
        let text = std::fs::read_to_string(assets().join("aircraft/spad7/livrees.ron"))
            .unwrap()
            .replacen("\"bleu\"]", "\"azur\"]", 1);
        std::fs::write(aircraft.join("livrees.ron"), text).unwrap();
        let error = load_liveries(&dir, "unknown").unwrap_err();
        assert!(error.to_string().contains("unknown colour azur"), "{error}");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
