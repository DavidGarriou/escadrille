use std::borrow::Cow;
use std::path::PathBuf;

use escadrille_data::units::Length;

use super::*;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}

fn spad() -> (AircraftData, Vec<u8>) {
    let data = load_aircraft(&assets(), "spad7").unwrap();
    let glb = std::fs::read(assets().join("aircraft/spad7/model.glb")).unwrap();
    (data, glb)
}

/// The deviations that `maquette.ron` accepts for the SPAD.
fn accepted() -> Vec<AcceptedDeviation> {
    load_mockup(&assets(), "spad7")
        .unwrap()
        .unwrap()
        .accepted_deviations
}

fn failed(report: &Report) -> Vec<&str> {
    report
        .checks
        .iter()
        .filter(|c| !c.passed)
        .map(|c| c.name.as_str())
        .collect()
}

/// The same model with its JSON edited (the binary chunk kept).
fn edited(glb: &[u8], edit: impl FnOnce(&mut gltf::json::Root)) -> Vec<u8> {
    let parts = gltf::binary::Glb::from_slice(glb).unwrap();
    let mut root: gltf::json::Root = gltf::json::deserialize::from_slice(&parts.json).unwrap();
    edit(&mut root);
    let json = gltf::json::serialize::to_vec(&root).unwrap();
    gltf::binary::Glb {
        header: parts.header,
        json: Cow::Owned(json),
        bin: parts.bin,
    }
    .to_vec()
    .unwrap()
}

fn root_node(root: &mut gltf::json::Root) -> &mut gltf::json::Node {
    root.nodes
        .iter_mut()
        .find(|n| n.name.as_deref() == Some("spad7"))
        .unwrap()
}

#[test]
fn the_spad_model_passes_with_its_accepted_height() {
    let report = check_aircraft(&assets(), "spad7").unwrap().unwrap();
    assert!(report.passed(), "{report:#?}");
    let accepted: Vec<&str> = report
        .checks
        .iter()
        .filter(|c| c.accepted)
        .map(|c| c.name.as_str())
        .collect();
    assert_eq!(accepted, ["height"], "{report:#?}");
    // Without it, only the height fails.
    let (data, glb) = spad();
    let report = check_model(&data, &glb, "", "", &[], None).unwrap();
    assert_eq!(failed(&report), ["height"], "{report:#?}");
}

#[test]
fn an_accepted_deviation_holds_only_at_its_measured_value() {
    let (data, glb) = spad();
    let deviation = |check: &str, measured: f64| AcceptedDeviation {
        check: check.to_owned(),
        measured: Length::M(measured),
        reason: "test".to_owned(),
    };
    // A height that has moved by 2 cm since it was accepted.
    let moved = [deviation("height", 2.251)];
    let report = check_model(&data, &glb, "", "", &moved, None).unwrap();
    assert_eq!(failed(&report), ["height"], "{report:#?}");
    // A span and a length within tolerance must not keep a deviation.
    let stale = [
        deviation("span", 7.82),
        deviation("length", 6.037),
        deviation("height", 2.271),
    ];
    let report = check_model(&data, &glb, "", "", &stale, None).unwrap();
    assert_eq!(failed(&report), ["span", "length"], "{report:#?}");
    // Only the dimensions can be accepted.
    let pivot = [deviation("ctrl_rudder", 0.1)];
    let report = check_model(&data, &glb, "", "", &pivot, None).unwrap();
    assert!(
        failed(&report).contains(&"accepted deviations"),
        "{report:#?}"
    );
}

#[test]
fn a_mirrored_model_fails() {
    // Modelled with the right wing on the left: the ailerons are on the wrong side.
    let (data, glb) = spad();
    let mirrored = edited(&glb, |root| {
        for node in &mut root.nodes {
            node.name = node.name.take().map(|n| match n.as_str() {
                "ctrl_aileron_r_upper" => "ctrl_aileron_l_upper".to_owned(),
                "ctrl_aileron_l_upper" => "ctrl_aileron_r_upper".to_owned(),
                _ => n,
            });
        }
    });
    let report = check_model(&data, &mirrored, "", "", &accepted(), None).unwrap();
    let failed = failed(&report);
    assert!(failed.contains(&"ctrl_aileron_r_upper"), "{report:#?}");
    assert!(failed.contains(&"ctrl_aileron_l_upper"), "{report:#?}");
    // A mirror by a negative scale is refused outright.
    let flipped = edited(&glb, |root| root_node(root).scale = Some([-1.0, 1.0, 1.0]));
    assert!(check_model(&data, &flipped, "", "", &accepted(), None).is_err());
}

#[test]
fn an_aileron_on_the_wrong_wing_fails() {
    // The right aileron, turned the right way, but placed on the left wing: on the line of
    // its hinge, off its segment.
    let (data, glb) = spad();
    let moved = edited(&glb, |root| {
        let node = root
            .nodes
            .iter_mut()
            .find(|n| n.name.as_deref() == Some("ctrl_aileron_r_upper"))
            .unwrap();
        // glTF x is the body's −y (ADR 0006).
        let t = node.translation.as_mut().unwrap();
        t[0] = -t[0];
    });
    let report = check_model(&data, &moved, "", "", &accepted(), None).unwrap();
    assert_eq!(failed(&report), ["ctrl_aileron_r_upper"], "{report:#?}");
}

#[test]
fn a_model_turned_back_to_front_fails() {
    // As if the half-turn of ADR 0006 were applied twice.
    let (data, glb) = spad();
    let turned = edited(&glb, |root| {
        root_node(root).rotation = Some(gltf::json::scene::UnitQuaternion([0.0, 1.0, 0.0, 0.0]));
    });
    let report = check_model(&data, &turned, "", "", &accepted(), None).unwrap();
    let failed = failed(&report);
    for name in [
        "prop",
        "ctrl_elevator",
        "ctrl_rudder",
        "cam_pilot_eye",
        "gun_1_muzzle",
    ] {
        assert!(failed.contains(&name), "{name}: {report:#?}");
    }
}

#[test]
fn a_model_shifted_by_five_centimetres_fails() {
    let (data, glb) = spad();
    let shifted = edited(&glb, |root| {
        root_node(root).translation = Some([0.0, 0.0, 0.05])
    });
    let report = check_model(&data, &shifted, "", "", &accepted(), None).unwrap();
    let failed = failed(&report);
    assert!(
        failed.contains(&"prop") && failed.contains(&"ctrl_rudder"),
        "{report:#?}"
    );
}

#[test]
fn a_third_party_model_must_be_listed() {
    let (data, glb) = spad();
    let foreign = edited(&glb, |root| {
        root.asset.copyright = Some("someone".to_owned())
    });
    let path = "assets/aircraft/spad7/model.glb";
    let report = check_model(&data, &foreign, "", path, &accepted(), None).unwrap();
    assert_eq!(failed(&report), ["licence"]);
    let listed = format!("| `{path}` | someone | … | CC-BY 4.0 |");
    assert!(
        check_model(&data, &foreign, &listed, path, &accepted(), None)
            .unwrap()
            .passed()
    );
}

#[test]
fn the_resting_height_is_taken_above_the_wheels_and_the_skid() {
    // A rectangle 1 m high on a wheel at the front and a skid at the rear, its centre of
    // gravity between them: tilted by the skid, it stands a little lower than 1.3 m.
    let points = [
        (0.0, -1.0),
        (-4.0, -1.0),
        (-4.0, 0.0),
        (0.0, 0.0),
        (0.0, 0.3),
        (-4.0, 0.1),
    ];
    let h = resting_height(&points, -1.0).unwrap();
    let tilt = (0.2f64 / 4.0).atan();
    let expected = 1.3 * tilt.cos();
    assert!((h - expected).abs() < 1e-9, "{h} {expected}");
}

fn mockup() -> MockupFile {
    load_mockup(&assets(), "spad7").unwrap().unwrap()
}

#[test]
fn the_spad_model_is_ready_for_damage() {
    let report = check_aircraft(&assets(), "spad7").unwrap().unwrap();
    for name in ["damage nodes", "hit volumes"] {
        assert!(
            report.checks.iter().any(|c| c.name == name && c.passed),
            "{name}: {report:#?}"
        );
    }
    // Its contract: three panels per wing and side, two struts per station and side.
    let (data, _) = spad();
    let nodes = damage_nodes(&data, &mockup());
    for name in [
        "wing_upper_r_mid",
        "dmg_wing_lower_l_tip_stub",
        "hit_spar_wing_upper_l_root_rear",
        "strut_r_outer_front",
        "dmg_strut_l_inner_rear_broken",
        "tailplane_l",
        "fin",
        "gun_1",
        "hit_pilot_head",
        "hit_cable_aileron_r",
        "fx_cockpit",
    ] {
        assert!(nodes.iter().any(|n| n == name), "{name}");
    }
    assert_eq!(nodes.iter().filter(|n| n.starts_with("wing_")).count(), 12);
    assert_eq!(nodes.iter().filter(|n| n.starts_with("strut_")).count(), 8);
}

#[test]
fn a_model_without_a_damaged_variant_fails() {
    let (data, glb) = spad();
    let mockup = mockup();
    let report = check_model(&data, &glb, "", "", &accepted(), Some(&mockup)).unwrap();
    assert!(failed(&report).is_empty(), "{report:#?}");
    // A cut wire's variant, a stub, the pilot's head: each missing one fails.
    for gone in [
        "dmg_wire_flying_r_1_cut",
        "dmg_prop_blade_2_stub",
        "hit_pilot_head",
    ] {
        let glb = edited(&glb, |root| {
            let node = root
                .nodes
                .iter_mut()
                .find(|n| n.name.as_deref() == Some(gone))
                .unwrap();
            node.name = Some("renamed".to_owned());
        });
        let report = check_model(&data, &glb, "", "", &accepted(), Some(&mockup)).unwrap();
        assert_eq!(failed(&report), ["damage nodes"], "{gone}");
    }
    // Without the mockup's hit volumes, the model is not asked for damage.
    let report = check_model(&data, &glb, "", "", &accepted(), None).unwrap();
    assert!(report.checks.iter().all(|c| c.name != "damage nodes"));
}

/// The twelve triangles of a box from `lo` to `hi`, wound outwards.
fn cuboid(lo: DVec3, hi: DVec3) -> Vec<[DVec3; 3]> {
    let c = |x: usize, y: usize, z: usize| {
        DVec3::new([lo.x, hi.x][x], [lo.y, hi.y][y], [lo.z, hi.z][z])
    };
    let quads = [
        [c(0, 0, 0), c(0, 1, 0), c(1, 1, 0), c(1, 0, 0)],
        [c(0, 0, 1), c(1, 0, 1), c(1, 1, 1), c(0, 1, 1)],
        [c(0, 0, 0), c(1, 0, 0), c(1, 0, 1), c(0, 0, 1)],
        [c(0, 1, 0), c(0, 1, 1), c(1, 1, 1), c(1, 1, 0)],
        [c(0, 0, 0), c(0, 0, 1), c(0, 1, 1), c(0, 1, 0)],
        [c(1, 0, 0), c(1, 1, 0), c(1, 1, 1), c(1, 0, 1)],
    ];
    quads
        .iter()
        .flat_map(|q| [[q[0], q[1], q[2]], [q[0], q[2], q[3]]])
        .collect()
}

#[test]
fn a_hit_volume_is_a_closed_convex_solid() {
    let unit = cuboid(DVec3::ZERO, DVec3::ONE);
    assert_eq!(solid_fault(&unit), None);
    // An export splits the vertices along the edges: still closed once welded.
    let moved: Vec<_> = unit
        .iter()
        .map(|t| t.map(|v| v + DVec3::splat(1e-7)))
        .collect();
    assert_eq!(solid_fault(&moved), None);
    assert_eq!(solid_fault(&unit[1..]).as_deref(), Some("open"));
    let flat = cuboid(DVec3::ZERO, DVec3::new(1.0, 1.0, 0.0));
    assert_eq!(solid_fault(&flat).as_deref(), Some("flat"));
    // Two boxes in one mesh (two runs of a cable): each closed, together not convex.
    let mut two = unit.clone();
    two.extend(cuboid(DVec3::new(2.0, 0.0, 0.0), DVec3::new(3.0, 3.0, 1.0)));
    assert_eq!(solid_fault(&two).as_deref(), Some("concave"));
}
