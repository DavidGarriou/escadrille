//! Parasitic drag of non-lifting parts (struts, wires, radiator, undercarriage, pilot),
//! each with its own position, so that it also creates moments (`modele-aero.md`).

use glam::DVec3;

use crate::rigid_body::Wrench;

/// One drag item: C_D × reference area, at a point.
#[derive(Debug, Clone, PartialEq)]
pub struct DragItem {
    pub name: String,
    /// Drag area C_D · S, m².
    pub drag_area: f64,
    /// Point of application, body axes, relative to the centre of gravity, m.
    pub position: DVec3,
}

impl DragItem {
    /// Drag force −½ ρ |v| v C_D S at the item, with v = air velocity + ω × r − local
    /// flow (slipstream, downwash; see [`crate::aero::surface::LiftingSurface::loads`]).
    pub fn loads(
        &self,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        local_flow: impl Fn(DVec3) -> DVec3,
    ) -> Wrench {
        let v = air_velocity + angular_velocity.cross(self.position) - local_flow(self.position);
        let force = -v * (0.5 * density * v.length() * self.drag_area);
        Wrench::force_at(force, self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drag_opposes_the_motion_with_the_dynamic_pressure() {
        let item = DragItem {
            name: "strut".into(),
            drag_area: 0.05,
            position: DVec3::new(0.0, 2.0, -1.0),
        };
        let w = item.loads(DVec3::new(50.0, 0.0, 0.0), DVec3::ZERO, 1.225, |_| {
            DVec3::ZERO
        });
        let expected = 0.5 * 1.225 * 2_500.0 * 0.05;
        assert!((w.force - DVec3::new(-expected, 0.0, 0.0)).length() < 1e-9);
        // Drag on the right wing yaws the nose right (positive yaw moment).
        assert!(w.moment.z > 0.0, "{w:?}");
    }

    #[test]
    fn no_force_at_rest() {
        let item = DragItem {
            name: "wire".into(),
            drag_area: 0.01,
            position: DVec3::ZERO,
        };
        assert_eq!(
            item.loads(DVec3::ZERO, DVec3::ZERO, 1.225, |_| DVec3::ZERO),
            Wrench::default()
        );
    }

    #[test]
    fn rotating_drag_items_damp_the_yaw() {
        // Yawing right slows the right-hand item and speeds up the left one: the
        // drag difference yaws back to the left.
        let item = |y: f64| DragItem {
            name: "strut".into(),
            drag_area: 0.05,
            position: DVec3::new(0.0, y, 0.0),
        };
        let v = DVec3::new(50.0, 0.0, 0.0);
        let yawing = DVec3::new(0.0, 0.0, 0.5);
        let total = item(2.0).loads(v, yawing, 1.225, |_| DVec3::ZERO)
            + item(-2.0).loads(v, yawing, 1.225, |_| DVec3::ZERO);
        assert!(total.moment.z < 0.0, "{total:?}");
    }

    #[test]
    fn slipstream_increases_the_drag_with_the_square_of_the_local_speed() {
        let item = DragItem {
            name: "radiator".into(),
            drag_area: 0.1,
            position: DVec3::new(1.5, 0.0, 0.0),
        };
        let v = DVec3::new(40.0, 0.0, 0.0);
        let free = item.loads(v, DVec3::ZERO, 1.225, |_| DVec3::ZERO);
        let blown = item.loads(v, DVec3::ZERO, 1.225, |_| DVec3::new(-10.0, 0.0, 0.0));
        assert!((blown.force.x / free.force.x - (50.0f64 / 40.0).powi(2)).abs() < 1e-12);
    }
}
