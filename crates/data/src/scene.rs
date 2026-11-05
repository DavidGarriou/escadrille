//! Reference scenes (`assets/scenes/<name>.ron`; skills `rendu-visuel` and `capture`):
//! what the capture mode renders, for Claude and the art director to look at. A scene is
//! a view, not historical data: its numbers carry no origin.
//!
//! The scenes of a flight live here: a scenario, a camera, a sun, the commands held. The
//! ones that set up the aircraft by hand (studio, damage gallery, axes of the model) are
//! still built in code, in the game.

use std::path::Path;

use serde::Deserialize;

use crate::units::Angle;
use crate::{DataError, load_ron};

/// Where the camera looks from.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum CameraSpec {
    /// Following the aircraft from behind, turned by `yaw` (to the right) and `pitch`.
    Chase { yaw: Angle, pitch: Angle },
    /// At the pilot's eye, looking `yaw` to the right and `pitch` up.
    Pilot { yaw: Angle, pitch: Angle },
}

/// The sun's direction.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct SunSpec {
    /// From north towards east.
    pub azimuth: Angle,
    pub elevation: Angle,
}

/// The pilot's commands held throughout: stick and rudder from −1 to 1, throttle from 0 to 1.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ControlsSpec {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub throttle: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Scene")]
pub struct SceneFile {
    /// What the scene checks, in French.
    pub description: String,
    /// The flight it starts from (`assets/scenarios`).
    pub scenario: String,
    pub camera: CameraSpec,
    /// The commands held, instead of the scenario's own.
    #[serde(default)]
    pub controls: Option<ControlsSpec>,
    /// The plain's afternoon sun when left out.
    #[serde(default)]
    pub sun: Option<SunSpec>,
    /// Frames rendered before the capture, unless the command line says otherwise.
    #[serde(default)]
    pub frames: Option<u32>,
}

fn directory(assets: &Path) -> std::path::PathBuf {
    assets.join("scenes")
}

/// Loads `assets/scenes/<name>.ron`.
pub fn load_scene(assets: &Path, name: &str) -> Result<SceneFile, DataError> {
    let path = directory(assets).join(format!("{name}.ron"));
    let scene: SceneFile = load_ron(&path)?;
    let commands = scene.controls.iter().all(|c| {
        [c.roll, c.pitch, c.yaw].iter().all(|v| v.abs() <= 1.0) && (0.0..=1.0).contains(&c.throttle)
    });
    let sun = scene
        .sun
        .iter()
        .all(|s| (-90.0..=90.0).contains(&s.elevation.si().to_degrees()));
    if !(commands && sun && scene.frames != Some(0)) {
        return Err(DataError::Invalid {
            origin: path.display().to_string(),
            message: "commands from −1 to 1 and throttle from 0 to 1, sun from −90° to 90°, \
                      at least one frame"
                .to_owned(),
        });
    }
    Ok(scene)
}

/// The names of the scenes in `assets/scenes`, sorted.
pub fn scene_names(assets: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory(assets))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension()? == "ron")
                .then(|| path.file_stem()?.to_str().map(str::to_owned))
                .flatten()
        })
        .collect();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::scenario::load_scenario;

    fn assets() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
    }

    #[test]
    fn every_scene_loads_and_starts_from_a_scenario() {
        let names = scene_names(&assets());
        assert!(names.contains(&"terrain-repos".to_owned()), "{names:?}");
        for name in names {
            let scene = load_scene(&assets(), &name).unwrap();
            assert!(!scene.description.is_empty(), "{name}");
            load_scenario(&assets(), &scene.scenario).unwrap();
        }
    }
}
