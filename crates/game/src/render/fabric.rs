//! Translucent fabric: doped linen lets the sun through, and seen against the light the
//! ribs and spars show as shadows (plan `m3-modele-spad7.md`, B5). A model's material
//! asks for it in its glTF extras (`diffuse_transmission`, `transmission_depth_m`,
//! written by `art/<id>/build.py`); the share of light let through is the alpha of its
//! base colour texture, as `KHR_materials_diffuse_transmission` stores it.
//!
//! Bevy lights the transmitted side at the surface's position moved `thickness` behind
//! it, and shades it from there: the depth set is past the opposite skin where the wing
//! is thickest, so that a wing does not shade its own fabric, but the other plane or the
//! fuselage still does.

use bevy::gltf::GltfMaterialExtras;
use bevy::prelude::*;

use super::extra_number;

/// Lets light through the materials whose extras ask for it, once each.
pub fn translucent_fabric(
    meshes: Query<
        (&GltfMaterialExtras, &MeshMaterial3d<StandardMaterial>),
        Added<GltfMaterialExtras>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (extras, material) in &meshes {
        let Some(share) = extra_number(&extras.value, "diffuse_transmission") else {
            continue;
        };
        let depth = extra_number(&extras.value, "transmission_depth_m").unwrap_or(0.0);
        // Look before taking it mutably: several meshes share a material.
        let done = materials
            .get(&material.0)
            .is_none_or(|m| m.diffuse_transmission_texture.is_some());
        if done {
            continue;
        }
        let mut m = materials
            .get_mut(&material.0)
            .expect("the material was just found");
        make_translucent(&mut m, share as f32, depth as f32);
    }
}

/// The material lets through `share` of the light, modulated by its colour's alpha.
pub fn make_translucent(material: &mut StandardMaterial, share: f32, depth: f32) {
    material.diffuse_transmission = share;
    material.diffuse_transmission_texture = material.base_color_texture.clone();
    material.thickness = depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_transmission_is_read_from_the_colour_texture() {
        let colour = Handle::<Image>::default();
        let mut material = StandardMaterial {
            base_color_texture: Some(colour.clone()),
            ..default()
        };
        make_translucent(&mut material, 1.0, 0.1058);
        assert_eq!(material.diffuse_transmission, 1.0);
        assert_eq!(material.diffuse_transmission_texture, Some(colour));
        assert!((material.thickness - 0.1058).abs() < 1e-6);
        // Opaque still: the light goes through, not the view.
        assert_eq!(material.alpha_mode, AlphaMode::Opaque);
    }
}
