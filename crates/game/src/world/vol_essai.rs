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

use super::plain::spawn_plain;
use crate::ASSETS_DIR;
use crate::camera::{PilotEye, View, spawn_flight_camera};
use crate::dev::capture::CaptureHold;
use crate::flight::bridge::to_bevy;
use crate::flight::{ON_THE_FIELD, TEST_FLIGHT, spawn_scenario};

pub const SCENE_CHASE: &str = "vol-essai-poursuite";
pub const SCENE_PILOT: &str = "vol-essai-pilote";

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
pub fn spawn_on_the_field(scene: Scene) -> Result {
    spawn_flight(ON_THE_FIELD, scene)
}

/// The scenes in flight.
pub fn spawn(scene: Scene) -> Result {
    spawn_flight(TEST_FLIGHT, scene)
}

fn spawn_flight(name: &str, scene: Scene) -> Result {
    let Scene {
        mut commands,
        asset_server,
        mut meshes,
        mut materials,
        mut images,
        mut media,
        mut hold,
    } = scene;
    spawn_plain(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut images,
        &mut media,
    );
    let (aircraft, cg) = spawn_scenario(name, &mut commands, &asset_server, &mut hold)?;
    // The pilot's eye of the 3D model's file (its node cam_pilot_eye).
    let assets = Path::new(ASSETS_DIR);
    let id = load_scenario(assets, name)?.aircraft;
    let eye = load_mockup(assets, &id)?.map_or(cg + DVec3::new(-0.7, 0.0, -0.9), |m| {
        DVec3::from_array(m.pilot_eye_m)
    });
    commands
        .entity(aircraft)
        .insert(PilotEye(to_bevy(eye - cg).as_vec3()));
    spawn_flight_camera(&mut commands);
    Ok(())
}

/// The capture of the pilot's view.
pub fn pilot_view(mut view: ResMut<View>) {
    *view = View::Pilot;
}
