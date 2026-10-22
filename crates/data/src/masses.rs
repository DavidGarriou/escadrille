//! Mass build-up files (`assets/aircraft/<id>/masses.ron`): the empty aircraft as a list
//! of components (engine, propeller, wings, fuselage…), each with its mass, position and
//! shape. They give the empty mass, centre of gravity and inertia written in the mass
//! block of `aircraft.ron` (`fdm_lab inertia <id>` prints them), and a test keeps the two
//! files in agreement. Positions are from the datum of `aircraft.ron`, body axes (x
//! forwards, y right, z down), metres.

use std::path::Path;

use serde::Deserialize;

use crate::aircraft::{InertiaSpec, MassSpec};
use crate::units::Mass;
use crate::{DataError, load_checked};

/// How far `aircraft.ron` may stray from the build-up (rounding of the copied values):
/// relative on the mass and the moments of inertia, metres on the centre of gravity.
pub const MASS_TOLERANCE: f64 = 0.005;
pub const CG_TOLERANCE_M: f64 = 0.005;
pub const INERTIA_TOLERANCE: f64 = 0.01;

/// Inertia of a component about its own centre.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Shape {
    /// Small against its distance to the centre of gravity.
    Point,
    /// Uniform solid box, edges along the body axes: lengths along x, y and z, m.
    Box { size_m: [f64; 3] },
    /// Principal moments about its centre, body axes, kg·m² (from a separate estimate).
    Own { ixx: f64, iyy: f64, izz: f64 },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ComponentSpec {
    pub name: String,
    pub mass: Mass,
    pub position_m: [f64; 3],
    pub shape: Shape,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Masses")]
pub struct MassesFile {
    pub aircraft: String,
    /// Must be the datum of `aircraft.ron`.
    pub datum: String,
    pub components: Vec<ComponentSpec>,
}

/// The empty aircraft from its components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmptyAircraft {
    /// kg.
    pub mass: f64,
    /// From the datum, m.
    pub cg_m: [f64; 3],
    /// About the centre of gravity, body axes, kg·m²; `ixz` = ∫ x z dm.
    pub inertia: InertiaSpec,
}

impl MassesFile {
    pub fn empty(&self) -> EmptyAircraft {
        let mass: f64 = self.components.iter().map(|c| c.mass.si()).sum();
        let mut cg = [0.0; 3];
        for c in &self.components {
            for (g, p) in cg.iter_mut().zip(c.position_m) {
                *g += c.mass.si() * p / mass;
            }
        }
        let mut inertia = InertiaSpec {
            ixx: 0.0,
            iyy: 0.0,
            izz: 0.0,
            ixz: 0.0,
        };
        for c in &self.components {
            let m = c.mass.si();
            let [x, y, z] = [0, 1, 2].map(|i| c.position_m[i] - cg[i]);
            let (ixx, iyy, izz) = match c.shape {
                Shape::Point => (0.0, 0.0, 0.0),
                Shape::Box { size_m: [a, b, h] } => (
                    m * (b * b + h * h) / 12.0,
                    m * (a * a + h * h) / 12.0,
                    m * (a * a + b * b) / 12.0,
                ),
                Shape::Own { ixx, iyy, izz } => (ixx, iyy, izz),
            };
            inertia.ixx += ixx + m * (y * y + z * z);
            inertia.iyy += iyy + m * (x * x + z * z);
            inertia.izz += izz + m * (x * x + y * y);
            inertia.ixz += m * x * z;
        }
        EmptyAircraft {
            mass,
            cg_m: cg,
            inertia,
        }
    }

    /// Whether the mass block of `aircraft.ron` agrees with the build-up.
    pub fn check(&self, spec: &MassSpec) -> Result<(), String> {
        let e = self.empty();
        let mut errors = Vec::new();
        let empty = spec.empty.si();
        if (empty - e.mass).abs() > MASS_TOLERANCE * e.mass {
            errors.push(format!(
                "masse à vide {empty:.1} kg, bilan {:.1} kg",
                e.mass
            ));
        }
        for (axis, (a, b)) in ["x", "y", "z"]
            .iter()
            .zip(spec.cg_empty_m.iter().zip(e.cg_m))
        {
            if (a - b).abs() > CG_TOLERANCE_M {
                errors.push(format!("centre de gravité {axis} {a:.3} m, bilan {b:.3} m"));
            }
        }
        let i = spec.inertia_empty_kg_m2;
        let scale = e.inertia.ixx.max(e.inertia.iyy).max(e.inertia.izz);
        for (name, a, b) in [
            ("ixx", i.ixx, e.inertia.ixx),
            ("iyy", i.iyy, e.inertia.iyy),
            ("izz", i.izz, e.inertia.izz),
            ("ixz", i.ixz, e.inertia.ixz),
        ] {
            if (a - b).abs() > INERTIA_TOLERANCE * scale {
                errors.push(format!("{name} {a:.1} kg·m², bilan {b:.1} kg·m²"));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join(" ; "))
        }
    }
}

/// Loads `assets/aircraft/<id>/masses.ron` (every number with an origin), if there is one.
pub fn load_masses(assets: &Path, id: &str) -> Result<Option<MassesFile>, DataError> {
    let path = assets.join("aircraft").join(id).join("masses.ron");
    if !path.exists() {
        return Ok(None);
    }
    load_checked(&path).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::from_ron_str;

    fn file(text: &str) -> MassesFile {
        from_ron_str(text, "test").unwrap()
    }

    #[test]
    fn two_points_give_the_textbook_dumbbell() {
        let f = file(
            r#"Masses(aircraft: "t", datum: "d", components: [
                (name: "a", mass: Kg(10.0), position_m: (1.0, 0.0, 0.5), shape: Point),
                (name: "b", mass: Kg(30.0), position_m: (-1.0, 0.0, -0.5), shape: Point),
            ])"#,
        );
        let e = f.empty();
        assert_eq!(e.mass, 40.0);
        // Centre of gravity at a quarter of the way from b to a.
        assert!((e.cg_m[0] + 0.5).abs() < 1e-12 && (e.cg_m[2] + 0.25).abs() < 1e-12);
        // Reduced mass 7.5 kg on a separation of (2, 0, 1).
        let mu = 10.0 * 30.0 / 40.0;
        assert!((e.inertia.ixx - mu * 1.0).abs() < 1e-12);
        assert!((e.inertia.iyy - mu * 5.0).abs() < 1e-12);
        assert!((e.inertia.izz - mu * 4.0).abs() < 1e-12);
        assert!((e.inertia.ixz - mu * 2.0).abs() < 1e-12);
    }

    #[test]
    fn a_box_has_its_own_inertia_and_the_check_sees_a_drift() {
        let f = file(
            r#"Masses(aircraft: "t", datum: "d", components: [
                (name: "plank", mass: Kg(12.0), position_m: (0.0, 0.0, 0.0), shape: Box(size_m: (1.0, 2.0, 0.0))),
            ])"#,
        );
        let e = f.empty();
        assert!((e.inertia.ixx - 4.0).abs() < 1e-12, "{e:?}");
        assert!((e.inertia.iyy - 1.0).abs() < 1e-12, "{e:?}");
        assert!((e.inertia.izz - 5.0).abs() < 1e-12, "{e:?}");
        let spec = |ixx: f64| -> MassSpec {
            from_ron_str(
                &format!(
                    r#"(empty: Kg(12.0), cg_empty_m: (0.0, 0.0, 0.0),
                        inertia_empty_kg_m2: (ixx: {ixx}, iyy: 1.0, izz: 5.0, ixz: 0.0),
                        pilot: Kg(80.0), pilot_m: (0.0, 0.0, 0.0),
                        fuel: (capacity: L(10.0), density_kg_per_l: 0.72, position_m: (0.0, 0.0, 0.0)))"#
                ),
                "test",
            )
            .unwrap()
        };
        assert!(f.check(&spec(4.0)).is_ok());
        let err = f.check(&spec(4.5)).unwrap_err();
        assert!(err.contains("ixx"), "{err}");
    }
}
