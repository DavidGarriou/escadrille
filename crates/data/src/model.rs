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

/// How far a declared hinge (`hinge_m`) may be from the one the strips give: a fraction of
/// the control's mean chord, and an angle. Beyond, the declared hinge is not that of the
/// control the flight model simulates.
pub const HINGE_GUARD_CHORD_FRACTION: f64 = 0.25;
pub const HINGE_GUARD_DEG: f64 = 10.0;

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
        // Members of each node, one group per control and side: the strips it covers, its
        // chord ratio and its span on the panel (fractions), in the order the controls come.
        let mut nodes: Vec<(String, Vec<Group>)> = Vec::new();
        for control in &spec.controls {
            // One node per side for ailerons; one node for an elevator or a rudder, even
            // spread over several panels (the SPAD's rudder, below and above the tailplane).
            // Each side is a group of its own: a mirrored panel has the same fractions on
            // both.
            let mut covered = false;
            for side in [1.0, -1.0, 0.0] {
                let node = match control.control {
                    Control::Aileron => format!(
                        "ctrl_aileron_{}_{}",
                        if side > 0.0 { "r" } else { "l" },
                        tiers.get(&index).copied().unwrap_or("upper")
                    ),
                    Control::Elevator => "ctrl_elevator".to_owned(),
                    Control::Rudder => "ctrl_rudder".to_owned(),
                };
                let strips: Vec<usize> = generated
                    .strips
                    .iter()
                    .enumerate()
                    .filter(|(_, placed)| {
                        side_of(placed.strip.position.y) == side
                            && placed.panel == control.panel
                            && (control.span_from..=control.span_to).contains(&placed.along)
                    })
                    .map(|(k, _)| k)
                    .collect();
                if strips.is_empty() {
                    continue;
                }
                if control.control == Control::Aileron && side == 0.0 {
                    return Err(invalid(format!(
                        "{}: an aileron on the plane of symmetry",
                        spec.name
                    )));
                }
                covered = true;
                // The declared hinge is the right-hand one.
                let mirror =
                    |p: [f64; 3]| DVec3::new(p[0], if side < 0.0 { -p[1] } else { p[1] }, p[2]);
                let group = Group {
                    strips,
                    ratio: control.chord_ratio,
                    span: (control.span_from, control.span_to),
                    declared: control.hinge_m.map(|[a, b]| (mirror(a), mirror(b))),
                };
                match nodes.iter_mut().find(|(name, _)| *name == node) {
                    Some((_, groups)) => groups.push(group),
                    None => nodes.push((node, vec![group])),
                }
            }
            if !covered {
                return Err(invalid(format!(
                    "{}: a {:?} that covers no strip",
                    spec.name, control.control
                )));
            }
        }
        for (node, groups) in nodes {
            let hinge_index = hinges.len();
            let mut axis = DVec3::ZERO;
            let mut segments = Vec::new();
            for group in &groups {
                let mut points = Vec::new();
                let (mut along, mut chord, mut width) = (DVec3::ZERO, 0.0, 0.0);
                for &k in &group.strips {
                    let sign = flight[k]
                        .control
                        .map(|c| c.alpha_per_rad.signum())
                        .ok_or_else(|| invalid(format!("{node}: strip without control")))?;
                    let plate = &mut plates[first + k];
                    plate.control = Some((hinge_index, group.ratio));
                    along += sign * plate.width * plate.span_axis();
                    chord += plate.width * group.ratio * plate.chord;
                    width += plate.width;
                    points.push((generated.strips[k].along, plate.hinge(group.ratio), *plate));
                }
                // The strips' hinge, and its axis oriented so that a positive rotation is
                // the positive deflection.
                let strips = segment(&points, group.span);
                let along = along.normalize();
                let (ends, direction) = match group.declared {
                    None => (strips, along),
                    Some((a, b)) => {
                        let direction = (b - a).normalize();
                        let direction = direction * direction.dot(along).signum();
                        let offset = 0.5 * (a + b) - 0.5 * (strips.0 + strips.1);
                        let off = (offset - along * offset.dot(along)).length();
                        let limit = HINGE_GUARD_CHORD_FRACTION * chord / width;
                        let turn = direction.angle_between(along).to_degrees();
                        if off > limit || turn > HINGE_GUARD_DEG {
                            return Err(invalid(format!(
                                "{node}: the declared hinge is {off:.3} m and {turn:.1}° from \
                                 the strips' (at most {limit:.3} m, {HINGE_GUARD_CHORD_FRACTION} × \
                                 the control's mean chord, and {HINGE_GUARD_DEG}°)"
                            )));
                        }
                        ((a, b), direction)
                    }
                };
                axis += (ends.1 - ends.0).length() * direction;
                segments.push(ends);
            }
            // One straight hinge for the node, through the length-weighted middle of its
            // segments: on a tapered panel, a constant chord ratio puts the strips' hinge
            // points on a line that leans from the span axis; a declared hinge says where
            // the real one is.
            let axis = axis.normalize();
            let length: f64 = segments.iter().map(|(a, b)| (*b - *a).length()).sum();
            let centre = segments
                .iter()
                .map(|(a, b)| (*b - *a).length() * 0.5 * (*a + *b))
                .sum::<DVec3>()
                / length;
            let (low, high) = segments
                .iter()
                .flat_map(|(a, b)| [*a, *b])
                .map(|p| (p - centre).dot(axis))
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), t| {
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

/// 1 on the right, −1 on the left, 0 on the plane of symmetry.
fn side_of(y: f64) -> f64 {
    if y > 1e-9 {
        1.0
    } else if y < -1e-9 {
        -1.0
    } else {
        0.0
    }
}

/// The strips of one control on one side of one panel.
struct Group {
    strips: Vec<usize>,
    ratio: f64,
    /// Fractions of the panel, root to tip.
    span: (f64, f64),
    /// Ends of the declared hinge on this side (`hinge_m`).
    declared: Option<(DVec3, DVec3)>,
}

/// The hinge of one control on one panel, between the ends of its span. The hinge point
/// is linear along a straight panel (leading edge and chord are): fitted on the strips'
/// points by least squares, so that it does not depend on how many strips there are. A
/// single strip gives its own width along its span axis.
fn segment(points: &[(f64, DVec3, Plate)], span: (f64, f64)) -> (DVec3, DVec3) {
    if let [(_, point, plate)] = points {
        let half = plate.span_axis() * (0.5 * plate.width);
        return (*point - half, *point + half);
    }
    let n = points.len() as f64;
    let mean_t = points.iter().map(|p| p.0).sum::<f64>() / n;
    let mean_p = points.iter().map(|p| p.1).sum::<DVec3>() / n;
    let var = points.iter().map(|p| (p.0 - mean_t).powi(2)).sum::<f64>();
    let slope = points
        .iter()
        .map(|p| (p.1 - mean_p) * (p.0 - mean_t))
        .sum::<DVec3>()
        / var;
    let at = |t: f64| mean_p + slope * (t - mean_t);
    (at(span.0), at(span.1))
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
        // Each hinge runs the length of its control: both halves of the elevator, the
        // rudder from the bottom of the fuselage to the top of the fin.
        let reach = |name: &str| {
            let h = hinge(name);
            (h.line.1 - h.line.0).length()
        };
        assert!(
            (reach("ctrl_elevator") - 2.81).abs() < 0.01,
            "{}",
            reach("ctrl_elevator")
        );
        assert!(
            (reach("ctrl_rudder") - 1.09).abs() < 0.01,
            "{}",
            reach("ctrl_rudder")
        );
        assert!((reach("ctrl_aileron_r_upper") - 1.955).abs() < 0.01);
    }

    #[test]
    fn the_hinges_do_not_depend_on_the_number_of_strips() {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        let mut data = load_aircraft(&assets, "spad7").unwrap();
        let before = contract(&data).unwrap();
        // The first panel of each surface only: the rudder's two panels change weight.
        for surface in &mut data.file.surfaces {
            surface.panels[0].strips *= 2;
        }
        let after = contract(&data).unwrap();
        assert_eq!(before.hinges.len(), after.hinges.len());
        for (a, b) in before.hinges.iter().zip(&after.hinges) {
            assert_eq!(a.node, b.node);
            assert!((a.pivot - b.pivot).length() < 1e-3, "{a:?}\n{b:?}");
            assert!(a.axis.angle_between(b.axis) < 1e-6, "{a:?}\n{b:?}");
        }
    }

    /// The SPAD with declared hinges: the rudder on a vertical post at x `rudder_x`, its
    /// top `top_dx` further aft, and a straight elevator hinge at x −4.60.
    fn with_declared_hinges(rudder_x: f64, top_dx: f64) -> AircraftData {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        let mut data = load_aircraft(&assets, "spad7").unwrap();
        for surface in &mut data.file.surfaces {
            for control in &mut surface.controls {
                control.hinge_m = match control.control {
                    Control::Rudder => {
                        Some([[rudder_x, 0.0, -0.31], [rudder_x - top_dx, 0.0, -1.40]])
                    }
                    Control::Elevator => Some([[-4.60, 0.0, -0.70], [-4.60, 1.405, -0.70]]),
                    Control::Aileron => None,
                };
            }
        }
        data
    }

    #[test]
    fn a_declared_hinge_is_taken_as_it_is() {
        let c = contract(&with_declared_hinges(-4.75, 0.0)).unwrap();
        let hinge = |name: &str| c.hinges.iter().find(|h| h.node == name).unwrap();
        let rudder = hinge("ctrl_rudder");
        assert!((rudder.pivot.x + 4.75).abs() < 1e-12, "{rudder:?}");
        assert!((rudder.line.0.z + 0.31).abs() < 1e-12 && (rudder.line.1.z + 1.40).abs() < 1e-12);
        // Still oriented as the flight model deflects it: trailing edge to the right.
        assert!(rudder.axis.cross(DVec3::NEG_X).y > 0.999, "{rudder:?}");
        // The right-hand elevator hinge, mirrored for the left half.
        let elevator = hinge("ctrl_elevator");
        assert!((elevator.pivot - DVec3::new(-4.60, 0.0, -0.70)).length() < 1e-12);
        assert!(((elevator.line.1 - elevator.line.0).length() - 2.81).abs() < 1e-12);
        assert!(elevator.axis.cross(DVec3::NEG_X).z < -0.999, "{elevator:?}");
        // Ailerons without one keep the strips' hinge.
        assert_eq!(
            hinge("ctrl_aileron_r_upper"),
            spad()
                .hinges
                .iter()
                .find(|h| h.node == "ctrl_aileron_r_upper")
                .unwrap()
        );
    }

    #[test]
    fn a_declared_hinge_far_from_the_strips_is_refused() {
        // 0.52 m behind the strips' rudder hinge (x −4.68), a quarter of its mean chord
        // being about 0.13 m; then tilted by 20°.
        for data in [
            with_declared_hinges(-5.20, 0.0),
            with_declared_hinges(-4.60, 0.40),
        ] {
            let error = contract(&data).unwrap_err().to_string();
            assert!(error.contains("ctrl_rudder: the declared hinge"), "{error}");
        }
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
