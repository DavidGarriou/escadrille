//! Development tools: gamepad lab, capture mode, force gizmos.

pub mod capture;
pub mod forces;
pub mod labo_manette;

use bevy::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((capture::CapturePlugin, labo_manette::LaboManettePlugin))
            .init_resource::<forces::ForceGizmos>()
            .add_systems(
                Update,
                (
                    forces::toggle,
                    forces::draw.run_if(|shown: Res<forces::ForceGizmos>| shown.0),
                ),
            );
    }
}
