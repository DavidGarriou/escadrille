//! Reference scenes of the SPAD as the game renders it, with the atmosphere, shadows,
//! temporal anti-aliasing and tone mapping (plan `m3-modele-spad7.md`, B6):
//! - `avion-studio`: at rest on its wheels and tail skid, seen three-quarter front from a
//!   standing man's eye, in afternoon sun: the model, its materials and textures;
//! - `contre_jour_toile`: flying overhead against the sun: the translucent fabric.
//!
//! The gallery of damage (`degats-spad7`, module `degats`) stands it on the grass the
//! same way, seen from its right.

use std::path::Path;

use bevy::light::atmosphere::ScatteringMedium;
use bevy::light::{Atmosphere, CascadeShadowConfigBuilder, SunDisk, light_consts::lux};
use bevy::math::{DQuat, DVec3, EulerRot};
use bevy::prelude::*;
use escadrille_data::aircraft::{Loading, load_aircraft};
use escadrille_data::mockup::load_mockup;

use super::atmosphere;
use crate::ASSETS_DIR;
use crate::camera::{PILOT_FOV_DEG, outdoor_camera};
use crate::dev::capture::CaptureHold;
use crate::flight::bridge::{model_transform, render_transform, to_bevy};

pub const SCENE: &str = "avion-studio";
pub const SCENE_CONTRE_JOUR: &str = "contre_jour_toile";
pub const SCENE_IN_THE_SUN: &str = "avion_dans_le_soleil";

const AIRCRAFT: &str = "spad7";

/// Heading of the aircraft, north.
const HEADING_DEG: f64 = 0.0;

/// Height of the eye of a man standing on the grass, m.
const EYE_HEIGHT_M: f64 = 1.65;

/// Where the camera is and looks, and where the sun shines from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct View {
    pub camera: Eye,
    /// The sun's azimuth, from north towards east, and elevation, degrees.
    pub sun_azimuth_deg: f64,
    pub sun_elevation_deg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Eye {
    /// On the grass, looking at a point of the aircraft.
    Standing {
        /// The camera's foot, from the centre of gravity (NED, m; its z is ignored).
        from_cg: DVec3,
        /// Height of the camera above the grass, m.
        height_m: f64,
        /// Point looked at, from the centre of gravity (NED, m).
        target_from_cg: DVec3,
    },
    /// In the cockpit, at the pilot's eye (`pilot_eye_m` of `maquette.ron`), looking
    /// ahead level with the ground, over the nose of the aircraft at rest.
    Pilot,
}

/// `avion-studio`: ahead and to the left, at the eye of a man standing on the grass,
/// looking at the middle of the wings' gap; the sun from the west-north-west, behind the
/// camera's left shoulder, 35° high: a summer afternoon over Flanders.
pub const STUDIO: View = View {
    camera: Eye::Standing {
        from_cg: DVec3::new(6.4, -6.2, 0.0),
        height_m: EYE_HEIGHT_M,
        target_from_cg: DVec3::new(-0.4, 0.0, -0.3),
    },
    sun_azimuth_deg: -70.0,
    sun_elevation_deg: 35.0,
};

/// Pitch of the aircraft resting on its wheels and tail skid (rad, nose up positive):
/// the angle of the line through the bottom of the wheels and the skid's shoe.
pub fn rest_pitch(wheel_bottom: DVec3, skid: DVec3) -> f64 {
    ((wheel_bottom.z - skid.z) / (wheel_bottom.x - skid.x)).atan()
}

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
        STUDIO,
    )?;
    commands
        .entity(model)
        .observe(super::maquette::turn_the_propeller);
    Ok(())
}

/// The SPAD at rest on the grass, seen from `view`: the model's entity, its propeller
/// still to be set.
pub fn spawn_at_rest(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    media: &mut Assets<ScatteringMedium>,
    hold: &mut CaptureHold,
    view: View,
) -> Result<Entity> {
    let assets = Path::new(ASSETS_DIR);
    let mockup = load_mockup(assets, AIRCRAFT)?.ok_or("the SPAD has no maquette.ron")?;
    let cg = centre_of_gravity()?;
    let wheel =
        DVec3::from_array(mockup.wheels.centre_m) + DVec3::Z * (0.5 * mockup.wheels.diameter.si());
    let skid = DVec3::from_array(mockup.skid_m);
    let attitude = DQuat::from_euler(
        EulerRot::ZYX,
        HEADING_DEG.to_radians(),
        rest_pitch(wheel, skid),
        0.0,
    );
    // The wheels' bottom on the ground (z = 0, down positive).
    let position = DVec3::new(0.0, 0.0, -(attitude * (wheel - cg)).z);
    let model = spawn_spad(commands, asset_server, hold, position, attitude, cg);
    spawn_outdoors(
        commands,
        meshes,
        materials,
        media,
        sun_direction(view.sun_azimuth_deg, view.sun_elevation_deg),
    );
    let at = |from_cg: DVec3| (to_bevy(position + from_cg)).as_vec3();
    let camera = match view.camera {
        Eye::Standing {
            from_cg,
            height_m,
            target_from_cg,
        } => {
            let eye = at(from_cg) * Vec3::new(1.0, 0.0, 1.0) + Vec3::Y * height_m as f32;
            Transform::from_translation(eye).looking_at(at(target_from_cg), Vec3::Y)
        }
        Eye::Pilot => {
            let eye = attitude * (DVec3::from_array(mockup.pilot_eye_m) - cg);
            // Level with the ground: the aircraft's heading, not its pitch.
            let ahead = DQuat::from_rotation_z(HEADING_DEG.to_radians()) * DVec3::X;
            Transform::from_translation(at(eye)).looking_at(at(eye + ahead), Vec3::Y)
        }
    };
    // The cockpit view's field, as in flight.
    let fov = match view.camera {
        Eye::Standing { .. } => 40.0,
        Eye::Pilot => PILOT_FOV_DEG,
    };
    commands.spawn(outdoor_camera(camera, fov, 13.0));
    Ok(model)
}

/// `contre_jour_toile` (skill `rendu-visuel`): the SPAD flying over a man standing on the
/// grass, the sun behind its wing: the fabric glows with the light it lets through, the
/// ribs, spars and painted marks dark across it. Hidden by the wing, the sun leaves no
/// glare veil (art director's review of 2026-11-11: with the sun 9° beyond the aircraft,
/// the veil drowned it; that view is now `avion_dans_le_soleil`).
pub fn spawn_contre_jour(
    commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    media: ResMut<Assets<ScatteringMedium>>,
    hold: ResMut<CaptureHold>,
) -> Result {
    let sun = sun_direction(CONTRE_JOUR_SUN.0, CONTRE_JOUR_SUN.1);
    spawn_overhead(commands, asset_server, meshes, materials, media, hold, sun)
}

/// The sun behind the wing of `contre_jour_toile`: azimuth and elevation, degrees.
const CONTRE_JOUR_SUN: (f64, f64) = (9.0, 45.0);

/// `avion_dans_le_soleil`: the same SPAD, the sun 9° beyond it, 50° high: the aircraft
/// lost in the sun's glare veil, the attack from the sun that the pilots feared.
pub fn spawn_in_the_sun(
    commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    media: ResMut<Assets<ScatteringMedium>>,
    hold: ResMut<CaptureHold>,
) -> Result {
    let sun = sun_direction(0.0, 50.0);
    spawn_overhead(commands, asset_server, meshes, materials, media, hold, sun)
}

/// The SPAD flying over a man standing on the grass, seen from below, with the sun at
/// `towards_sun`.
fn spawn_overhead(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut hold: ResMut<CaptureHold>,
    towards_sun: DVec3,
) -> Result {
    let cg = centre_of_gravity()?;
    // Heading north-east, banked 10° to the right, 13 m up and 15 m north of the man:
    // three-quarter from below, the nose towards him.
    let attitude = DQuat::from_euler(EulerRot::ZYX, 45f64.to_radians(), 0.0, 10f64.to_radians());
    let position = DVec3::new(15.0, 0.0, -(13.0 + EYE_HEIGHT_M));
    let model = spawn_spad(
        &mut commands,
        &asset_server,
        &mut hold,
        position,
        attitude,
        cg,
    );
    commands
        .entity(model)
        .observe(super::maquette::turn_the_propeller);
    // Seen 41° high to the north.
    spawn_outdoors(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut media,
        towards_sun,
    );
    let eye = Vec3::Y * EYE_HEIGHT_M as f32;
    commands.spawn(outdoor_camera(
        Transform::from_translation(eye).looking_at(to_bevy(position).as_vec3(), Vec3::Y),
        // A longer lens: the aircraft fills the frame from 20 m.
        24.0,
        13.0,
    ));
    Ok(())
}

fn centre_of_gravity() -> Result<DVec3> {
    let data = load_aircraft(Path::new(ASSETS_DIR), AIRCRAFT)?;
    Ok(data
        .build(&Loading { fuel_fraction: 1.0 })?
        .summary
        .cg_from_datum)
}

/// Direction towards the sun (NED) from its azimuth (from north, towards east) and
/// elevation, degrees.
fn sun_direction(azimuth_deg: f64, elevation_deg: f64) -> DVec3 {
    let (az, el) = (azimuth_deg.to_radians(), elevation_deg.to_radians());
    DVec3::new(el.cos() * az.cos(), el.cos() * az.sin(), -el.sin())
}

/// The model, its entity at the centre of gravity as the flight plugin will place it:
/// the entity of the model's root.
fn spawn_spad(
    commands: &mut Commands,
    asset_server: &AssetServer,
    hold: &mut CaptureHold,
    position_ned: DVec3,
    attitude: DQuat,
    cg: DVec3,
) -> Entity {
    hold.0 += 1;
    let model = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset(format!("aircraft/{AIRCRAFT}/model.glb")));
    let aircraft = commands
        .spawn((
            Name::new("SPAD VII"),
            render_transform(position_ned, attitude, DVec3::ZERO),
            Visibility::default(),
        ))
        .id();
    commands
        .spawn((
            Name::new("Model"),
            WorldAssetRoot(model),
            model_transform(cg),
            ChildOf(aircraft),
        ))
        .id()
}

/// Grass, the atmosphere and the sun, its shadows sharp near the aircraft.
fn spawn_outdoors(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    media: &mut Assets<ScatteringMedium>,
    towards_sun: DVec3,
) {
    commands.spawn((
        Name::new("Grass"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(4000.0, 4000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.38, 0.20),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    commands.spawn(Atmosphere::earth(
        media.add(atmosphere::medium(atmosphere::Haze::default())),
    ));
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            illuminance: lux::RAW_SUNLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        SunDisk::EARTH,
        // The first cascade over the aircraft's length.
        CascadeShadowConfigBuilder {
            num_cascades: 2,
            minimum_distance: 0.5,
            first_cascade_far_bound: 20.0,
            maximum_distance: 80.0,
            ..default()
        }
        .build(),
        Transform::default().looking_to(-to_bevy(towards_sun).as_vec3(), Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_spad_rests_tail_down_by_fourteen_degrees() {
        // The 1917 drawing: about 14° tail down on the ground (maquette.ron, skid_m).
        let assets = Path::new(ASSETS_DIR);
        let m = load_mockup(assets, AIRCRAFT).unwrap().unwrap();
        let wheel =
            DVec3::from_array(m.wheels.centre_m) + DVec3::Z * (0.5 * m.wheels.diameter.si());
        let pitch = rest_pitch(wheel, DVec3::from_array(m.skid_m)).to_degrees();
        assert!((13.5..15.5).contains(&pitch), "{pitch}°");
    }
}
