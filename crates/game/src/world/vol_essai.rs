//! The test flight (plans `m2-vol-essai.md`, `m2-train-sol.md`): `just run` starts it, the
//! SPAD at rest on the field, over the plain. The reference scenes of a flight
//! (`assets/scenes`) start from one of its scenarios, with their own camera, sun and
//! commands.

use std::path::Path;

use bevy::ecs::system::SystemParam;
use bevy::light::atmosphere::ScatteringMedium;
use bevy::math::DVec3;
use bevy::prelude::*;
use escadrille_data::mockup::load_mockup;
use escadrille_data::scenario::load_scenario;
use escadrille_data::scene::{CameraSpec, load_scene};
use escadrille_fdm::aircraft::PilotControls;

use super::meadow::spawn_meadow;
use super::plain::{SUN_AZIMUTH_DEG, SUN_ELEVATION_DEG, Sun, spawn_plain};
use crate::ASSETS_DIR;
use crate::camera::{PilotEye, View, spawn_flight_camera};
use crate::dev::capture::{CaptureHold, CaptureRequest};
use crate::flight::bridge::to_bevy;
use crate::flight::{ON_THE_FIELD, PilotInput, StartCommands, spawn_scenario};

/// What a flight's scenes share: the plain and its assets.
#[derive(SystemParam)]
pub struct Scene<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    images: ResMut<'w, Assets<Image>>,
    media: ResMut<'w, Assets<ScatteringMedium>>,
    hold: ResMut<'w, CaptureHold>,
}

/// The afternoon sun of the test flight.
const AFTERNOON: Sun = Sun {
    azimuth_deg: SUN_AZIMUTH_DEG,
    elevation_deg: SUN_ELEVATION_DEG,
};

/// `just run`: the SPAD at rest on the field.
pub fn spawn_on_the_field(mut scene: Scene) -> Result {
    spawn_flight(ON_THE_FIELD, &mut scene, AFTERNOON, (0.0, 0.0))
}

/// A scene of `assets/scenes`: its scenario, sun, camera and held commands.
pub fn spawn_scene_file(mut scene: Scene, request: Res<CaptureRequest>) -> Result {
    let file = load_scene(Path::new(ASSETS_DIR), &request.scene)?;
    let sun = file.sun.map_or(AFTERNOON, |s| Sun {
        azimuth_deg: s.azimuth.si().to_degrees(),
        elevation_deg: s.elevation.si().to_degrees(),
    });
    let (view, yaw, pitch) = match file.camera {
        CameraSpec::Chase { yaw, pitch } => (View::Chase, yaw, pitch),
        CameraSpec::Pilot { yaw, pitch } => (View::Pilot, yaw, pitch),
    };
    let look = (yaw.si() as f32, pitch.si() as f32);
    spawn_flight(&file.scenario, &mut scene, sun, look)?;
    scene.commands.insert_resource(view);
    if let Some(c) = file.controls {
        let held = PilotControls {
            roll: c.roll,
            pitch: c.pitch,
            yaw: c.yaw,
            throttle: c.throttle,
        };
        scene.commands.insert_resource(StartCommands(held));
        scene.commands.insert_resource(PilotInput::holding(held));
    }
    Ok(())
}

fn spawn_flight(name: &str, scene: &mut Scene, sun: Sun, look: (f32, f32)) -> Result {
    let Scene {
        commands,
        asset_server,
        meshes,
        materials,
        images,
        media,
        hold,
    } = scene;
    spawn_plain(commands, meshes, materials, images, media, sun);
    spawn_meadow(commands, meshes, materials, images);
    let (aircraft, cg) = spawn_scenario(name, commands, asset_server, hold)?;
    // The pilot's eye of the 3D model's file (its node cam_pilot_eye).
    let assets = Path::new(ASSETS_DIR);
    let id = load_scenario(assets, name)?.aircraft;
    let eye = load_mockup(assets, &id)?.map_or(cg + DVec3::new(-0.7, 0.0, -0.9), |m| {
        DVec3::from_array(m.pilot_eye_m)
    });
    commands
        .entity(aircraft)
        .insert(PilotEye(to_bevy(eye - cg).as_vec3()));
    spawn_flight_camera(commands, look);
    Ok(())
}
