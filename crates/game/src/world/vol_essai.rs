//! The test flight (plans `m2-vol-essai.md`, `m2-train-sol.md`): `just run` starts it, the
//! SPAD at rest on the field, over the plain, under fair-weather cumulus. The reference
//! scenes of a flight (`assets/scenes`) start from one of its scenarios, with their own
//! camera, sun, commands and clouds.

use std::path::Path;

use bevy::ecs::system::SystemParam;
use bevy::light::atmosphere::ScatteringMedium;
use bevy::math::DVec3;
use bevy::prelude::*;
use escadrille_data::mockup::load_mockup;
use escadrille_data::scenario::load_scenario;
use escadrille_data::scene::{CameraSpec, load_scene};
use escadrille_fdm::aircraft::PilotControls;
use escadrille_weather::clouds::CloudLayer;

use super::atmosphere::{HAZE_TOP_M, Haze, SUMMER_VISIBILITY_M, haze_of_the_day};
use super::meadow::spawn_meadow;
use super::plain::{SUN_AZIMUTH_DEG, SUN_ELEVATION_DEG, Sun, spawn_plain};
use crate::ASSETS_DIR;
use crate::camera::{PilotEye, View, spawn_flight_camera};
use crate::dev::capture::CaptureHold;
use crate::flight::bridge::to_bevy;
use crate::flight::{ON_THE_FIELD, PilotInput, StartCommands, spawn_scenario};
use crate::render::clouds::Clouds;
use crate::world::scenes::FlightScene;

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

/// The clouds of the test flight, until a mission's weather sets them: the field of
/// cumulus of the scene `cumulus_sous_la_base`, reviewed by the art director, as the
/// pilot sees it from the field and climbing (base 1 200 m, 700 m thick).
fn fair_weather() -> Clouds {
    let mut layer = CloudLayer {
        base_altitude: 1_200.0,
        cover: 0.55,
        cloud_type: 1.0,
        ..default()
    };
    layer.thickness[2] = 700.0;
    Clouds { layer, seed: 8 }
}

/// `just run`: the SPAD at rest on the field, under the clouds.
pub fn spawn_on_the_field(mut scene: Scene) -> Result {
    let clouds = fair_weather();
    let haze = haze_under(
        AFTERNOON_SOLAR_HOUR,
        SUMMER_VISIBILITY_M,
        Some(&clouds.layer),
    );
    spawn_flight(ON_THE_FIELD, &mut scene, AFTERNOON, haze, (0.0, 0.0))?;
    scene.commands.insert_resource(clouds);
    Ok(())
}

/// The haze at `solar_hour` with `visibility_m` by day, its afternoon layer up to the base
/// of the clouds (fair-weather cumulus form at the top of the afternoon's mixed layer), or
/// to [`HAZE_TOP_M`] under a clear sky.
fn haze_under(solar_hour: f32, visibility_m: f32, clouds: Option<&CloudLayer>) -> Haze {
    let top = clouds.map_or(HAZE_TOP_M, |c| c.base_altitude as f32);
    haze_of_the_day(solar_hour, visibility_m, top)
}

/// The local solar time of the test flight's afternoon, hours.
const AFTERNOON_SOLAR_HOUR: f32 = 15.0;

/// A scene of `assets/scenes`, captured or flown: its scenario, sun, camera, clouds, haze
/// and held commands. Flown, the levers start where the scene holds them, then follow the
/// gamepad.
pub fn spawn_scene_file(mut scene: Scene, name: Res<FlightScene>) -> Result {
    let file = load_scene(Path::new(ASSETS_DIR), &name.0)?;
    let sun = file.sun.map_or(AFTERNOON, |s| Sun {
        azimuth_deg: s.azimuth.si().to_degrees(),
        elevation_deg: s.elevation.si().to_degrees(),
    });
    let (view, yaw, pitch) = match file.camera {
        CameraSpec::Chase { yaw, pitch } => (View::Chase, yaw, pitch),
        CameraSpec::Pilot { yaw, pitch } => (View::Pilot, yaw, pitch),
    };
    let look = (yaw.si() as f32, pitch.si() as f32);
    let clouds = file.clouds.map(|c| {
        let mut layer = CloudLayer {
            base_altitude: c.base.si(),
            cover: c.cover,
            cloud_type: c.cloud_type,
            ..default()
        };
        if let Some(thickness) = c.thickness {
            layer.thickness[2] = thickness.si();
        }
        Clouds {
            layer,
            seed: c.seed,
        }
    });
    let visibility = file
        .visibility
        .map_or(SUMMER_VISIBILITY_M, |v| v.si() as f32);
    let haze = haze_under(
        AFTERNOON_SOLAR_HOUR,
        visibility,
        clouds.as_ref().map(|c| &c.layer),
    );
    spawn_flight(&file.scenario, &mut scene, sun, haze, look)?;
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
    if let Some(clouds) = clouds {
        scene.commands.insert_resource(clouds);
    }
    Ok(())
}

fn spawn_flight(name: &str, scene: &mut Scene, sun: Sun, haze: Haze, look: (f32, f32)) -> Result {
    let Scene {
        commands,
        asset_server,
        meshes,
        materials,
        images,
        media,
        hold,
    } = scene;
    spawn_plain(commands, meshes, materials, images, media, sun, haze);
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
