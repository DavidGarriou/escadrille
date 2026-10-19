//! Development tools: gamepad lab, capture mode (later: force gizmos, telemetry).

pub mod capture;
pub mod labo_manette;

use bevy::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((capture::CapturePlugin, labo_manette::LaboManettePlugin));
    }
}
