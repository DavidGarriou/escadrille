//! `degats-spad7`: a gallery of the damage the 3D model is ready to show (plan
//! `docs/plans/m4-degats-modele-3d.md`, step 6; catalogue `docs/gdd/combat-degats.md`
//! § 3 bis). The SPAD stands on the grass as in `avion-studio`, seen from its right
//! (`degats-spad7`) and from close to its nose (`degats-spad7-proche`), and its right side
//! has taken it all:
//! - the upper wing's middle panel has lost its fabric: its ribs and spars show;
//! - the lower wing's tip panel is gone: the stub of its spars stays;
//! - the outer front strut is broken in two;
//! - flying, landing and incidence wires are cut, their halves streaming aft;
//! - the upper blade of the propeller is broken;
//! - the windscreen is holed by two bullets, its laminated glass crushed and cracked.
//!
//! The game hides the damaged variants as the model spawns (`render::damage`); this
//! scene shows some of them in place of the parts they stand for.

use bevy::gltf::GltfMaterialExtras;
use bevy::light::atmosphere::ScatteringMedium;
use bevy::prelude::*;
use bevy::world_serialization::WorldInstanceReady;

use bevy::math::DVec3;

use super::studio::{Eye, View, spawn_at_rest};
use crate::dev::capture::CaptureHold;
use crate::render::extra_number;

pub const SCENE: &str = "degats-spad7";
pub const SCENE_CLOSE: &str = "degats-spad7-proche";

/// Each part lost, and the damaged variant shown in its place (`""`: nothing).
pub const REPLACED: &[(&str, &str)] = &[
    ("wing_lower_r_tip", "dmg_wing_lower_r_tip_stub"),
    ("strut_r_outer_front", "dmg_strut_r_outer_front_broken"),
    ("wire_flying_r_1", "dmg_wire_flying_r_1_cut"),
    ("wire_flying_r_2", "dmg_wire_flying_r_2_cut"),
    ("wire_landing_r_1", "dmg_wire_landing_r_1_cut"),
    ("wire_incidence_r_1", "dmg_wire_incidence_r_1_cut"),
    ("prop_blade_1", "dmg_prop_blade_1_stub"),
    ("windscreen", "dmg_windscreen_broken"),
];

/// The propeller turned further than the studio's, so that the broken blade stands out
/// of the cowl's dark face, against the light cowl and the sky.
const PROP_TURN_DEG: f32 = -70.0;

/// The panel whose fabric is gone.
pub const BARE: &str = "wing_upper_r_mid";

/// `degats-spad7`: ahead and to the right, low, the bare frame against the sky; the sun
/// low (28°) behind the camera's right shoulder, so that the breaks show their relief.
/// Fixed once reviewed: the before and after of the M4.
pub const GALLERY: View = View {
    camera: Eye::Standing {
        from_cg: DVec3::new(5.4, 5.6, 0.0),
        height_m: 1.5,
        target_from_cg: DVec3::new(-0.4, 1.3, -0.6),
    },
    sun_azimuth_deg: 55.0,
    sun_elevation_deg: 28.0,
};

/// `degats-spad7-proche`: what the pilot sees, the windscreen hit in front of his eyes,
/// the broken blade beyond the cowl; the sun low on his right. (From outside, the
/// cabane struts, the gun and the upper wing hide the small windscreen.)
pub const CLOSE: View = View {
    camera: Eye::Pilot,
    sun_azimuth_deg: 40.0,
    sun_elevation_deg: 28.0,
};

/// `degats-spad7`.
pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut hold: ResMut<CaptureHold>,
) -> Result {
    let model = spawn_at_rest(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &mut media,
        &mut hold,
        GALLERY,
    )?;
    show(&mut commands, model);
    Ok(())
}

/// `degats-spad7-proche`.
pub fn spawn_close(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut hold: ResMut<CaptureHold>,
) -> Result {
    let model = spawn_at_rest(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &mut media,
        &mut hold,
        CLOSE,
    )?;
    show(&mut commands, model);
    Ok(())
}

/// The propeller stopped, the damage shown, once the model is spawned.
fn show(commands: &mut Commands, model: Entity) {
    commands
        .entity(model)
        .observe(super::maquette::turn_the_propeller)
        .observe(show_the_damage);
}

/// Once the model is spawned: the parts lost hidden, their variants shown, the bare
/// panel's fabric hidden (the meshes of its material that lets light through). By
/// commands, applied after every observer: after the game has hidden the variants.
fn show_the_damage(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    names: Query<&Name>,
    fabric: Query<&GltfMaterialExtras>,
    mut transforms: Query<&mut Transform>,
) {
    for entity in children.iter_descendants(ready.entity) {
        let Ok(name) = names.get(entity) else {
            continue;
        };
        let name = name.as_str();
        if name == "prop"
            && let Ok(mut transform) = transforms.get_mut(entity)
        {
            transform.rotate_local_x(PROP_TURN_DEG.to_radians());
        }
        if REPLACED.iter().any(|(part, _)| *part == name) {
            commands.entity(entity).insert(Visibility::Hidden);
        } else if REPLACED.iter().any(|(_, variant)| *variant == name) {
            commands.entity(entity).insert(Visibility::Inherited);
        } else if name == BARE {
            for mesh in children.iter_descendants(entity) {
                let translucent = fabric
                    .get(mesh)
                    .is_ok_and(|e| extra_number(&e.value, "diffuse_transmission").is_some());
                if translucent {
                    commands.entity(mesh).insert(Visibility::Hidden);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::damage::hidden_at_spawn;

    #[test]
    fn each_part_lost_has_a_hidden_variant() {
        for (part, variant) in REPLACED {
            assert!(!hidden_at_spawn(part), "{part}");
            assert!(hidden_at_spawn(variant), "{variant}");
            assert!(variant.contains(part), "{part} → {variant}");
        }
        assert!(!hidden_at_spawn(BARE));
    }
}
