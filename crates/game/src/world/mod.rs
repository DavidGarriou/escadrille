//! World: terrain, sky, weather and reference scenes.
//!
//! Reference scenes so far: `m0-sol` checks the capture chain, `maquette-spad7` the
//! frames of the 3D models (ADR 0006), `avion-studio` and `contre_jour_toile` the SPAD
//! as the game renders it, `vol-essai-poursuite` and `vol-essai-pilote` the test flight.
//! Outside captures, the game starts the test flight (`vol_essai`).

pub mod maquette;
pub mod plain;
pub mod scenes;
pub mod studio;
pub mod vol_essai;

use bevy::prelude::*;

use crate::states::AppState;
use scenes::{capture_scene_among, capture_scene_is};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(scenes::SKY_COLOR))
            .add_systems(OnEnter(AppState::Vol), vol_essai::spawn)
            .add_systems(
                OnEnter(AppState::Capture),
                (
                    scenes::spawn_m0_sol.run_if(capture_scene_is("m0-sol")),
                    maquette::spawn.run_if(capture_scene_is(maquette::SCENE)),
                    studio::spawn.run_if(capture_scene_is(studio::SCENE)),
                    studio::spawn_contre_jour.run_if(capture_scene_is(studio::SCENE_CONTRE_JOUR)),
                    vol_essai::spawn.run_if(capture_scene_among(&[
                        vol_essai::SCENE_CHASE,
                        vol_essai::SCENE_PILOT,
                    ])),
                    vol_essai::pilot_view.run_if(capture_scene_is(vol_essai::SCENE_PILOT)),
                ),
            )
            .add_systems(
                Update,
                maquette::draw_gizmos.run_if(resource_exists::<maquette::Pose>),
            )
            .add_systems(OnEnter(AppState::LaboManette), scenes::spawn_ui_camera);
    }
}
