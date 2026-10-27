//! Reference scenes of the SPAD as the game renders it, with the atmosphere, shadows,
//! temporal anti-aliasing and tone mapping (plan `m3-modele-spad7.md`, B6):
//! - `avion-studio`: at rest on its wheels and tail skid, seen three-quarter front from a
//!   standing man's eye, in afternoon sun: the model, its materials and textures;
//! - `contre_jour_toile`: flying overhead against the sun: the translucent fabric.

use std::path::Path;

use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::camera::Exposure;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::light::atmosphere::ScatteringMedium;
use bevy::light::{
    Atmosphere, AtmosphereEnvironmentMapLight, CascadeShadowConfigBuilder, SunDisk,
    light_consts::lux,
};
use bevy::math::{DQuat, DVec3, EulerRot};
use bevy::pbr::AtmosphereSettings;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use escadrille_data::aircraft::{Loading, load_aircraft};
use escadrille_data::mockup::load_mockup;

use crate::ASSETS_DIR;
use crate::dev::capture::CaptureHold;
use crate::flight::bridge::{model_transform, render_transform, to_bevy};

pub const SCENE: &str = "avion-studio";
pub const SCENE_CONTRE_JOUR: &str = "contre_jour_toile";

const AIRCRAFT: &str = "spad7";

/// Heading of the aircraft, north.
const HEADING_DEG: f64 = 0.0;

/// The camera, from the centre of gravity (NED, m): ahead and to the left, at the eye of
/// a man standing on the grass.
const CAMERA_FROM_CG: DVec3 = DVec3::new(6.4, -6.2, 0.0);
const EYE_HEIGHT_M: f64 = 1.65;

/// Point looked at, from the centre of gravity (NED, m): the middle of the wings' gap.
const TARGET_FROM_CG: DVec3 = DVec3::new(-0.4, 0.0, -0.3);

/// Sun from the west-north-west, behind the camera's left shoulder, 35° high: the
/// light of a summer afternoon over Flanders.
const SUN_AZIMUTH_DEG: f64 = -70.0;
const SUN_ELEVATION_DEG: f64 = 35.0;

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
    spawn_spad(
        &mut commands,
        &asset_server,
        &mut hold,
        position,
        attitude,
        cg,
    );
    spawn_outdoors(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut media,
        sun_direction(SUN_AZIMUTH_DEG, SUN_ELEVATION_DEG),
    );
    let at = |from_cg: DVec3| (to_bevy(position + from_cg)).as_vec3();
    let eye = at(CAMERA_FROM_CG) * Vec3::new(1.0, 0.0, 1.0) + Vec3::Y * EYE_HEIGHT_M as f32;
    commands.spawn(camera(
        Transform::from_translation(eye).looking_at(at(TARGET_FROM_CG), Vec3::Y),
        40.0,
        13.0,
    ));
    Ok(())
}

/// `contre_jour_toile` (skill `rendu-visuel`): the SPAD flying level over a man standing
/// on the grass, the sun 9° beyond it: its fabric glows with the light it lets through,
/// the ribs, spars and painted marks dark across it.
pub fn spawn_contre_jour(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut hold: ResMut<CaptureHold>,
) -> Result {
    let cg = centre_of_gravity()?;
    // Heading north-east, banked 10° to the right, 13 m up and 15 m north of the man:
    // three-quarter from below, the nose towards him.
    let attitude = DQuat::from_euler(EulerRot::ZYX, 45f64.to_radians(), 0.0, 10f64.to_radians());
    let position = DVec3::new(15.0, 0.0, -(13.0 + EYE_HEIGHT_M));
    spawn_spad(
        &mut commands,
        &asset_server,
        &mut hold,
        position,
        attitude,
        cg,
    );
    // Seen 41° high to the north; the sun to the north too, 50° high.
    spawn_outdoors(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut media,
        sun_direction(0.0, 50.0),
    );
    let eye = Vec3::Y * EYE_HEIGHT_M as f32;
    commands.spawn(camera(
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

/// The model, its entity at the centre of gravity as the flight plugin will place it.
fn spawn_spad(
    commands: &mut Commands,
    asset_server: &AssetServer,
    hold: &mut CaptureHold,
    position_ned: DVec3,
    attitude: DQuat,
    cg: DVec3,
) {
    hold.0 += 1;
    let model = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset(format!("aircraft/{AIRCRAFT}/model.glb")));
    commands
        .spawn((
            Name::new("SPAD VII"),
            render_transform(position_ned, attitude, DVec3::ZERO),
            Visibility::default(),
        ))
        .with_children(|aircraft| {
            aircraft
                .spawn((
                    Name::new("Model"),
                    WorldAssetRoot(model),
                    model_transform(cg),
                ))
                .observe(super::maquette::turn_the_propeller);
        });
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
        media.add(ScatteringMedium::earth(256, 256)),
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

/// A camera of `fov_deg` vertical field, with the atmosphere and its light, temporal
/// anti-aliasing and the game's tone mapping.
fn camera(transform: Transform, fov_deg: f32, ev100: f32) -> impl Bundle {
    (
        Name::new("Camera"),
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: fov_deg.to_radians(),
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
