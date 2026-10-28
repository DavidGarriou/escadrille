//! The 3D model's moving parts follow the flight model: each control surface turns about
//! its hinge by its deflection (the node's local +X, model contract), the propeller about
//! its axis at the shaft's speed; above [`BLUR_RPM`] its blades give way to the blurred
//! disc, which the eye sees instead.

use bevy::ecs::system::SystemParam;
use bevy::math::Quat;
use bevy::prelude::*;
use bevy::world_serialization::WorldInstanceReady;
use escadrille_data::aircraft::Control;

use super::simulation::Simulated;
use crate::dev::capture::CaptureHold;

/// Shaft speed above which the blades are drawn as their blurred disc, rpm.
pub const BLUR_RPM: f64 = 400.0;

/// Opacity of the blurred disc: the blades sweep about a tenth of it at any instant, and
/// the eye keeps a faint trace of them.
pub const DISC_ALPHA: f32 = 0.18;

/// The control nodes of the model contract, with the command that moves each.
#[derive(Component, Debug, Clone, Default)]
pub struct ModelNodes {
    pub controls: Vec<(String, Control)>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ControlSurface {
    aircraft: Entity,
    control: Control,
    rest: Quat,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct PropellerBlades {
    aircraft: Entity,
    rest: Quat,
    angle: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct PropellerDisc {
    aircraft: Entity,
}

/// The meshes of the model and their materials, to make the propeller's disc translucent.
#[derive(SystemParam)]
pub struct ModelMaterials<'w, 's> {
    meshes: Query<'w, 's, &'static MeshMaterial3d<StandardMaterial>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
}

/// Once the model is spawned: finds its moving nodes, makes the propeller's disc
/// translucent, and lets the capture go on.
pub fn prepare_model(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    models: Query<(&ModelNodes, &ChildOf)>,
    children: Query<&Children>,
    nodes: Query<(&Name, &Transform)>,
    mut model_materials: ModelMaterials,
    hold: Option<ResMut<CaptureHold>>,
) {
    let Ok((model, parent)) = models.get(ready.entity) else {
        return;
    };
    let aircraft = parent.parent();
    for entity in children.iter_descendants(ready.entity) {
        let Ok((name, transform)) = nodes.get(entity) else {
            continue;
        };
        let rest = transform.rotation;
        if let Some((_, control)) = model.controls.iter().find(|(n, _)| n == name.as_str()) {
            commands.entity(entity).insert(ControlSurface {
                aircraft,
                control: *control,
                rest,
            });
        }
        match name.as_str() {
            "prop" => {
                commands.entity(entity).insert(PropellerBlades {
                    aircraft,
                    rest,
                    angle: 0.0,
                });
            }
            "prop_blur" => {
                commands.entity(entity).insert(PropellerDisc { aircraft });
                for mesh in children.iter_descendants(entity) {
                    let Some(base) = model_materials
                        .meshes
                        .get(mesh)
                        .ok()
                        .and_then(|m| model_materials.materials.get(&m.0))
                        .cloned()
                    else {
                        continue;
                    };
                    let disc = model_materials.materials.add(StandardMaterial {
                        base_color: base.base_color.with_alpha(DISC_ALPHA),
                        alpha_mode: AlphaMode::Blend,
                        ..base
                    });
                    commands.entity(mesh).insert(MeshMaterial3d(disc));
                }
            }
            _ => {}
        }
    }
    if let Some(mut hold) = hold {
        hold.0 = hold.0.saturating_sub(1);
    }
}

/// Each control surface at its deflection.
pub fn move_controls(
    aircraft: Query<&Simulated>,
    mut surfaces: Query<(&ControlSurface, &mut Transform)>,
) {
    for (surface, mut transform) in &mut surfaces {
        let Ok(sim) = aircraft.get(surface.aircraft) else {
            continue;
        };
        let d = sim.aircraft.deflections(&sim.controls);
        let angle = match surface.control {
            Control::Aileron => d.aileron,
            Control::Elevator => d.elevator,
            Control::Rudder => d.rudder,
        };
        transform.rotation = surface.rest * Quat::from_rotation_x(angle as f32);
    }
}

/// The propeller turned at the shaft's speed; the blades or the disc shown.
pub fn turn_propeller(
    time: Res<Time>,
    aircraft: Query<&Simulated>,
    mut blades: Query<
        (&mut PropellerBlades, &mut Transform, &mut Visibility),
        Without<PropellerDisc>,
    >,
    mut discs: Query<(&PropellerDisc, &mut Visibility), Without<PropellerBlades>>,
) {
    let rpm = |entity: Entity| {
        aircraft.get(entity).map_or(0.0, |sim| {
            sim.state.rotor_speed * 60.0 / std::f64::consts::TAU
        })
    };
    for (mut prop, mut transform, mut visibility) in &mut blades {
        let speed = rpm(prop.aircraft) * std::f64::consts::TAU / 60.0;
        prop.angle = (prop.angle + (speed as f32) * time.delta_secs()) % std::f32::consts::TAU;
        transform.rotation = prop.rest * Quat::from_rotation_x(prop.angle);
        *visibility = if rpm(prop.aircraft) < BLUR_RPM {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for (disc, mut visibility) in &mut discs {
        *visibility = if rpm(disc.aircraft) < BLUR_RPM {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}
