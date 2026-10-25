//! Provisional 3D models (plan `docs/plans/modele-3d-provisoire.md`): boxes and plates at
//! the dimensions of `aircraft.ron` and `maquette.ron`, with the nodes of the model
//! contract (`escadrille_data::model`), written as binary glTF in the axes of ADR 0006.
//!
//! - The lifting surfaces are the flight model's strips, each drawn as a plate; a strip
//!   that carries a control surface is cut at its hinge, the rear part going to the
//!   control's node.
//! - Geometry is built in body axes from the datum and converted once, point by point,
//!   with `escadrille_data::frames::to_gltf`.
//! - Colours tell the sides apart on a capture: the right halves of the wings and of the
//!   tailplane are green, the left halves beige, the control surfaces orange.

use std::borrow::Cow;
use std::collections::BTreeMap;

use escadrille_data::aircraft::{AircraftData, SurfaceKind};
use escadrille_data::frames::to_gltf;
use escadrille_data::mockup::MockupFile;
use escadrille_data::model::{Contract, Hinge, Plate, contract};
use glam::{DMat3, DQuat, DVec3};
use gltf_json as json;
use json::validation::Checked::Valid;
use json::validation::USize64;

/// Thickness of the plates, fraction of the chord (E: display only).
const PLATE_THICKNESS: f64 = 0.06;

/// Interplane struts: chord, thickness, and their distance ahead of and behind the
/// station of `maquette.ron`, the front and rear spars (E: display only), m.
const STRUT_CHORD: f64 = 0.10;
const STRUT_THICKNESS: f64 = 0.035;
const STRUT_SPREAD: f64 = 0.30;

/// Cabane struts from the fuselage top to the upper plane: stations x and y (E:), m.
const CABANE_X: [f64; 2] = [-0.10, -0.70];
const CABANE_Y: f64 = 0.30;

/// Undercarriage legs: from the axle near each wheel to the fuselage bottom, ahead of and
/// behind the axle, and their thickness (E: display only), m.
const LEG_SPREAD: f64 = 0.35;
const LEG_ROOT_Y: f64 = 0.30;
const LEG_THICKNESS: f64 = 0.05;

/// Propeller blade chord and thickness, hub size (E: display only), m.
const BLADE_CHORD: f64 = 0.18;
const BLADE_THICKNESS: f64 = 0.03;
const HUB_SIZE: f64 = 0.20;

/// Sides of the round parts: wheels and the propeller disc.
const WHEEL_SIDES: usize = 16;
const DISC_SIDES: usize = 32;

/// Paints of the model, in the order of its materials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Paint {
    Fuselage,
    LeftWing,
    RightWing,
    Control,
    Propeller,
    Disc,
    Metal,
}

const PAINTS: [(Paint, &str, [f32; 4]); 7] = [
    (Paint::Fuselage, "fuselage", [0.55, 0.55, 0.52, 1.0]),
    (Paint::LeftWing, "aile gauche", [0.85, 0.80, 0.65, 1.0]),
    (Paint::RightWing, "aile droite", [0.40, 0.70, 0.40, 1.0]),
    (Paint::Control, "gouvernes", [0.55, 0.20, 0.70, 1.0]),
    (Paint::Propeller, "hélice", [0.35, 0.22, 0.12, 1.0]),
    (Paint::Disc, "disque d'hélice", [0.85, 0.85, 0.85, 0.25]),
    (Paint::Metal, "métal", [0.20, 0.20, 0.20, 1.0]),
];

/// Where a plate is cut for its control, from its leading edge along the chord: the point
/// of the chord nearest the node's hinge line, within the chord. On a tapered panel the
/// strips' own hinge points lean from that line; cutting there would open a gap or make
/// the parts overlap when the control turns. Parallel lines fall back on the strip's own
/// chord ratio.
fn cut(plate: &Plate, hinge: &Hinge, ratio: f64) -> f64 {
    let (aft, axis) = (-plate.chord_axis, hinge.axis);
    let from_pivot = plate.leading_edge() - hinge.pivot;
    let b = aft.dot(axis);
    let denominator = 1.0 - b * b;
    if denominator < 1e-9 {
        return (1.0 - ratio) * plate.chord;
    }
    let t = (b * axis.dot(from_pivot) - aft.dot(from_pivot)) / denominator;
    t.clamp(0.0, plate.chord)
}

/// Triangles of one paint in one node, in the node's axes (glTF).
#[derive(Debug, Default)]
struct Part {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

/// A node being built: its place in the model (glTF axes) and its parts.
#[derive(Debug)]
struct Node {
    name: String,
    translation: DVec3,
    rotation: DQuat,
    parts: BTreeMap<Paint, Part>,
}

impl Node {
    /// A node turning about `pivot` (body axes, from the datum), its local axes the glTF
    /// images of the body directions `x` and `y` (then z = x × y).
    fn new(name: &str, pivot: DVec3, x: DVec3, y: DVec3) -> Self {
        let (gx, gy) = (to_gltf(x).normalize(), to_gltf(y));
        let gy = (gy - gx * gy.dot(gx)).normalize();
        let rotation = DQuat::from_mat3(&DMat3::from_cols(gx, gy, gx.cross(gy))).normalize();
        Self {
            name: name.to_owned(),
            translation: to_gltf(pivot),
            rotation,
            parts: BTreeMap::new(),
        }
    }

    /// A node at the datum with the model's own axes.
    fn fixed(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            translation: DVec3::ZERO,
            rotation: DQuat::IDENTITY,
            parts: BTreeMap::new(),
        }
    }

    fn local(&self, p: DVec3) -> [f32; 3] {
        (self.rotation.inverse() * (to_gltf(p) - self.translation))
            .as_vec3()
            .to_array()
    }

    fn local_direction(&self, d: DVec3) -> [f32; 3] {
        (self.rotation.inverse() * to_gltf(d)).as_vec3().to_array()
    }

    /// A flat polygon (body axes), its normal on the side of `outward`.
    fn polygon(&mut self, paint: Paint, points: &[DVec3], outward: DVec3) {
        let mut normal = DVec3::ZERO;
        for i in 1..points.len() - 1 {
            normal += (points[i] - points[0]).cross(points[i + 1] - points[0]);
        }
        if normal.length() < 1e-12 {
            return;
        }
        let reversed = normal.dot(outward) < 0.0;
        let normal = if reversed { -normal } else { normal }.normalize();
        let ordered: Vec<DVec3> = if reversed {
            points.iter().rev().copied().collect()
        } else {
            points.to_vec()
        };
        let n = self.local_direction(normal);
        let local: Vec<[f32; 3]> = ordered.iter().map(|&p| self.local(p)).collect();
        let part = self.parts.entry(paint).or_default();
        let base = part.positions.len() as u32;
        for p in local {
            part.positions.push(p);
            part.normals.push(n);
        }
        for i in 1..ordered.len() as u32 - 1 {
            part.indices.extend([base, base + i, base + i + 1]);
        }
    }

    /// A box (body axes): centre and three half-axes.
    fn cuboid(&mut self, paint: Paint, centre: DVec3, half: [DVec3; 3]) {
        for k in 0..3 {
            let (a, u, v) = (half[k], half[(k + 1) % 3], half[(k + 2) % 3]);
            for side in [1.0, -1.0] {
                let c = centre + a * side;
                self.polygon(
                    paint,
                    &[c - u - v, c + u - v, c + u + v, c - u + v],
                    a * side,
                );
            }
        }
    }

    /// A bar from `a` to `b` (body axes), square section of side `thickness`.
    fn bar(&mut self, paint: Paint, a: DVec3, b: DVec3, thickness: f64) {
        let along = 0.5 * (b - a);
        let (u, v) = perpendiculars(along.normalize());
        let t = 0.5 * thickness;
        self.cuboid(paint, 0.5 * (a + b), [along, u * t, v * t]);
    }

    /// A prism of `sides` sides about `axis` (body axes), centred on `centre`.
    fn prism(&mut self, paint: Paint, centre: DVec3, axis: DVec3, radius: f64, length: f64) {
        let sides = if paint == Paint::Disc {
            DISC_SIDES
        } else {
            WHEEL_SIDES
        };
        let axis = axis.normalize();
        let (u, v) = perpendiculars(axis);
        let ring: Vec<DVec3> = (0..sides)
            .map(|i| {
                let t = std::f64::consts::TAU * i as f64 / sides as f64;
                (u * t.cos() + v * t.sin()) * radius
            })
            .collect();
        let half = axis * (0.5 * length);
        for i in 0..sides {
            let (p, q) = (ring[i], ring[(i + 1) % sides]);
            let quad = [
                centre + p - half,
                centre + q - half,
                centre + q + half,
                centre + p + half,
            ];
            self.polygon(paint, &quad, p + q);
        }
        for side in [1.0, -1.0] {
            let cap: Vec<DVec3> = ring.iter().map(|&p| centre + p + half * side).collect();
            self.polygon(paint, &cap, axis * side);
        }
    }
}

/// Two unit vectors perpendicular to `axis` and to each other.
fn perpendiculars(axis: DVec3) -> (DVec3, DVec3) {
    let helper = if axis.z.abs() < 0.9 {
        DVec3::Z
    } else {
        DVec3::X
    };
    let u = axis.cross(helper).normalize();
    (u, axis.cross(u))
}

/// Linear interpolation of the fuselage sections' bottom at `x` (held at the ends).
fn fuselage_bottom(mockup: &MockupFile, x: f64) -> f64 {
    let s = &mockup.fuselage;
    if x >= s[0].x_m {
        return s[0].bottom_m;
    }
    for w in s.windows(2) {
        if x >= w[1].x_m {
            let t = (w[0].x_m - x) / (w[0].x_m - w[1].x_m);
            return w[0].bottom_m + t * (w[1].bottom_m - w[0].bottom_m);
        }
    }
    s[s.len() - 1].bottom_m
}

/// Same for the top.
fn fuselage_top(mockup: &MockupFile, x: f64) -> f64 {
    let s = &mockup.fuselage;
    if x >= s[0].x_m {
        return s[0].top_m;
    }
    for w in s.windows(2) {
        if x >= w[1].x_m {
            let t = (w[0].x_m - x) / (w[0].x_m - w[1].x_m);
            return w[0].top_m + t * (w[1].top_m - w[0].top_m);
        }
    }
    s[s.len() - 1].top_m
}

/// Height (z) of a wing's plates at span `y`: the plate nearest in y.
fn wing_z(contract: &Contract, surface: usize, y: f64) -> Option<f64> {
    contract
        .plates
        .iter()
        .filter(|p| p.surface == surface)
        .min_by(|a, b| {
            (a.position.y - y)
                .abs()
                .total_cmp(&(b.position.y - y).abs())
        })
        .map(|p| p.position.z)
}

/// The provisional model of an aircraft, as binary glTF.
pub fn build_model(data: &AircraftData, mockup: &MockupFile) -> Result<Vec<u8>, String> {
    let file = &data.file;
    let contract = contract(data).map_err(|e| e.to_string())?;
    let mut body = Node::fixed("body");
    let up = DVec3::NEG_Z;

    // Fuselage: rectangular sections lofted from the nose to the tail post.
    let corners = |s: &escadrille_data::mockup::SectionSpec| {
        let w = 0.5 * s.width.si();
        [
            DVec3::new(s.x_m, -w, s.top_m),
            DVec3::new(s.x_m, w, s.top_m),
            DVec3::new(s.x_m, w, s.bottom_m),
            DVec3::new(s.x_m, -w, s.bottom_m),
        ]
    };
    for w in mockup.fuselage.windows(2) {
        let (a, b) = (corners(&w[0]), corners(&w[1]));
        let outward = [DVec3::NEG_Z, DVec3::Y, DVec3::Z, DVec3::NEG_Y];
        for k in 0..4 {
            let j = (k + 1) % 4;
            body.polygon(Paint::Fuselage, &[a[k], a[j], b[j], b[k]], outward[k]);
        }
    }
    let (first, last) = (
        &mockup.fuselage[0],
        &mockup.fuselage[mockup.fuselage.len() - 1],
    );
    body.polygon(Paint::Fuselage, &corners(first), DVec3::X);
    body.polygon(Paint::Fuselage, &corners(last), DVec3::NEG_X);

    // Lifting surfaces: one plate per strip, cut where the hinge line of its control's
    // node crosses its chord, so that the moving part turns about the line it is cut on.
    let mut controls: Vec<Node> = contract
        .hinges
        .iter()
        .map(|h| {
            let reference = if h.axis.z.abs() > 0.9 { DVec3::X } else { up };
            Node::new(&h.node, h.pivot, h.axis, reference)
        })
        .collect();
    for plate in &contract.plates {
        let spec = &file.surfaces[plate.surface];
        let paint = match spec.kind {
            SurfaceKind::Vertical => Paint::Fuselage,
            SurfaceKind::Horizontal if plate.position.y > 1e-9 => Paint::RightWing,
            SurfaceKind::Horizontal => Paint::LeftWing,
        };
        let (c, n, s) = (plate.chord_axis, plate.normal, plate.span_axis());
        let thick = n * (0.5 * PLATE_THICKNESS * plate.chord);
        let span = s * (0.5 * plate.width);
        let leading = plate.leading_edge();
        let trailing = leading - c * plate.chord;
        let slab = |node: &mut Node, paint: Paint, from: DVec3, to: DVec3| {
            let length = (to - from).dot(c);
            if length > 1e-9 {
                node.cuboid(paint, 0.5 * (from + to), [c * (0.5 * length), span, thick]);
            }
        };
        match plate.control {
            None => slab(&mut body, paint, trailing, leading),
            Some((h, ratio)) => {
                let hinge = leading - c * cut(plate, &contract.hinges[h], ratio);
                slab(&mut body, paint, hinge, leading);
                slab(&mut controls[h], Paint::Control, trailing, hinge);
            }
        }
    }

    // Interplane and cabane struts, between the two wings when there are two.
    let wings: Vec<usize> = file
        .downwash
        .as_ref()
        .map(|d| {
            d.wings
                .iter()
                .filter_map(|w| file.surfaces.iter().position(|s| s.name == *w))
                .collect()
        })
        .unwrap_or_default();
    if let [a, b] = wings[..] {
        for strut in &mockup.struts {
            for side in [1.0, -1.0] {
                let y = strut.y_m * side;
                let (Some(za), Some(zb)) = (wing_z(&contract, a, y), wing_z(&contract, b, y))
                else {
                    continue;
                };
                for dx in [STRUT_SPREAD, -STRUT_SPREAD] {
                    let x = strut.x_m + dx;
                    let half = [
                        DVec3::X * (0.5 * STRUT_CHORD),
                        DVec3::Y * (0.5 * STRUT_THICKNESS),
                        DVec3::Z * (0.5 * (za - zb).abs()),
                    ];
                    body.cuboid(Paint::Metal, DVec3::new(x, y, 0.5 * (za + zb)), half);
                }
            }
        }
        let upper = if wing_z(&contract, a, 0.0) < wing_z(&contract, b, 0.0) {
            a
        } else {
            b
        };
        for x in CABANE_X {
            for side in [1.0, -1.0] {
                let y = CABANE_Y * side;
                if let Some(z) = wing_z(&contract, upper, y) {
                    let top = fuselage_top(mockup, x);
                    body.bar(
                        Paint::Metal,
                        DVec3::new(x, y, top),
                        DVec3::new(x, y, z),
                        STRUT_THICKNESS,
                    );
                }
            }
        }
    }

    // Undercarriage: wheels, axle, legs, and the tail skid.
    let w = &mockup.wheels;
    let centre = DVec3::from_array(w.centre_m);
    let radius = 0.5 * w.diameter.si();
    for side in [1.0, -1.0] {
        let c = DVec3::new(centre.x, centre.y * side, centre.z);
        body.prism(Paint::Metal, c, DVec3::Y, radius, w.width.si());
        let foot = DVec3::new(centre.x, (centre.y - 0.1) * side, centre.z);
        for dx in [LEG_SPREAD, -LEG_SPREAD] {
            let x = centre.x + dx;
            let root = DVec3::new(x, LEG_ROOT_Y * side, fuselage_bottom(mockup, x));
            body.bar(Paint::Metal, foot, root, LEG_THICKNESS);
        }
    }
    body.bar(
        Paint::Metal,
        DVec3::new(centre.x, centre.y, centre.z),
        DVec3::new(centre.x, -centre.y, centre.z),
        LEG_THICKNESS,
    );
    let skid = DVec3::from_array(mockup.skid_m);
    body.bar(
        Paint::Metal,
        DVec3::new(skid.x, 0.0, fuselage_bottom(mockup, skid.x)),
        skid,
        LEG_THICKNESS,
    );

    // Propeller: two blades and a hub; its disc as a separate node.
    let p = contract.propeller;
    let mut prop = Node::new("prop", p.hub, p.axis, up);
    for side in [1.0, -1.0] {
        let tip = p.hub + up * (p.radius * side);
        let half = [
            (tip - p.hub) * 0.5,
            DVec3::Y * (0.5 * BLADE_CHORD),
            DVec3::X * (0.5 * BLADE_THICKNESS),
        ];
        prop.cuboid(Paint::Propeller, 0.5 * (p.hub + tip), half);
    }
    let h = 0.5 * HUB_SIZE;
    prop.cuboid(
        Paint::Propeller,
        p.hub,
        [DVec3::X * h, DVec3::Y * h, DVec3::Z * h],
    );
    let mut blur = Node::new("prop_blur", p.hub, p.axis, up);
    blur.prism(Paint::Disc, p.hub, DVec3::X, p.radius, 0.01);

    // Pilot's eye and guns: local −Z forward, +Y up, as glTF cameras.
    let looking = |name: &str, at: [f64; 3]| Node::new(name, DVec3::from_array(at), DVec3::Y, up);
    let mut nodes = vec![body, prop, blur];
    nodes.append(&mut controls);
    nodes.push(looking("cam_pilot_eye", mockup.pilot_eye_m));
    for (k, muzzle) in mockup.gun_muzzles_m.iter().enumerate() {
        nodes.push(looking(&format!("gun_{}_muzzle", k + 1), *muzzle));
    }
    write_glb(&file.id, &nodes)
}

/// The nodes under one root node named after the aircraft, as binary glTF.
fn write_glb(id: &str, nodes: &[Node]) -> Result<Vec<u8>, String> {
    let mut root = json::Root {
        asset: json::Asset {
            generator: Some("Panache blanc, tools/mockup".to_owned()),
            copyright: Some("Panache blanc, CC-BY-SA 4.0 (LICENSE-ASSETS)".to_owned()),
            ..Default::default()
        },
        ..Default::default()
    };
    let materials: BTreeMap<Paint, json::Index<json::Material>> = PAINTS
        .iter()
        .map(|&(paint, name, colour)| {
            let material = json::Material {
                name: Some(name.to_owned()),
                alpha_mode: Valid(if colour[3] < 1.0 {
                    json::material::AlphaMode::Blend
                } else {
                    json::material::AlphaMode::Opaque
                }),
                double_sided: paint == Paint::Disc,
                pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                    base_color_factor: json::material::PbrBaseColorFactor(colour),
                    metallic_factor: json::material::StrengthFactor(0.0),
                    roughness_factor: json::material::StrengthFactor(0.8),
                    ..Default::default()
                },
                ..Default::default()
            };
            (paint, root.push(material))
        })
        .collect();

    let mut bin: Vec<u8> = Vec::new();
    let buffer = json::Index::new(0);
    let view =
        |root: &mut json::Root, bin: &mut Vec<u8>, bytes: Vec<u8>, target: json::buffer::Target| {
            let (offset, length) = (bin.len(), bytes.len());
            bin.extend(bytes);
            while !bin.len().is_multiple_of(4) {
                bin.push(0);
            }
            root.push(json::buffer::View {
                buffer,
                byte_length: USize64::from(length),
                byte_offset: Some(USize64::from(offset)),
                byte_stride: None,
                extensions: Default::default(),
                extras: Default::default(),
                name: None,
                target: Some(Valid(target)),
            })
        };
    let floats = |v: &[[f32; 3]]| -> Vec<u8> {
        v.iter()
            .flat_map(|p| p.iter().flat_map(|f| f.to_le_bytes()))
            .collect()
    };
    let mut children = Vec::new();
    for node in nodes {
        let mut primitives = Vec::new();
        for (paint, part) in &node.parts {
            let count = part.positions.len();
            let (mut min, mut max) = ([f32::MAX; 3], [f32::MIN; 3]);
            for p in &part.positions {
                for (k, &v) in p.iter().enumerate() {
                    min[k] = min[k].min(v);
                    max[k] = max[k].max(v);
                }
            }
            let accessor = |root: &mut json::Root,
                            view: json::Index<json::buffer::View>,
                            count: usize,
                            kind: json::accessor::Type,
                            component: json::accessor::ComponentType,
                            bounds: Option<([f32; 3], [f32; 3])>| {
                root.push(json::Accessor {
                    buffer_view: Some(view),
                    byte_offset: Some(USize64(0)),
                    count: USize64::from(count),
                    component_type: Valid(json::accessor::GenericComponentType(component)),
                    extensions: Default::default(),
                    extras: Default::default(),
                    type_: Valid(kind),
                    min: bounds.map(|(lo, _)| json::Value::from(Vec::from(lo))),
                    max: bounds.map(|(_, hi)| json::Value::from(Vec::from(hi))),
                    name: None,
                    normalized: false,
                    sparse: None,
                })
            };
            let target = json::buffer::Target::ArrayBuffer;
            let positions_view = view(&mut root, &mut bin, floats(&part.positions), target);
            let normals_view = view(&mut root, &mut bin, floats(&part.normals), target);
            let index_bytes = part.indices.iter().flat_map(|i| i.to_le_bytes()).collect();
            let indices_view = view(
                &mut root,
                &mut bin,
                index_bytes,
                json::buffer::Target::ElementArrayBuffer,
            );
            let f32s = json::accessor::ComponentType::F32;
            let positions = accessor(
                &mut root,
                positions_view,
                count,
                json::accessor::Type::Vec3,
                f32s,
                Some((min, max)),
            );
            let normals = accessor(
                &mut root,
                normals_view,
                count,
                json::accessor::Type::Vec3,
                f32s,
                None,
            );
            let indices = accessor(
                &mut root,
                indices_view,
                part.indices.len(),
                json::accessor::Type::Scalar,
                json::accessor::ComponentType::U32,
                None,
            );
            let mut attributes = BTreeMap::new();
            attributes.insert(Valid(json::mesh::Semantic::Positions), positions);
            attributes.insert(Valid(json::mesh::Semantic::Normals), normals);
            primitives.push(json::mesh::Primitive {
                attributes,
                extensions: Default::default(),
                extras: Default::default(),
                indices: Some(indices),
                material: Some(materials[paint]),
                mode: Valid(json::mesh::Mode::Triangles),
                targets: None,
            });
        }
        let mesh = (!primitives.is_empty()).then(|| {
            root.push(json::Mesh {
                extensions: Default::default(),
                extras: Default::default(),
                name: Some(node.name.clone()),
                primitives,
                weights: None,
            })
        });
        let (t, r) = (node.translation.as_vec3(), node.rotation.as_quat());
        children.push(
            root.push(json::Node {
                mesh,
                name: Some(node.name.clone()),
                translation: (t != glam::Vec3::ZERO).then(|| t.to_array()),
                rotation: (node.rotation != DQuat::IDENTITY)
                    .then(|| json::scene::UnitQuaternion(r.to_array())),
                ..Default::default()
            }),
        );
    }
    let model = root.push(json::Node {
        name: Some(id.to_owned()),
        children: Some(children),
        ..Default::default()
    });
    let scene = root.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: Some(id.to_owned()),
        nodes: vec![model],
    });
    root.scene = Some(scene);
    root.push(json::Buffer {
        byte_length: USize64::from(bin.len()),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: None,
    });
    let text = json::serialize::to_vec(&root).map_err(|e| e.to_string())?;
    gltf::binary::Glb {
        header: gltf::binary::Header {
            magic: *b"glTF",
            version: 2,
            length: 0,
        },
        json: Cow::Owned(text),
        bin: Some(Cow::Owned(bin)),
    }
    .to_vec()
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests;
