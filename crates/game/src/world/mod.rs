//! World: terrain, sky, weather and reference scenes.
//!
//! M0 only provides the `m0-sol` reference scene used to validate the capture chain.

pub mod scenes;

use bevy::prelude::*;

use crate::states::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(scenes::SKY_COLOR))
            .add_systems(OnEnter(AppState::Vol), scenes::spawn_m0_sol)
            // `m0-sol` is the only known scene; a scene registry comes with the second one.
            .add_systems(OnEnter(AppState::Capture), scenes::spawn_m0_sol)
            .add_systems(OnEnter(AppState::LaboManette), scenes::spawn_ui_camera);
    }
}
