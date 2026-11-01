//! The test flight (plans `m2-vol-essai.md`, `m2-train-sol.md`): `just run` starts it, the
//! SPAD at rest on the field, over the plain. Two reference scenes capture its level
//! flight after a few seconds, hands off: `vol-essai-poursuite` (chase view) and
//! `vol-essai-pilote` (pilot's view).

use std::path::Path;

use bevy::ecs::system::SystemParam;
use bevy::light::atmosphere::ScatteringMedium;
use bevy::math::DVec3;
use bevy::prelude::*;
use escadrille_data::mockup::load_mockup;
use escadrille_data::scenario::load_scenario;
use escadrille_fdm::aircraft::PilotControls;

use super::meadow::spawn_meadow;
use super::plain::spawn_plain;
use crate::ASSETS_DIR;
use crate::camera::{FlightCamera, PilotEye, View, spawn_flight_camera};
use crate::dev::capture::CaptureHold;
use crate::flight::bridge::to_bevy;
use crate::flight::{
    FINAL_APPROACH, ON_THE_FIELD, PilotInput, StartCommands, TEST_FLIGHT, spawn_scenario,
};

pub const SCENE_CHASE: &str = "vol-essai-poursuite";
pub const SCENE_PILOT: &str = "vol-essai-pilote";
/// At rest on the field, from the chase view and from the pilot's seat.
pub const SCENE_FIELD: &str = "terrain-repos";
pub const SCENE_FIELD_PILOT: &str = "terrain-pilote";
/// Rolling, throttle open: long enough a capture (`--frames`) shows the tail coming up.
pub const SCENE_ROLL: &str = "terrain-roulage";
/// The meadow as the pilot sees it from the start of the final approach.
pub const SCENE_FINAL: &str = "terrain-finale";
/// The chase view from ahead and to the side: the wheels and the skid on the ground.
pub const SCENE_WHEELS: &str = "terrain-roues";
/// Chase camera's yaw of that scene, rad: ahead of the left wing.
const WHEELS_YAW: f32 = 2.2;

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

/// `just run`, and the scenes on the field: the SPAD at rest.
pub fn spawn_on_the_field(mut scene: Scene) -> Result {
    spawn_flight(ON_THE_FIELD, &mut scene)
}

/// The scenes in flight.
pub fn spawn(mut scene: Scene) -> Result {
    spawn_flight(TEST_FLIGHT, &mut scene)
}

/// The scene of the final approach.
pub fn spawn_on_final(mut scene: Scene) -> Result {
    spawn_flight(FINAL_APPROACH, &mut scene)
}

/// The rolling scene: on the field, throttle open and the stick pushed.
pub fn spawn_rolling(mut scene: Scene) -> Result {
    spawn_flight(ON_THE_FIELD, &mut scene)?;
    let held = PilotControls {
        throttle: 1.0,
        pitch: -0.5,
        ..PilotControls::default()
    };
    scene.commands.insert_resource(StartCommands(held));
    scene.commands.insert_resource(PilotInput::holding(held));
    Ok(())
}

fn spawn_flight(name: &str, scene: &mut Scene) -> Result {
    let Scene {
        commands,
        asset_server,
        meshes,
        materials,
        images,
        media,
        hold,
    } = scene;
    spawn_plain(commands, meshes, materials, images, media);
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
    spawn_flight_camera(commands);
    Ok(())
}

/// The chase camera of the scene of the wheels.
pub fn look_at_the_wheels(mut cameras: Query<&mut FlightCamera>) {
    for mut camera in &mut cameras {
        camera.yaw = WHEELS_YAW;
    }
}

/// The capture of the pilot's view.
pub fn pilot_view(mut view: ResMut<View>) {
    *view = View::Pilot;
}
