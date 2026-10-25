//! Contract between an aircraft's 3D model and its data (skill `pipeline-assets`, ADR
//! 0006): the nodes the model must have, and where its moving parts turn, read from
//! `aircraft.ron`. `tools/mockup` writes models with it and `tools/asset_check` checks any
//! model against it, so that the two never disagree.
//!
//! Positions are body axes from the datum of `aircraft.ron`, metres. Directions:
//! - a control surface turns about its hinge; its node's local +X axis is the hinge,
//!   oriented so that a positive rotation about it (right-hand rule) is the flight model's
//!   positive deflection (ailerons: roll right; elevator: nose up; rudder: nose right);
//! - the propeller's node turns about its local +X axis, the propeller's rotation axis
//!   (its normal rotation is positive about it);
//! - `cam_pilot_eye` and the gun muzzles look along their local −Z, as glTF cameras.

use glam::DVec3;

use crate::DataError;
use crate::aircraft::{AircraftData, Control, Loading, Rotation, SurfaceKind, surface_strips};

/// One strip of a lifting surface, drawn as a plate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plate {
    /// Index of the surface in `aircraft.ron`.
    pub surface: usize,
    /// Quarter-chord point.
    pub position: DVec3,
    /// Unit vectors: along the chord towards the leading edge, and the lift side.
    pub chord_axis: DVec3,
    pub normal: DVec3,
    pub chord: f64,
    /// Width along the span.
    pub width: f64,
    /// The control surface it carries: index in [`Contract::hinges`] and chord ratio.
    pub control: Option<(usize, f64)>,
}

impl Plate {
    pub fn leading_edge(&self) -> DVec3 {
        self.position + self.chord_axis * (0.25 * self.chord)
    }

    /// Point of the hinge at this strip, for a control of chord ratio `ratio`.
    pub fn hinge(&self, ratio: f64) -> DVec3 {
        self.leading_edge() - self.chord_axis * ((1.0 - ratio) * self.chord)
    }

    /// Span direction, unit: chord × normal.
    pub fn span_axis(&self) -> DVec3 {
        self.chord_axis.cross(self.normal)
    }
}

/// Where a control surface turns.
#[derive(Debug, Clone, PartialEq)]
pub struct Hinge {
    /// Node name (`ctrl_aileron_r_upper`, `ctrl_elevator`…).
    pub node: String,
    /// Ends of the hinge line over the control's strips.
    pub line: (DVec3, DVec3),
    /// Middle of the line: the node's pivot.
    pub pivot: DVec3,
    /// Unit; a positive rotation about it is the positive deflection.
    pub axis: DVec3,
}

/// Where the propeller turns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PropellerAxis {
    pub hub: DVec3,
    /// Unit; the propeller's normal rotation is positive about it.
    pub axis: DVec3,
    pub radius: f64,
}

/// What a model of the aircraft must hold.
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    pub plates: Vec<Plate>,
    pub hinges: Vec<Hinge>,
    pub propeller: PropellerAxis,
    /// Names of the nodes that must exist.
    pub required: Vec<String>,
}

/// Fixed nodes every aircraft model has, besides its control surfaces.
pub const FIXED_NODES: [&str; 5] = ["body", "prop", "prop_blur", "cam_pilot_eye", "gun_1_muzzle"];

/// The contract of an aircraft's model, from its data.
pub fn contract(data: &AircraftData) -> Result<Contract, DataError> {
    let file = &data.file;
    let invalid = |message: String| DataError::Invalid {
        origin: format!("model of {}", file.id),
        message,
    };
    // The flight model gives each control strip the sign of its deflection.
    let built = data.build(&Loading { fuel_fraction: 1.0 })?;
    let flight_surfaces = built.aircraft.lifting().surfaces();
    let tiers = wing_tiers(data);
    let (mut plates, mut hinges) = (Vec::new(), Vec::new());
    for (index, spec) in file.surfaces.iter().enumerate() {
        let generated = surface_strips(spec).map_err(invalid)?;
        let flight = flight_surfaces[index].strips();
        if flight.len() != generated.strips.len() {
            return Err(invalid(format!("{}: strips out of step", spec.name)));
        }
        let first = plates.len();
        for placed in &generated.strips {
            let s = placed.strip;
            plates.push(Plate {
                surface: index,
                position: s.position,
                chord_axis: s.chord_axis,
                normal: s.normal,
                chord: s.chord,
                width: s.area / s.chord,
                control: None,
            });
        }
        // Members of each node: (strip index, chord ratio), in the order the controls come.
        let mut nodes: Vec<(String, Vec<(usize, f64)>)> = Vec::new();
        for control in &spec.controls {
            // One node per side for ailerons; one node for an elevator or a rudder, even
            // spread over several panels (the SPAD's rudder, below and above the tailplane).
            let sides: &[f64] = match control.control {
                Control::Aileron => &[1.0, -1.0],
                Control::Elevator | Control::Rudder => &[0.0],
            };
            for &side in sides {
                let node = match control.control {
                    Control::Aileron => format!(
                        "ctrl_aileron_{}_{}",
                        if side > 0.0 { "r" } else { "l" },
                        tiers.get(&index).copied().unwrap_or("upper")
                    ),
                    Control::Elevator => "ctrl_elevator".to_owned(),
                    Control::Rudder => "ctrl_rudder".to_owned(),
                };
                let members: Vec<(usize, f64)> = generated
                    .strips
                    .iter()
                    .enumerate()
                    .filter(|(_, placed)| {
                        (side == 0.0 || placed.strip.position.y * side > 0.0)
                            && placed.panel == control.panel
                            && (control.span_from..=control.span_to).contains(&placed.along)
                    })
                    .map(|(k, _)| (k, control.chord_ratio))
                    .collect();
                if members.is_empty() {
                    return Err(invalid(format!("{}: {node} covers no strip", spec.name)));
                }
                match nodes.iter_mut().find(|(name, _)| *name == node) {
                    Some((_, list)) => list.extend(members),
                    None => nodes.push((node, members)),
                }
            }
        }
        for (node, members) in nodes {
            let hinge_index = hinges.len();
            let (mut axis, mut centre) = (DVec3::ZERO, DVec3::ZERO);
            let mut points = Vec::new();
            for &(k, ratio) in &members {
                let sign = flight[k]
                    .control
                    .map(|c| c.alpha_per_rad.signum())
                    .ok_or_else(|| invalid(format!("{node}: strip without control")))?;
                let plate = &mut plates[first + k];
                plate.control = Some((hinge_index, ratio));
                axis += sign * plate.span_axis();
                let point = plate.hinge(ratio);
                centre += point;
                points.push(point);
            }
            // A straight hinge along the span through the strips' hinge points: with a
            // constant chord ratio on a tapered panel they are not quite in line.
            let axis = axis.normalize();
            let centre = centre / points.len() as f64;
            let along = points.iter().map(|p| (*p - centre).dot(axis));
            let (low, high) = along.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), t| {
                (lo.min(t), hi.max(t))
            });
            let line = (centre + axis * low, centre + axis * high);
            hinges.push(Hinge {
                node,
                line,
                pivot: 0.5 * (line.0 + line.1),
                axis,
            });
        }
    }
    let p = &file.propeller;
    let axis = match p.rotation {
        Rotation::Clockwise => DVec3::X,
        Rotation::CounterClockwise => DVec3::NEG_X,
    };
    let propeller = PropellerAxis {
        hub: DVec3::from_array(p.hub_m),
        axis,
        radius: 0.5 * p.diameter.si(),
    };
    let mut required: Vec<String> = FIXED_NODES.iter().map(|&n| n.to_owned()).collect();
    required.extend(hinges.iter().map(|h| h.node.clone()));
    Ok(Contract {
        plates,
        hinges,
        propeller,
        required,
    })
}

/// « upper », « middle » or « lower » for each wing (the surfaces of the downwash block),
/// by height; a monoplane's wing is « upper ».
fn wing_tiers(data: &AircraftData) -> std::collections::BTreeMap<usize, &'static str> {
    let file = &data.file;
    let mut wings: Vec<(usize, f64)> = file
        .surfaces
        .iter()
        .enumerate()
        .filter(|(_, s)| s.kind == SurfaceKind::Horizontal)
        .filter(|(_, s)| {
            file.downwash
                .as_ref()
                .is_some_and(|d| d.wings.contains(&s.name))
        })
        .map(|(i, s)| (i, s.panels.first().map_or(0.0, |p| p.root_m[2])))
        .collect();
    // Highest first: z is down.
    wings.sort_by(|a, b| a.1.total_cmp(&b.1));
    let names: &[&str] = match wings.len() {
        3 => &["upper", "middle", "lower"],
        2 => &["upper", "lower"],
        _ => &["upper"],
    };
    wings
        .iter()
        .zip(names.iter().copied().chain(std::iter::repeat("upper")))
        .map(|(&(i, _), name)| (i, name))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::aircraft::load_aircraft;

    fn spad() -> Contract {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        contract(&load_aircraft(&assets, "spad7").unwrap()).unwrap()
    }

    #[test]
    fn the_spad_needs_its_ailerons_elevator_rudder_and_fixed_nodes() {
        let c = spad();
        for name in [
            "body",
            "prop",
            "prop_blur",
            "cam_pilot_eye",
            "gun_1_muzzle",
            "ctrl_aileron_r_upper",
            "ctrl_aileron_l_upper",
            "ctrl_elevator",
            "ctrl_rudder",
        ] {
            assert!(
                c.required.iter().any(|r| r == name),
                "{name}: {:?}",
                c.required
            );
        }
        assert_eq!(c.required.len(), 9, "{:?}", c.required);
        // Clockwise seen by the pilot: right-handed about the thrust axis.
        assert_eq!(c.propeller.axis, DVec3::X);
        assert!((c.propeller.radius - 1.2).abs() < 1e-12);
    }

    #[test]
    fn a_positive_deflection_turns_each_control_the_flight_model_s_way() {
        let c = spad();
        let hinge = |name: &str| c.hinges.iter().find(|h| h.node == name).unwrap();
        // Roll right: the right aileron's trailing edge goes up, the left one down. The
        // trailing edge is behind the hinge (−x); turning it by +δ about the axis moves
        // it by δ axis × (−x).
        let trailing = |h: &Hinge| h.axis.cross(DVec3::NEG_X);
        assert!(trailing(hinge("ctrl_aileron_r_upper")).z < -0.9);
        assert!(trailing(hinge("ctrl_aileron_l_upper")).z > 0.9);
        // Nose up: the elevator's trailing edge goes up. Nose right: the rudder's goes to
        // the right (+y), the fin's lift to the left, and the tail with it.
        assert!(trailing(hinge("ctrl_elevator")).z < -0.9);
        assert!(trailing(hinge("ctrl_rudder")).y > 0.9);
        // Straight hinges along the span: spanwise for the wings and the tailplane,
        // vertical for the rudder, which is one node over its two panels.
        for h in &c.hinges {
            let line = (h.line.1 - h.line.0).normalize();
            assert!(line.dot(h.axis) > 0.999, "{h:?}");
            let spanwise = if h.node == "ctrl_rudder" {
                h.axis.z
            } else {
                h.axis.y
            };
            assert!(spanwise.abs() > 0.99, "{h:?}");
        }
        assert_eq!(
            c.hinges.iter().filter(|h| h.node == "ctrl_rudder").count(),
            1
        );
    }

    #[test]
    fn every_plate_is_a_strip_of_the_flight_model() {
        let c = spad();
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        let data = load_aircraft(&assets, "spad7").unwrap();
        let built = data.build(&Loading { fuel_fraction: 1.0 }).unwrap();
        let count: usize = built
            .aircraft
            .lifting()
            .surfaces()
            .iter()
            .map(|s| s.strips().len())
            .sum();
        assert_eq!(c.plates.len(), count);
        let area: f64 = c.plates.iter().map(|p| p.chord * p.width).sum();
        let flight: f64 = built
            .aircraft
            .lifting()
            .surfaces()
            .iter()
            .flat_map(|s| s.strips())
            .map(|s| s.area)
            .sum();
        assert!((area - flight).abs() < 1e-9 * flight);
        // Controlled plates carry the hinge of their side.
        for p in c.plates.iter().filter(|p| p.control.is_some()) {
            let (h, _) = p.control.unwrap();
            let node = &c.hinges[h].node;
            if node.starts_with("ctrl_aileron_r") {
                assert!(p.position.y > 0.0);
            }
            if node.starts_with("ctrl_aileron_l") {
                assert!(p.position.y < 0.0);
            }
        }
    }
}
