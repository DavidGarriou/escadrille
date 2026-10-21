//! Fuselage as a slender body: the side force and the destabilising moments that plain
//! drag items cannot give (aerodynamicist review of 2026-10-21: without them, the
//! directional and longitudinal stabilities are overestimated).
//!
//! Two terms, after Allen and Perkins (*A study of effects of viscosity on flow over
//! slender inclined bodies of revolution*, NACA Report 1048, 1951):
//! - the potential (Munk) couple, q Vol (k₂ − k₁) sin 2α, which turns the body broadside
//!   to the flow; written ρ Vol (k₂ − k₁) u × (cross flow), smooth at rest and right in
//!   reversed flow;
//! - the viscous cross-flow force, ½ ρ η C_dc A |V_c| V_c, opposing the cross flow V_c,
//!   with the plan area for the vertical component and the side area for the lateral one
//!   (a square-ish fuselage is not a body of revolution).
//!
//! Axial drag stays in the drag items. The data give the volume, the apparent-mass factor
//! k₂ − k₁ (Lamb's coefficients for the fineness ratio, C:), the projected areas and
//! η C_dc (E:).

use glam::DVec3;

use crate::rigid_body::Wrench;

#[derive(Debug, Clone, PartialEq)]
pub struct SlenderBody {
    pub name: String,
    /// Centroid of the projected areas, where the cross-flow force acts: body axes,
    /// relative to the centre of gravity, m.
    pub position: DVec3,
    /// Volume, m³.
    pub volume: f64,
    /// Apparent-mass factor k₂ − k₁.
    pub apparent_mass: f64,
    /// Area seen from above (cross flow in pitch), m².
    pub plan_area: f64,
    /// Area seen from the side (cross flow in yaw), m².
    pub side_area: f64,
    /// η C_dc: cross-flow drag coefficient times Allen and Perkins' efficiency factor.
    pub crossflow_drag: f64,
}

impl SlenderBody {
    /// Force and moment about the centre of gravity, same conventions as
    /// [`crate::aero::drag::DragItem::loads`].
    pub fn loads(
        &self,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        local_flow: impl Fn(DVec3) -> DVec3,
    ) -> Wrench {
        let v = air_velocity + angular_velocity.cross(self.position) - local_flow(self.position);
        let cross = DVec3::new(0.0, v.y, v.z);
        let munk = -DVec3::X.cross(cross) * (density * self.apparent_mass * self.volume * v.x);
        let force = DVec3::new(0.0, -self.side_area * v.y, -self.plan_area * v.z)
            * (0.5 * density * self.crossflow_drag * cross.length());
        Wrench::force_at(force, self.position)
            + Wrench {
                force: DVec3::ZERO,
                moment: munk,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RHO: f64 = 1.225;

    /// Synthetic fuselage (not historical data).
    fn fuselage() -> SlenderBody {
        SlenderBody {
            name: "fuselage".into(),
            position: DVec3::new(-0.5, 0.0, 0.0),
            volume: 3.0,
            apparent_mass: 0.9,
            plan_area: 4.0,
            side_area: 5.0,
            crossflow_drag: 0.8,
        }
    }

    fn still(_: DVec3) -> DVec3 {
        DVec3::ZERO
    }

    #[test]
    fn munk_couple_is_destabilising_in_pitch_and_yaw() {
        let body = SlenderBody {
            plan_area: 0.0,
            side_area: 0.0,
            ..fuselage()
        };
        let (v, alpha) = (40.0, 0.1_f64);
        let nose_up = body.loads(
            DVec3::new(v * alpha.cos(), 0.0, v * alpha.sin()),
            DVec3::ZERO,
            RHO,
            still,
        );
        let expected = 0.5 * RHO * v * v * 3.0 * 0.9 * (2.0 * alpha).sin();
        assert!(
            (nose_up.moment.y - expected).abs() < 1e-9 * expected,
            "{nose_up:?}"
        );
        assert_eq!(nose_up.force, DVec3::ZERO);
        // Air from the right: the nose turns further left, away from the wind.
        let slip = body.loads(DVec3::new(40.0, 4.0, 0.0), DVec3::ZERO, RHO, still);
        assert!(slip.moment.z < 0.0, "{slip:?}");
        // Flying backwards nose up: the couple changes sign with u.
        let backwards = body.loads(DVec3::new(-40.0, 0.0, 4.0), DVec3::ZERO, RHO, still);
        assert!(backwards.moment.y < 0.0);
    }

    #[test]
    fn crossflow_force_opposes_the_cross_flow() {
        let body = SlenderBody {
            apparent_mass: 0.0,
            ..fuselage()
        };
        let w = body.loads(DVec3::new(30.0, 3.0, -4.0), DVec3::ZERO, RHO, still);
        let q_c = 0.5 * RHO * 0.8 * 5.0;
        assert!((w.force.y + q_c * 5.0 * 3.0).abs() < 1e-9);
        assert!((w.force.z - q_c * 4.0 * 4.0).abs() < 1e-9);
        assert_eq!(w.force.x, 0.0);
        // Behind the centre of gravity, the side force turns the nose into the wind.
        assert!(w.moment.z > 0.0);
    }

    #[test]
    fn nothing_without_cross_flow_and_mirror_symmetric() {
        let body = fuselage();
        assert_eq!(
            body.loads(DVec3::new(40.0, 0.0, 0.0), DVec3::ZERO, RHO, still),
            Wrench::default()
        );
        assert_eq!(
            body.loads(DVec3::ZERO, DVec3::ZERO, RHO, still),
            Wrench::default()
        );
        let a = body.loads(
            DVec3::new(40.0, 3.0, 2.0),
            DVec3::new(0.2, 0.1, -0.3),
            RHO,
            still,
        );
        let b = body.loads(
            DVec3::new(40.0, -3.0, 2.0),
            DVec3::new(-0.2, 0.1, 0.3),
            RHO,
            still,
        );
        assert!((a.force.y + b.force.y).abs() < 1e-12 && (a.force.z - b.force.z).abs() < 1e-12);
        assert!((a.moment.x + b.moment.x).abs() < 1e-12 && (a.moment.z + b.moment.z).abs() < 1e-12);
        assert!((a.moment.y - b.moment.y).abs() < 1e-12);
    }
}
