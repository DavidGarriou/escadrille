//! World: terrain, sky, weather and reference scenes.
//!
//! Reference scenes so far: `m0-sol` checks the capture chain, `maquette-spad7` the
//! frames of the 3D models (ADR 0006), `avion-studio` and `contre_jour_toile` the SPAD
//! as the game renders it, `degats-spad7` and `degats-spad7-proche` the damage its model can show,
//! `vol-essai-poursuite` and `vol-essai-pilote` the test flight, `terrain-repos`,
//! `terrain-pilote`, `terrain-roulage`, `terrain-roues` and `terrain-finale` the SPAD on and
//! over the field.
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
use scenes::{capture_scene_among, capture_scene_is};

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
                    vol_essai::spawn.run_if(capture_scene_among(&[
                        vol_essai::SCENE_CHASE,
                        vol_essai::SCENE_PILOT,
                    ])),
                    vol_essai::spawn_on_the_field.run_if(capture_scene_among(&[
                        vol_essai::SCENE_FIELD,
                        vol_essai::SCENE_FIELD_PILOT,
                        vol_essai::SCENE_WHEELS,
                    ])),
                    vol_essai::spawn_rolling.run_if(capture_scene_is(vol_essai::SCENE_ROLL)),
                    vol_essai::spawn_on_final.run_if(capture_scene_is(vol_essai::SCENE_FINAL)),
                    vol_essai::pilot_view.run_if(capture_scene_among(&[
                        vol_essai::SCENE_PILOT,
                        vol_essai::SCENE_FIELD_PILOT,
                        vol_essai::SCENE_FINAL,
                    ])),
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
                    vol_essai::look_at_the_wheels.run_if(capture_scene_is(vol_essai::SCENE_WHEELS)),
                ),
            )
            .add_systems(OnEnter(AppState::LaboManette), scenes::spawn_ui_camera);
    }
}
