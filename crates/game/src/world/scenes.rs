//! Reference scenes, rendered by the capture mode and reviewed by Claude.
//!
//! Scenes will move to `assets/scenes/<scene>.ron` once there is more than one;
//! for now `m0-sol` is built in code.

use bevy::light::light_consts;
use bevy::prelude::*;

/// Names accepted by `--capture`.
pub const KNOWN_SCENES: &[&str] = &["m0-sol"];

/// Plain overcast-blue sky, until the atmosphere arrives (M3).
pub const SKY_COLOR: Color = Color::srgb(0.62, 0.72, 0.82);

/// `m0-sol`: a ground plane and size markers (1 m cube, 10 m post), lit by a sun.
///
/// It checks the capture chain, scale and lighting conventions (Y up, metres),
/// not beauty. No historical dimension is used here on purpose.
pub fn spawn_m0_sol(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(400.0, 400.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.32, 0.40, 0.22))),
    ));

    let marker = materials.add(Color::srgb(0.85, 0.82, 0.74));
    commands.spawn((
        Name::new("Marker 1 m"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(marker.clone()),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        Name::new("Marker 10 m"),
        Mesh3d(meshes.add(Cuboid::new(0.3, 10.0, 0.3))),
        MeshMaterial3d(marker),
        Transform::from_xyz(6.0, 5.0, -4.0),
    ));

    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(30.0, 50.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(-14.0, 5.0, 20.0).looking_at(Vec3::new(2.0, 4.0, -2.0), Vec3::Y),
    ));
}

/// The gamepad lab only needs a camera for egui.
pub fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((Name::new("UI camera"), Camera2d));
}
