//! The test flight (plan `m2-vol-essai.md`): `just run` starts it, the SPAD in level
//! flight over the plain. Two reference scenes capture it after a few seconds of flight,
//! hands off: `vol-essai-poursuite` (chase view) and `vol-essai-pilote` (pilot's view).

use std::path::Path;

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
use crate::flight::{TEST_FLIGHT, spawn_scenario};

pub const SCENE_CHASE: &str = "vol-essai-poursuite";
pub const SCENE_PILOT: &str = "vol-essai-pilote";

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut hold: ResMut<CaptureHold>,
) -> Result {
    spawn_plain(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut images,
        &mut media,
    );
    let (aircraft, cg) = spawn_scenario(TEST_FLIGHT, &mut commands, &asset_server, &mut hold)?;
    // The pilot's eye of the 3D model's file (its node cam_pilot_eye).
    let assets = Path::new(ASSETS_DIR);
    let id = load_scenario(assets, TEST_FLIGHT)?.aircraft;
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
