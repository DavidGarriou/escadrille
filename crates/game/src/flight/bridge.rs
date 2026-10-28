//! The one conversion between the flight model's frames and Bevy's (ADR 0006).
//!
//! - Flight model (`escadrille_fdm`): body axes x forward, y right, z down; world local
//!   NED, x north, y east, z down; f64.
//! - Bevy: local axes forward −Z, right +X, up +Y ([`Transform::forward`]); world north
//!   −Z, east +X, up +Y.
//!
//! Both are right-handed, and the body and the world map the same way: the change of
//! frame is one proper rotation M, (x, y, z) ↦ (y, −z, −x) ([`to_bevy`], defined once in
//! `escadrille_data::frames` for the asset tools and re-exported here). Positions,
//! velocities, forces and angular velocities all go through it; attitudes are conjugated
//! by it ([`attitude_to_bevy`]). A glTF scene faces +Z (glTF 2.0): [`GLTF_TO_BEVY`] turns
//! it to face Bevy's −Z. World positions stay in f64 until the render origin is taken off
//! ([`render_transform`]).

use bevy::math::{DQuat, DVec3, Quat, Vec3};
use bevy::prelude::Transform;
pub use escadrille_data::frames::{from_bevy, to_bevy};

/// A flight-model attitude (rotation from body to NED axes) as a Bevy rotation (local to
/// world): the same angle about the axis that [`to_bevy`] gives, (w ; v) ↦ (w ; M v).
pub fn attitude_to_bevy(q: DQuat) -> DQuat {
    let v = to_bevy(DVec3::new(q.x, q.y, q.z));
    DQuat::from_xyzw(v.x, v.y, v.z, q.w)
}

/// A Bevy rotation as a flight-model attitude: the inverse of [`attitude_to_bevy`].
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the inverse conversion of ADR 0006, tested with it"
    )
)]
pub fn attitude_from_bevy(q: DQuat) -> DQuat {
    let v = from_bevy(DVec3::new(q.x, q.y, q.z));
    DQuat::from_xyzw(v.x, v.y, v.z, q.w)
}

/// Half a turn about Y: a glTF scene, which faces +Z with its right on −X (glTF 2.0),
/// then faces Bevy's −Z with its right on +X. The rotation of Bevy's experimental
/// `GltfConvertCoordinates`, applied here instead (ADR 0006, option B1).
pub const GLTF_TO_BEVY: Quat = Quat::from_xyzw(0.0, 1.0, 0.0, 0.0);

/// Transform of an aircraft's 3D model under the aircraft's entity. The model's origin is
/// the datum of `aircraft.ron`, the entity is at the centre of gravity, `cg_from_datum`
/// (body axes, m) away from it.
pub fn model_transform(cg_from_datum: DVec3) -> Transform {
    Transform {
        translation: to_bevy(-cg_from_datum).as_vec3(),
        rotation: GLTF_TO_BEVY,
        scale: Vec3::ONE,
    }
}

/// Render transform of a simulated body from its flight-model position (NED, m) and
/// attitude, relative to the render origin (Bevy world axes, m): the floating origin of
/// skill `bevy`, so that only the small difference goes to f32.
pub fn render_transform(position_ned: DVec3, attitude: DQuat, render_origin: DVec3) -> Transform {
    Transform {
        translation: (to_bevy(position_ned) - render_origin).as_vec3(),
        rotation: attitude_to_bevy(attitude).as_quat(),
        scale: Vec3::ONE,
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::EulerRot;

    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-6
    }

    /// Flight-model attitude from heading, pitch and bank, as the flight model builds it.
    fn attitude(heading_deg: f64, pitch_deg: f64, bank_deg: f64) -> DQuat {
        DQuat::from_euler(
            EulerRot::ZYX,
            heading_deg.to_radians(),
            pitch_deg.to_radians(),
            bank_deg.to_radians(),
        )
    }

    #[test]
    fn an_attitude_turns_the_converted_vectors_as_it_turns_the_flight_model_s() {
        let q = attitude(37.0, -12.0, 64.0);
        let b = attitude_to_bevy(q);
        assert!((b.length() - 1.0).abs() < 1e-15);
        for v in [DVec3::X, DVec3::Y, DVec3::Z, DVec3::new(0.2, 0.9, -0.4)] {
            assert!((b * to_bevy(v) - to_bevy(q * v)).length() < 1e-12, "{v}");
        }
        assert!((attitude_from_bevy(b) - q).length() < 1e-15);
    }

    #[test]
    fn simple_flights_look_right_in_bevy() {
        let origin = DVec3::ZERO;
        let t = |q: DQuat| render_transform(DVec3::ZERO, q, origin);
        // Level, heading north: Bevy's own axes.
        let north = t(attitude(0.0, 0.0, 0.0));
        assert!(close(*north.forward(), Vec3::NEG_Z) && close(*north.up(), Vec3::Y));
        // Heading east: the nose points to +X.
        assert!(close(*t(attitude(90.0, 0.0, 0.0)).forward(), Vec3::X));
        // Nose up 10°: forward rises.
        let (s, c) = 10f32.to_radians().sin_cos();
        assert!(close(
            *t(attitude(0.0, 10.0, 0.0)).forward(),
            Vec3::new(0.0, s, -c)
        ));
        // Banked 30° to the right: the right wing goes down, the top leans right.
        let bank = t(attitude(0.0, 0.0, 30.0));
        let (s, c) = 30f32.to_radians().sin_cos();
        assert!(close(*bank.right(), Vec3::new(c, -s, 0.0)));
        assert!(close(*bank.up(), Vec3::new(s, c, 0.0)));
    }

    #[test]
    fn a_right_hand_propeller_turns_clockwise_on_screen_seen_from_the_cockpit() {
        // Right-handed about the thrust axis, body +x: the top blade moves to the right.
        let turn = DQuat::from_axis_angle(DVec3::X, 10f64.to_radians());
        let tip = to_bevy(turn * DVec3::NEG_Z);
        // Seen from the cockpit, looking along −Z with +X on the right and +Y up.
        assert!(tip.x > 0.0 && tip.y > 0.0 && tip.z.abs() < 1e-15, "{tip}");
        let on_screen = attitude_to_bevy(turn) * to_bevy(DVec3::NEG_Z);
        assert!((on_screen - tip).length() < 1e-15);
    }

    #[test]
    fn a_gltf_model_is_turned_to_face_bevy_s_forward() {
        // glTF 2.0: forward +Z, right −X, up +Y.
        let turn = Transform::from_rotation(GLTF_TO_BEVY);
        assert!(close(turn.rotation * Vec3::Z, Vec3::NEG_Z));
        assert!(close(turn.rotation * Vec3::NEG_X, Vec3::X));
        assert!(close(turn.rotation * Vec3::Y, Vec3::Y));
        // The model tools write glTF with `escadrille_data::frames::to_gltf`: turned by
        // this constant, it lands where `to_bevy` puts it.
        for body in [DVec3::X, DVec3::Y, DVec3::Z, DVec3::new(1.0, 2.0, 3.0)] {
            let gltf = escadrille_data::frames::to_gltf(body);
            assert!((GLTF_TO_BEVY.as_dquat() * gltf - to_bevy(body)).length() < 1e-15);
        }
    }

    #[test]
    fn the_model_sits_at_its_datum_from_the_centre_of_gravity() {
        // SPAD: centre of gravity 0.461 m behind and 0.423 m above the datum (the leading
        // edge of the lower plane), so the datum is ahead (−Z) and below (−Y).
        let model = model_transform(DVec3::new(-0.461, 0.0, -0.423));
        assert!(close(model.translation, Vec3::new(0.0, -0.423, -0.461)));
        assert_eq!(model.rotation, GLTF_TO_BEVY);
        assert_eq!(model.scale, Vec3::ONE);
    }

    #[test]
    fn only_the_offset_from_the_render_origin_goes_to_f32() {
        // 80 km east of the map's origin, at 2 000 m: a millimetre to the east (+X) is
        // kept near the camera.
        let position = DVec3::new(-5_000.0, 80_000.0, -2_000.0);
        let camera = to_bevy(position) + DVec3::new(3.0, -1.0, 12.0);
        let t = render_transform(
            position + DVec3::new(0.0, 0.0012, 0.0),
            DQuat::IDENTITY,
            camera,
        );
        assert!(
            close(t.translation, Vec3::new(-2.9988, 1.0, -12.0)),
            "{t:?}"
        );
    }
}
