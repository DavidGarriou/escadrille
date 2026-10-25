//! Checks the 3D models against the aircraft data (skill `pipeline-assets`, ADR 0006,
//! step 4 of `docs/plans/modele-3d-provisoire.md`). For each `assets/aircraft/<id>/model.glb`:
//! - the glTF is valid and holds the nodes of the model contract (`escadrille_data::model`);
//! - each control surface turns about its hinge (pivot within [`PIVOT_TOLERANCE_M`] of
//!   the hinge segment, axis within [`AXIS_TOLERANCE_DEG`]), the propeller about its
//!   axis, on the hub;
//! - the pilot's eye and the guns look forward;
//! - the overall dimensions match `aircraft.ron` within [`DIMENSION_TOLERANCE`]: span,
//!   length, and height in the nearer of the flying and the resting attitudes;
//! - the file is the project's own or is listed in `assets/LICENCES.md`.
//!
//! A mirrored, turned or shifted model fails: that is the error ADR 0006 guards against.
//! A dimension that a provisional model misses for a known reason passes if its
//! `maquette.ron` accepts it, at the value measured then (within
//! [`ACCEPTED_DEVIATION_TOLERANCE_M`]).

use std::path::Path;

use escadrille_data::aircraft::{AircraftData, Loading, load_aircraft};
use escadrille_data::frames::from_gltf;
use escadrille_data::mockup::{AcceptedDeviation, load_mockup};
use escadrille_data::model::{Contract, contract};
use glam::{DQuat, DVec3};

/// How far a pivot may be from its axis, m.
pub const PIVOT_TOLERANCE_M: f64 = 0.01;

/// How far an axis may turn from the data's, degrees.
pub const AXIS_TOLERANCE_DEG: f64 = 1.0;

/// How far the eye may look from straight ahead, degrees (a model may tilt it a little).
pub const EYE_TOLERANCE_DEG: f64 = 5.0;

/// Relative tolerance on the overall dimensions (`pipeline-assets`).
pub const DIMENSION_TOLERANCE: f64 = 0.01;

/// How far an accepted deviation may move before it must be looked at again, m.
pub const ACCEPTED_DEVIATION_TOLERANCE_M: f64 = 0.005;

const DIMENSIONS: [&str; 3] = ["span", "length", "height"];

/// One check and its outcome.
#[derive(Debug, Clone, PartialEq)]
pub struct Check {
    pub name: String,
    pub passed: bool,
    /// Passed only by an accepted deviation of `maquette.ron`.
    pub accepted: bool,
    pub detail: String,
}

/// The checks of one model.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub aircraft: String,
    pub checks: Vec<Check>,
}

impl Report {
    pub fn passed(&self) -> bool {
        self.checks.iter().all(|c| c.passed)
    }
}

#[derive(Default)]
struct Checks(Vec<Check>);

impl Checks {
    fn add(&mut self, name: &str, passed: bool, detail: String) {
        self.push(name, passed, false, detail);
    }

    fn push(&mut self, name: &str, passed: bool, accepted: bool, detail: String) {
        self.0.push(Check {
            name: name.to_owned(),
            passed,
            accepted,
            detail,
        });
    }
}

/// A node of the model, placed in body axes from the datum.
struct Placed {
    name: Option<String>,
    translation: DVec3,
    rotation: DQuat,
    vertices: Vec<DVec3>,
}

/// The model's nodes in body axes, parents' transforms applied.
fn placed_nodes(gltf: &gltf::Gltf) -> Result<Vec<Placed>, String> {
    let blob = gltf.blob.as_deref();
    let mut out = Vec::new();
    let mut stack: Vec<(gltf::Node, DVec3, DQuat, f64)> = Vec::new();
    let scene = gltf
        .default_scene()
        .or_else(|| gltf.scenes().next())
        .ok_or("no scene")?;
    for node in scene.nodes() {
        stack.push((node, DVec3::ZERO, DQuat::IDENTITY, 1.0));
    }
    while let Some((node, parent_t, parent_r, parent_s)) = stack.pop() {
        let (t, r, s) = node.transform().decomposed();
        if (s[0] - s[1]).abs() > 1e-6 || (s[1] - s[2]).abs() > 1e-6 || s[0] <= 0.0 {
            return Err(format!(
                "{}: non-uniform or negative scale {s:?} (apply the transforms)",
                node.name().unwrap_or("?")
            ));
        }
        let (t, r) = (
            DVec3::from(t.map(f64::from)),
            DQuat::from_array(r.map(f64::from)),
        );
        let scale = parent_s * f64::from(s[0]);
        let translation = parent_t + parent_r * (t * parent_s);
        let rotation = parent_r * r;
        let mut vertices = Vec::new();
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let positions = primitive
                    .reader(|_| blob)
                    .read_positions()
                    .ok_or("a primitive without positions")?;
                vertices.extend(positions.map(|p| {
                    from_gltf(translation + rotation * (DVec3::from(p.map(f64::from)) * scale))
                }));
            }
        }
        out.push(Placed {
            name: node.name().map(str::to_owned),
            translation: from_gltf(translation),
            rotation,
            vertices,
        });
        for child in node.children() {
            stack.push((child, translation, rotation, scale));
        }
    }
    Ok(out)
}

/// A local glTF direction of a node, in body axes.
fn direction(node: &Placed, local: DVec3) -> DVec3 {
    from_gltf(node.rotation * local)
}

fn angle_deg(a: DVec3, b: DVec3) -> f64 {
    a.normalize()
        .dot(b.normalize())
        .clamp(-1.0, 1.0)
        .acos()
        .to_degrees()
}

/// Distance from `p` to the segment from `a` to `b`: a pivot off the end of its hinge,
/// on the other wing for instance, is as wrong as one beside it.
fn distance_to_segment(p: DVec3, a: DVec3, b: DVec3) -> f64 {
    let ab = b - a;
    let t = if ab.length_squared() > 0.0 {
        ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (p - (a + ab * t)).length()
}

/// Height of a cloud of points (x, z down) resting on the ground: the lowest edge of
/// their side outline that has the centre of gravity above it (wheels and tail skid for a
/// tail-dragger), and the farthest point above that edge.
fn resting_height(points: &[(f64, f64)], cg_x: f64) -> Option<f64> {
    let mut sorted: Vec<(f64, f64)> = points.to_vec();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    // Monotone chain, the outline on the +z (ground) side.
    let mut hull: Vec<(f64, f64)> = Vec::new();
    for &p in &sorted {
        while hull.len() >= 2 {
            let (a, b) = (hull[hull.len() - 2], hull[hull.len() - 1]);
            let cross = (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
            if cross >= 0.0 {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(p);
    }
    let edge = hull.windows(2).find(|w| w[0].0 <= cg_x && cg_x <= w[1].0)?;
    let (a, b) = (edge[0], edge[1]);
    let (dx, dz) = (b.0 - a.0, b.1 - a.1);
    let length = (dx * dx + dz * dz).sqrt();
    // Signed distance above the edge (towards −z).
    points
        .iter()
        .map(|p| ((p.0 - a.0) * dz - (p.1 - a.1) * dx) / length)
        .map(f64::abs)
        .fold(None, |m: Option<f64>, d| Some(m.map_or(d, |m| m.max(d))))
}

/// Checks one aircraft's model; `None` when it has none.
pub fn check_aircraft(assets: &Path, id: &str) -> Result<Option<Report>, String> {
    let path = assets.join("aircraft").join(id).join("model.glb");
    if !path.exists() {
        return Ok(None);
    }
    let data = load_aircraft(assets, id).map_err(|e| e.to_string())?;
    let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let licences = std::fs::read_to_string(assets.join("LICENCES.md")).unwrap_or_default();
    let relative = format!("assets/aircraft/{id}/model.glb");
    let accepted = load_mockup(assets, id)
        .map_err(|e| e.to_string())?
        .map(|m| m.accepted_deviations)
        .unwrap_or_default();
    Ok(Some(check_model(
        &data, &bytes, &licences, &relative, &accepted,
    )?))
}

/// Checks a model (binary glTF) against its aircraft's data; `licences` is the text of
/// `assets/LICENCES.md`, `relative` the model's path as the register would list it,
/// `accepted` the deviations of its `maquette.ron`.
pub fn check_model(
    data: &AircraftData,
    glb: &[u8],
    licences: &str,
    relative: &str,
    accepted: &[AcceptedDeviation],
) -> Result<Report, String> {
    let mut checks = Checks::default();
    let gltf = match gltf::Gltf::from_slice(glb) {
        Ok(g) => g,
        Err(e) => {
            checks.add("glTF", false, e.to_string());
            return Ok(Report {
                aircraft: data.file.id.clone(),
                checks: checks.0,
            });
        }
    };
    checks.add("glTF", true, "valid".to_owned());
    let contract: Contract = contract(data).map_err(|e| e.to_string())?;
    let nodes = placed_nodes(&gltf)?;
    let find = |name: &str| nodes.iter().find(|n| n.name.as_deref() == Some(name));

    let missing: Vec<&String> = contract
        .required
        .iter()
        .filter(|r| find(r).is_none())
        .collect();
    checks.add(
        "nodes",
        missing.is_empty(),
        if missing.is_empty() {
            format!("{} required nodes", contract.required.len())
        } else {
            format!("missing: {missing:?}")
        },
    );

    for hinge in &contract.hinges {
        let Some(node) = find(&hinge.node) else {
            continue;
        };
        let off = distance_to_segment(node.translation, hinge.line.0, hinge.line.1);
        let turn = angle_deg(direction(node, DVec3::X), hinge.axis);
        checks.add(
            &hinge.node,
            off <= PIVOT_TOLERANCE_M && turn <= AXIS_TOLERANCE_DEG,
            format!(
                "pivot {:.1} mm off its hinge, axis {turn:.2}° off",
                off * 1e3
            ),
        );
    }

    let p = contract.propeller;
    for name in ["prop", "prop_blur"] {
        if let Some(node) = find(name) {
            let off = (node.translation - p.hub).length();
            let turn = angle_deg(direction(node, DVec3::X), p.axis);
            checks.add(
                name,
                off <= PIVOT_TOLERANCE_M && turn <= AXIS_TOLERANCE_DEG,
                format!("pivot {:.1} mm off the hub, axis {turn:.2}° off", off * 1e3),
            );
        }
    }

    for node in nodes.iter().filter(|n| {
        n.name
            .as_deref()
            .is_some_and(|s| s == "cam_pilot_eye" || s.starts_with("gun_"))
    }) {
        let name = node.name.clone().unwrap_or_default();
        let ahead = angle_deg(direction(node, DVec3::NEG_Z), DVec3::X);
        let upright = angle_deg(direction(node, DVec3::Y), DVec3::NEG_Z);
        let limit = if name == "cam_pilot_eye" {
            EYE_TOLERANCE_DEG
        } else {
            AXIS_TOLERANCE_DEG
        };
        checks.add(
            &name,
            ahead <= limit && upright <= EYE_TOLERANCE_DEG,
            format!("looks {ahead:.2}° off ahead, {upright:.2}° off upright"),
        );
    }

    match data.file.dimensions {
        None => checks.add(
            "dimensions",
            false,
            "aircraft.ron has no dimensions block".to_owned(),
        ),
        Some(d) => {
            let solid: Vec<DVec3> = nodes
                .iter()
                .filter(|n| !matches!(n.name.as_deref(), Some("prop" | "prop_blur")))
                .flat_map(|n| n.vertices.iter().copied())
                .collect();
            let (min, max) = solid.iter().fold(
                (DVec3::splat(f64::MAX), DVec3::splat(f64::MIN)),
                |(lo, hi), &v| (lo.min(v), hi.max(v)),
            );
            let mut dimension = |name: &str, model: f64, source: f64, detail: String| {
                let within = ((model - source) / source).abs() <= DIMENSION_TOLERANCE;
                let (passed, accepted, detail) = match accepted.iter().find(|a| a.check == name) {
                    None => (within, false, detail),
                    Some(_) if within => (
                        false,
                        false,
                        format!("{detail}; within tolerance: remove its accepted deviation"),
                    ),
                    Some(a)
                        if (model - a.measured.si()).abs() <= ACCEPTED_DEVIATION_TOLERANCE_M =>
                    {
                        (
                            true,
                            true,
                            format!("{detail}; accepted deviation: {}", a.reason),
                        )
                    }
                    Some(a) => (
                        false,
                        false,
                        format!(
                            "{detail}; the deviation was accepted at {:.3} m",
                            a.measured.si()
                        ),
                    ),
                };
                checks.push(name, passed, accepted, detail);
            };
            let (span, source) = (max.y - min.y, d.span.si());
            dimension(
                "span",
                span,
                source,
                format!("{span:.3} m against {source:.3} m"),
            );
            let (length, source) = (max.x.max(p.hub.x) - min.x, d.length.si());
            dimension(
                "length",
                length,
                source,
                format!("{length:.3} m (hub to tail) against {source:.3} m"),
            );
            let source = d.height.si();
            let flying = max.z - min.z;
            let cg_x = data
                .build(&Loading { fuel_fraction: 1.0 })
                .map_err(|e| e.to_string())?
                .summary
                .cg_from_datum
                .x;
            let side: Vec<(f64, f64)> = solid.iter().map(|v| (v.x, v.z)).collect();
            let resting = resting_height(&side, cg_x);
            let nearer = resting.map_or(flying, |r| {
                if (r - source).abs() < (flying - source).abs() {
                    r
                } else {
                    flying
                }
            });
            dimension(
                "height",
                nearer,
                source,
                format!(
                    "{flying:.3} m flying, {} at rest, against {source:.3} m",
                    resting.map_or("no resting attitude".to_owned(), |r| format!("{r:.3} m"))
                ),
            );
        }
    }
    let unknown: Vec<&str> = accepted
        .iter()
        .map(|a| a.check.as_str())
        .filter(|c| !DIMENSIONS.contains(c))
        .collect();
    if !unknown.is_empty() {
        checks.add(
            "accepted deviations",
            false,
            format!("only the dimensions can be accepted, not {unknown:?}"),
        );
    }

    let own = gltf
        .document
        .as_json()
        .asset
        .copyright
        .as_deref()
        .is_some_and(|c| c.contains("Panache blanc"));
    let listed = licences.contains(relative);
    checks.add(
        "licence",
        own || listed,
        if own {
            "the project's own (CC-BY-SA 4.0)".to_owned()
        } else if listed {
            "listed in assets/LICENCES.md".to_owned()
        } else {
            "neither the project's own nor listed in assets/LICENCES.md".to_owned()
        },
    );
    Ok(Report {
        aircraft: data.file.id.clone(),
        checks: checks.0,
    })
}

/// Checks every aircraft that has a model.
pub fn check_all(assets: &Path) -> Result<Vec<Report>, String> {
    let dir = assets.join("aircraft");
    let mut ids: Vec<String> = std::fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    ids.sort();
    let mut reports = Vec::new();
    for id in ids {
        if let Some(report) = check_aircraft(assets, &id)? {
            reports.push(report);
        }
    }
    Ok(reports)
}

#[cfg(test)]
mod tests;
