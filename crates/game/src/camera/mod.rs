//! Views (plan `m2-vol-essai.md`, step 4): chase behind the player's aircraft, or the
//! pilot's eye; the right stick looks around, R3 looks ahead again, west (Y) changes view.

use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::camera::Exposure;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::light::AtmosphereEnvironmentMapLight;
use bevy::pbr::AtmosphereSettings;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

use crate::flight::FlightSet;
use crate::flight::simulation::Player;
use crate::input::{Action, Controls, InputSet};

/// Chase view: distance behind and height above the aircraft, m.
pub const CHASE_DISTANCE_M: f32 = 10.0;
pub const CHASE_HEIGHT_M: f32 = 2.5;
/// Time constant of the chase camera following the aircraft's direction, s.
pub const CHASE_LAG_S: f32 = 0.3;
/// Vertical fields of view, degrees.
pub const CHASE_FOV_DEG: f32 = 50.0;
pub const PILOT_FOV_DEG: f32 = 60.0;
/// Free look in the cockpit: yaw either way, pitch down and up, rad.
pub const PILOT_LOOK_YAW: f32 = 150.0 * std::f32::consts::PI / 180.0;
pub const PILOT_LOOK_DOWN: f32 = 30.0 * std::f32::consts::PI / 180.0;
pub const PILOT_LOOK_UP: f32 = 80.0 * std::f32::consts::PI / 180.0;
/// Exposure of the outdoor scenes in afternoon sun (avion-studio, 2026-10-27).
pub const DAYLIGHT_EV100: f32 = 13.0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSet;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    #[default]
    Chase,
    Pilot,
}

/// The camera that follows the player.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct FlightCamera {
    /// Free look, rad: yaw to the right, pitch up.
    yaw: f32,
    pitch: f32,
    /// Direction the chase camera looks along, following the aircraft's with a lag.
    chase: Option<Vec3>,
}

/// The pilot's eye in the player's aircraft: its offset from the aircraft's entity (the
/// centre of gravity), in the entity's local Bevy axes.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PilotEye(pub Vec3);

/// The player's aircraft as the camera sees it.
type PlayerView<'w, 's> =
    Query<'w, 's, (&'static Transform, &'static PilotEye), (With<Player>, Without<FlightCamera>)>;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<View>().add_systems(
            Update,
            follow_player
                .in_set(CameraSet)
                .after(InputSet)
                .after(FlightSet::Place),
        );
    }
}

/// A camera for outdoor daylight: vertical field `fov_deg`, the atmosphere and its light,
/// temporal anti-aliasing and the game's tone mapping.
pub fn outdoor_camera(transform: Transform, fov_deg: f32, ev100: f32) -> impl Bundle {
    (
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: fov_deg.to_radians(),
            near: 0.05,
            ..default()
        }),
        transform,
        AtmosphereSettings::default(),
        AtmosphereEnvironmentMapLight::default(),
        Exposure { ev100 },
        // Chosen on the studio's captures (2026-10-27): ACES saturates the varnished
        // linen towards orange, AgX washes it out at this exposure.
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        Msaa::Off,
        TemporalAntiAliasing::default(),
    )
}

/// The flight camera, behind the start until the aircraft is placed.
pub fn spawn_flight_camera(commands: &mut Commands) {
    commands.spawn((
        Name::new("Flight camera"),
        FlightCamera::default(),
        outdoor_camera(Transform::default(), CHASE_FOV_DEG, DAYLIGHT_EV100),
    ));
}

fn follow_player(
    time: Res<Time<Real>>,
    mut actions: MessageReader<Action>,
    controls: Res<Controls>,
    mut view: ResMut<View>,
    player: PlayerView,
    mut cameras: Query<(&mut FlightCamera, &mut Transform, &mut Projection)>,
) {
    let dt = time.delta_secs();
    let Ok((aircraft, eye)) = player.single() else {
        return;
    };
    for action in actions.read() {
        match action {
            Action::ChangeView => {
                *view = match *view {
                    View::Chase => View::Pilot,
                    View::Pilot => View::Chase,
                };
                for (mut camera, _, _) in &mut cameras {
                    camera.yaw = 0.0;
                    camera.pitch = 0.0;
                }
            }
            Action::LookAhead => {
                for (mut camera, _, _) in &mut cameras {
                    camera.yaw = 0.0;
                    camera.pitch = 0.0;
                }
            }
            _ => {}
        }
    }
    for (mut camera, mut transform, mut projection) in &mut cameras {
        camera.yaw += controls.look.x * dt;
        camera.pitch += controls.look.y * dt;
        let fov = match *view {
            View::Chase => {
                camera.yaw = camera.yaw.rem_euclid(std::f32::consts::TAU);
                camera.pitch = camera.pitch.clamp(-1.2, 1.2);
                let forward = aircraft.forward().as_vec3();
                let lagged = camera.chase.map_or(forward, |d| {
                    d.lerp(forward, 1.0 - (-dt / CHASE_LAG_S).exp())
                        .normalize_or(forward)
                });
                camera.chase = Some(lagged);
                let look = Quat::from_rotation_y(-camera.yaw)
                    * Quat::from_axis_angle(
                        lagged.cross(Vec3::Y).normalize_or(Vec3::X),
                        camera.pitch,
                    );
                let direction = look * lagged;
                let target = aircraft.translation;
                *transform = Transform::from_translation(
                    target - direction * CHASE_DISTANCE_M + Vec3::Y * CHASE_HEIGHT_M,
                )
                .looking_at(target + Vec3::Y * 0.5, Vec3::Y);
                CHASE_FOV_DEG
            }
            View::Pilot => {
                camera.chase = None;
                camera.yaw = camera.yaw.clamp(-PILOT_LOOK_YAW, PILOT_LOOK_YAW);
                camera.pitch = camera.pitch.clamp(-PILOT_LOOK_DOWN, PILOT_LOOK_UP);
                *transform = *aircraft
                    * Transform::from_translation(eye.0).with_rotation(
                        Quat::from_rotation_y(-camera.yaw) * Quat::from_rotation_x(camera.pitch),
                    );
                PILOT_FOV_DEG
            }
        };
        if let Projection::Perspective(p) = &mut *projection {
            p.fov = fov.to_radians();
        }
    }
}
