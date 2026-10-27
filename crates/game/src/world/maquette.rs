//! `maquette-spad7`: the provisional SPAD model in a known attitude, with the flight
//! model's axes drawn over it, to check ADR 0006 by eye (step 5 of
//! `docs/plans/modele-3d-provisoire.md`). On the capture:
//! - the nose is on the red arrow (body x);
//! - the right wing, green, is on the side of the green arrow (body y) and lower;
//! - the blue arrow (body z) points down through the floor of the fuselage;
//! - the propeller, turned from its rest position (blades vertical), has its top blade
//!   on the side the yellow arrow turns to: clockwise seen from behind for the SPAD.
//!
//! The model is placed as the flight plugin will place it: the aircraft's entity at the
//! centre of gravity by [`render_transform`], the model under it by [`model_transform`].

use std::path::Path;

use bevy::color::palettes::css;
use bevy::gizmos::config::{DefaultGizmoConfigGroup, GizmoConfigStore};
use bevy::light::light_consts;
use bevy::math::{DQuat, DVec3, EulerRot};
use bevy::prelude::*;
use bevy::world_serialization::WorldInstanceReady;
use escadrille_data::aircraft::{Loading, load_aircraft};
use escadrille_data::model::contract;

use crate::ASSETS_DIR;
use crate::dev::capture::CaptureHold;
use crate::flight::bridge::{model_transform, render_transform, to_bevy};

pub const SCENE: &str = "maquette-spad7";

const AIRCRAFT: &str = "spad7";

/// Heading east, level, banked 30° to the right: the right wing is the lower one.
const HEADING_DEG: f64 = 90.0;
const BANK_DEG: f64 = 30.0;

/// Centre of gravity 6 m above the ground (NED, z down), at the render origin.
const POSITION_NED: DVec3 = DVec3::new(0.0, 0.0, -6.0);

/// Turn of the propeller from its rest position, positive about its axis: the way it
/// turns in flight.
const PROP_TURN_DEG: f64 = 30.0;

/// Length of the arrows of the body axes, m.
const AXIS_LENGTH_M: f64 = 3.0;

/// Where the model is, for the gizmos.
#[derive(Resource, Debug, Clone, Copy)]
pub struct Pose {
    /// Bevy transform of the aircraft's entity (at the centre of gravity).
    aircraft: Transform,
    cg_from_datum: DVec3,
    hub: DVec3,
    axis: DVec3,
    radius: f64,
}

impl Pose {
    /// A point of the model (body axes, from the datum) in Bevy world axes.
    fn point(&self, from_datum: DVec3) -> Vec3 {
        self.aircraft
            .transform_point(to_bevy(from_datum - self.cg_from_datum).as_vec3())
    }

    /// A body direction in Bevy world axes.
    fn direction(&self, body: DVec3) -> Vec3 {
        self.aircraft.rotation * to_bevy(body).as_vec3()
    }
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hold: ResMut<CaptureHold>,
    mut gizmos: ResMut<GizmoConfigStore>,
) -> Result {
    let data = load_aircraft(Path::new(ASSETS_DIR), AIRCRAFT)?;
    let cg_from_datum = data
        .build(&Loading { fuel_fraction: 1.0 })?
        .summary
        .cg_from_datum;
    let propeller = contract(&data)?.propeller;
    let attitude = DQuat::from_euler(
        EulerRot::ZYX,
        HEADING_DEG.to_radians(),
        0.0,
        BANK_DEG.to_radians(),
    );
    let pose = Pose {
        aircraft: render_transform(POSITION_NED, attitude, DVec3::ZERO),
        cg_from_datum,
        hub: propeller.hub,
        axis: propeller.axis,
        radius: propeller.radius,
    };
    commands.insert_resource(pose);

    hold.0 += 1;
    let model = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset(format!("aircraft/{AIRCRAFT}/model.glb")));
    commands
        .spawn((Name::new("SPAD VII"), pose.aircraft, Visibility::default()))
        .with_children(|aircraft| {
            aircraft
                .spawn((
                    Name::new("Model"),
                    WorldAssetRoot(model),
                    model_transform(cg_from_datum),
                ))
                .observe(turn_the_propeller);
        });

    // Drawn over the model: the axes start at the centre of gravity, inside the fuselage.
    let (config, _) = gizmos.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;
    config.line.width = 4.0;

    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(400.0, 400.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.32, 0.40, 0.22))),
    ));
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-20.0, 50.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Behind, to the left and above: the nose ahead, the right wing far and low.
    let target = pose.aircraft.translation;
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_translation(target + Vec3::new(-10.0, 3.5, -5.5))
            .looking_at(target + Vec3::new(0.8, -0.3, 0.0), Vec3::Y),
    ));
    Ok(())
}

/// Once the model is spawned: turns the propeller by [`PROP_TURN_DEG`] about its local +X
/// (the axis of the model contract) and hides its disc, which would cover the blades.
pub(super) fn turn_the_propeller(
    ready: On<WorldInstanceReady>,
    children: Query<&Children>,
    mut nodes: Query<(&Name, &mut Transform, &mut Visibility)>,
    mut hold: ResMut<CaptureHold>,
) {
    for entity in children.iter_descendants(ready.entity) {
        let Ok((name, mut transform, mut visibility)) = nodes.get_mut(entity) else {
            continue;
        };
        match name.as_str() {
            "prop" => transform.rotate_local_x((PROP_TURN_DEG as f32).to_radians()),
            "prop_blur" => *visibility = Visibility::Hidden,
            _ => {}
        }
    }
    hold.0 = hold.0.saturating_sub(1);
}

/// Body axes at the centre of gravity, and the sense of rotation of the propeller: an
/// arc a little behind its disc, positive about its axis, from just past the top blade.
pub fn draw_gizmos(pose: Res<Pose>, mut gizmos: Gizmos) {
    let cg = pose.aircraft.translation;
    for (axis, color) in [
        (DVec3::X, css::RED),
        (DVec3::Y, css::LIME),
        (DVec3::Z, css::BLUE),
    ] {
        gizmos
            .arrow(cg, cg + pose.direction(axis * AXIS_LENGTH_M), color)
            .with_tip_length(0.35);
    }

    let up = DVec3::NEG_Z;
    let side = pose.axis.cross(up);
    let radius = pose.radius * 1.12;
    let at = |deg: f64| {
        let a = deg.to_radians();
        pose.point(pose.hub - pose.axis * 0.20 + (up * a.cos() + side * a.sin()) * radius)
    };
    let (from, to) = (PROP_TURN_DEG + 15.0, PROP_TURN_DEG + 105.0);
    let centre = pose.point(pose.hub - pose.axis * 0.20);
    gizmos
        .short_arc_3d_between(centre, at(from), at(to), css::YELLOW)
        .resolution(32);
    gizmos
        .arrow(at(to - 10.0), at(to), css::YELLOW)
        .with_tip_length(0.30);
}
