//! Rendering: how the game's assets are prepared for the GPU (skill `rendu-visuel`).

pub mod mipmaps;

use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mipmaps::fill_mip_chains);
    }
}
