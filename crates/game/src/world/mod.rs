//! World: terrain, sky, weather and reference scenes.
//!
//! Reference scenes built in code: `m0-sol` checks the capture chain, `maquette-spad7` the
//! frames of the 3D models (ADR 0006), `avion-studio` and `contre_jour_toile` the SPAD
//! as the game renders it, `degats-spad7` and `degats-spad7-proche` the damage its model
//! can show. The scenes of a flight are files of `assets/scenes` (`scenes`).
//! Outside captures, the game starts the test flight, on the field (`vol_essai`).

pub mod degats;
pub mod maquette;
pub mod meadow;
pub mod plain;
pub mod scenes;
pub mod studio;
pub mod vol_essai;

use bevy::prelude::*;

use crate::flight::FlightSet;
use crate::flight::simulation::RenderOrigin;
use crate::states::AppState;
use scenes::{capture_from_file, capture_scene_is};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(scenes::SKY_COLOR))
            .add_systems(OnEnter(AppState::Vol), vol_essai::spawn_on_the_field)
            .add_systems(
                OnEnter(AppState::Capture),
                (
                    scenes::spawn_m0_sol.run_if(capture_scene_is("m0-sol")),
                    maquette::spawn.run_if(capture_scene_is(maquette::SCENE)),
                    studio::spawn.run_if(capture_scene_is(studio::SCENE)),
                    studio::spawn_contre_jour.run_if(capture_scene_is(studio::SCENE_CONTRE_JOUR)),
                    degats::spawn.run_if(capture_scene_is(degats::SCENE)),
                    degats::spawn_close.run_if(capture_scene_is(degats::SCENE_CLOSE)),
                    vol_essai::spawn_scene_file.run_if(capture_from_file),
                ),
            )
            .add_systems(
                Update,
                (
                    maquette::draw_gizmos.run_if(resource_exists::<maquette::Pose>),
                    meadow::place_meadow
                        .after(FlightSet::Place)
                        .run_if(resource_changed::<RenderOrigin>),
                    meadow::fade_tufts.after(FlightSet::Place),
                ),
            )
            .add_systems(OnEnter(AppState::LaboManette), scenes::spawn_ui_camera);
    }
}
