//! The test flight (plans `m2-vol-essai.md`, `m2-train-sol.md`): `just run` starts it, the
//! SPAD at rest on the field, over the plain, under fair-weather cumulus. The reference
//! scenes of a flight (`assets/scenes`) start from one of its scenarios, with their own
//! camera, sun, commands and clouds.

use std::path::Path;

use bevy::ecs::system::SystemParam;
use bevy::light::atmosphere::ScatteringMedium;
use bevy::math::DVec3;
use bevy::prelude::*;
use escadrille_data::legal_time::to_utc;
use escadrille_data::mockup::load_mockup;
use escadrille_data::scenario::load_scenario;
use escadrille_data::scene::{CameraSpec, SunSpec, TimeSpec, load_scene};
use escadrille_fdm::aircraft::PilotControls;
use escadrille_weather::clouds::CloudLayer;
use escadrille_weather::sun::{SunPosition, Utc};

use super::atmosphere::{HAZE_TOP_M, Haze, SUMMER_VISIBILITY_M, haze_of_the_day};
use super::meadow::spawn_meadow;
use super::plain::{SUN_AZIMUTH_DEG, SUN_ELEVATION_DEG, Sun, spawn_plain};
use super::sky::{Sky, YPRES};
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

/// The test flight's day and hour, French legal time: 1 September 1917, 15 h, the sun
/// 224° and 39.6° high over Ypres, where the plain's afternoon had it (US Naval
/// Observatory, `docs/recherche/heure-legale-1917.md` § 7).
pub const FLIGHT_TIME: TimeSpec = TimeSpec {
    year: 1917,
    month: 9,
    day: 1,
    hour: 15,
    minute: 0,
};

/// `just run`: the SPAD at rest on the field, under the clouds.
pub fn spawn_on_the_field(mut scene: Scene) -> Result {
    let clouds = fair_weather();
    let sky = sky_of(Some(FLIGHT_TIME), None);
    let haze = haze_under(&sky, SUMMER_VISIBILITY_M, Some(&clouds.layer));
    spawn_flight(ON_THE_FIELD, &mut scene, &sky, haze, (0.0, 0.0))?;
    scene.commands.insert_resource(clouds);
    Ok(())
}

/// The sky of a scene: the sun and the moon of its time over Ypres, the sun put where the
/// scene says if it does; without a time, the plain's afternoon sun, and no moon.
fn sky_of(time: Option<TimeSpec>, sun: Option<SunSpec>) -> Sky {
    let mut sky = match time {
        Some(t) => {
            let (year, month, day, hour) = to_utc(t);
            Sky::at(
                Utc {
                    year,
                    month,
                    day,
                    hour,
                },
                YPRES,
            )
        }
        None => Sky::sun_only(SunPosition {
            azimuth_deg: SUN_AZIMUTH_DEG,
            elevation_deg: SUN_ELEVATION_DEG,
        }),
    };
    if let Some(s) = sun {
        sky.sun = SunPosition {
            azimuth_deg: s.azimuth.si().to_degrees(),
            elevation_deg: s.elevation.si().to_degrees(),
        };
    }
    sky
}

/// The haze of the sky's hour with `visibility_m` by day, its afternoon layer up to the
/// base of the clouds (fair-weather cumulus form at the top of the afternoon's mixed
/// layer), or to [`HAZE_TOP_M`] under a clear sky.
fn haze_under(sky: &Sky, visibility_m: f32, clouds: Option<&CloudLayer>) -> Haze {
    let top = clouds.map_or(HAZE_TOP_M, |c| c.base_altitude as f32);
    haze_of_the_day(sky.solar_hour() as f32, visibility_m, top)
}

/// A scene of `assets/scenes`, captured or flown: its scenario, sun, camera, clouds, haze
/// and held commands. Flown, the levers start where the scene holds them, then follow the
/// gamepad.
pub fn spawn_scene_file(mut scene: Scene, name: Res<FlightScene>) -> Result {
    let file = load_scene(Path::new(ASSETS_DIR), &name.0)?;
    let sky = sky_of(file.time, file.sun);
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
    let haze = haze_under(&sky, visibility, clouds.as_ref().map(|c| &c.layer));
    spawn_flight(&file.scenario, &mut scene, &sky, haze, look)?;
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

fn spawn_flight(name: &str, scene: &mut Scene, sky: &Sky, haze: Haze, look: (f32, f32)) -> Result {
    let Scene {
        commands,
        asset_server,
        meshes,
        materials,
        images,
        media,
        hold,
    } = scene;
    let sun = Sun {
        azimuth_deg: sky.sun.azimuth_deg,
        elevation_deg: sky.sun.elevation_deg,
    };
    spawn_plain(commands, meshes, materials, images, media, sun, haze);
    commands.insert_resource(*sky);
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
