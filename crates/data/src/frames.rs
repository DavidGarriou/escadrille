//! Frames of the 3D models (ADR 0006): the flight model's body axes (x forward, y right,
//! z down) and world NED axes, Bevy's axes (forward −Z, right +X, up +Y; world north −Z,
//! east +X) and glTF's (forward +Z, right −X, up +Y). Engine-independent, so that the
//! tools that write and check models (`tools/mockup`, `tools/asset_check`) and the game's
//! bridge (`crates/game/src/flight/bridge.rs`) share one definition.

use glam::DVec3;

/// A flight-model vector (body axes or NED) in Bevy axes (local or world): the one proper
/// rotation M of ADR 0006, (x, y, z) ↦ (y, −z, −x).
pub fn to_bevy(v: DVec3) -> DVec3 {
    DVec3::new(v.y, -v.z, -v.x)
}

/// A Bevy vector in flight-model axes: the inverse of [`to_bevy`].
pub fn from_bevy(v: DVec3) -> DVec3 {
    DVec3::new(-v.z, v.x, -v.y)
}

/// A body-axes vector in glTF axes: [`to_bevy`] then half a turn about Y (glTF faces +Z,
/// Bevy −Z), (x, y, z) ↦ (−y, −z, x).
pub fn to_gltf(v: DVec3) -> DVec3 {
    let b = to_bevy(v);
    DVec3::new(-b.x, b.y, -b.z)
}

/// A glTF vector in body axes: the inverse of [`to_gltf`].
pub fn from_gltf(v: DVec3) -> DVec3 {
    from_bevy(DVec3::new(-v.x, v.y, -v.z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_axes_map_as_adr_0006_says() {
        // Nose and north to −Z, right wing and east to +X, down to −Y in Bevy.
        assert_eq!(to_bevy(DVec3::X), DVec3::NEG_Z);
        assert_eq!(to_bevy(DVec3::Y), DVec3::X);
        assert_eq!(to_bevy(DVec3::Z), DVec3::NEG_Y);
        // glTF: nose +Z, right wing −X, down −Y.
        assert_eq!(to_gltf(DVec3::X), DVec3::Z);
        assert_eq!(to_gltf(DVec3::Y), DVec3::NEG_X);
        assert_eq!(to_gltf(DVec3::Z), DVec3::NEG_Y);
        for v in [DVec3::X, DVec3::Y, DVec3::Z, DVec3::new(1.5, -2.0, 0.25)] {
            assert_eq!(from_bevy(to_bevy(v)), v);
            assert_eq!(to_bevy(from_bevy(v)), v);
            assert_eq!(from_gltf(to_gltf(v)), v);
        }
        // The line of the ADR for the modeller: Blender is glTF before the exporter's
        // (X, Y, Z) ↦ (X, Z, −Y), so body (x, y, z) is (−y, −x, −z) in Blender.
        let g = to_gltf(DVec3::new(1.0, 2.0, 3.0));
        assert_eq!(DVec3::new(g.x, -g.z, g.y), DVec3::new(-2.0, -1.0, -3.0));
    }

    #[test]
    fn the_changes_of_frame_are_proper_rotations() {
        // Lengths and cross products kept: no mirror, which would turn the propeller and
        // the torque the other way on screen.
        let (a, b) = (DVec3::new(0.3, -1.2, 2.0), DVec3::new(-0.7, 0.4, 1.1));
        for f in [to_bevy, to_gltf] {
            assert!((f(a).length() - a.length()).abs() < 1e-15);
            assert!((f(a.cross(b)) - f(a).cross(f(b))).length() < 1e-15);
        }
    }
}
