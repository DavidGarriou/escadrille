//! World: terrain, sky, weather and reference scenes.
//!
//! Reference scenes so far: `m0-sol` checks the capture chain, `maquette-spad7` the
//! frames of the 3D models (ADR 0006).

pub mod maquette;
pub mod scenes;

use bevy::prelude::*;

use crate::states::AppState;
use scenes::capture_scene_is;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(scenes::SKY_COLOR))
            .add_systems(OnEnter(AppState::Vol), scenes::spawn_m0_sol)
            .add_systems(
                OnEnter(AppState::Capture),
                (
                    scenes::spawn_m0_sol.run_if(capture_scene_is("m0-sol")),
                    maquette::spawn.run_if(capture_scene_is(maquette::SCENE)),
                ),
            )
            .add_systems(
                Update,
                maquette::draw_gizmos.run_if(resource_exists::<maquette::Pose>),
            )
            .add_systems(OnEnter(AppState::LaboManette), scenes::spawn_ui_camera);
    }
}
