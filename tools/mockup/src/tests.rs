use std::path::PathBuf;

use escadrille_data::aircraft::{Control, load_aircraft};
use escadrille_data::frames::from_gltf;
use escadrille_data::livery::{LiveryFile, MarkSpec, load_liveries};
use escadrille_data::mockup::load_mockup;
use glam::{DQuat, DVec3};

use super::*;

fn spad() -> (AircraftData, MockupFile) {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    (
        load_aircraft(&assets, "spad7").unwrap(),
        load_mockup(&assets, "spad7").unwrap().unwrap(),
    )
}

/// Every vertex of a node, in body axes from the datum.
fn vertices(gltf: &gltf::Gltf, name: &str) -> Vec<DVec3> {
    let node = gltf.nodes().find(|n| n.name() == Some(name)).unwrap();
    let (t, r, _) = node.transform().decomposed();
    let (t, r) = (
        DVec3::from(t.map(f64::from)),
        DQuat::from_array(r.map(f64::from)),
    );
    let blob = gltf.blob.as_deref();
    node.mesh()
        .into_iter()
        .flat_map(|m| m.primitives())
        .flat_map(|p| {
            p.reader(|_| blob)
                .read_positions()
                .unwrap()
                .collect::<Vec<_>>()
        })
        .map(|p| from_gltf(t + r * DVec3::from(p.map(f64::from))))
        .collect()
}

#[test]
fn the_spad_model_is_valid_gltf_with_the_nodes_of_the_contract() {
    let (data, mockup) = spad();
    let glb = build_model(&data, &mockup).unwrap();
    let gltf = gltf::Gltf::from_slice(&glb).unwrap();
    let names: Vec<&str> = gltf.nodes().filter_map(|n| n.name()).collect();
    for required in contract(&data).unwrap().required {
        assert!(names.contains(&required.as_str()), "{required}: {names:?}");
    }
    assert!(names.contains(&"spad7"));
    assert_eq!(gltf.scenes().count(), 1);
}

#[test]
fn the_model_is_the_same_every_time() {
    let (data, mockup) = spad();
    assert_eq!(
        build_model(&data, &mockup).unwrap(),
        build_model(&data, &mockup).unwrap()
    );
}

#[test]
fn the_wings_and_the_moving_parts_are_where_the_data_puts_them() {
    let (data, mockup) = spad();
    let gltf = gltf::Gltf::from_slice(&build_model(&data, &mockup).unwrap()).unwrap();
    // Wing tips at ±3.91 m: the right wing on the right (+y in body axes).
    let body = vertices(&gltf, "body");
    let (left, right) = body.iter().fold((f64::MAX, f64::MIN), |(lo, hi), p| {
        (lo.min(p.y), hi.max(p.y))
    });
    assert!(
        (right - 3.91).abs() < 1e-5 && (left + 3.91).abs() < 1e-5,
        "{left} {right}"
    );
    let aileron = vertices(&gltf, "ctrl_aileron_r_upper");
    assert!(
        aileron.iter().all(|p| p.y > 0.0 && p.z < -1.0),
        "upper right"
    );
    // Hinged nodes sit on their pivot, their local +X along the hinge axis.
    let c = contract(&data).unwrap();
    for h in &c.hinges {
        let node = gltf
            .nodes()
            .find(|n| n.name() == Some(h.node.as_str()))
            .unwrap();
        let (t, r, _) = node.transform().decomposed();
        let t = from_gltf(DVec3::from(t.map(f64::from)));
        let x = from_gltf(DQuat::from_array(r.map(f64::from)) * DVec3::X);
        assert!((t - h.pivot).length() < 1e-5, "{}: {t} {}", h.node, h.pivot);
        assert!(x.dot(h.axis) > 0.99999, "{}: {x} {}", h.node, h.axis);
    }
    // The propeller turns about +x (clockwise seen by the pilot), its pivot on the hub.
    let prop = gltf.nodes().find(|n| n.name() == Some("prop")).unwrap();
    let (t, r, _) = prop.transform().decomposed();
    let x = from_gltf(DQuat::from_array(r.map(f64::from)) * DVec3::X);
    assert!((from_gltf(DVec3::from(t.map(f64::from))) - c.propeller.hub).length() < 1e-5);
    assert!(x.dot(DVec3::X) > 0.99999, "{x}");
    // The pilot's eye looks forward (local −Z) with its top up (local +Y).
    let eye = gltf
        .nodes()
        .find(|n| n.name() == Some("cam_pilot_eye"))
        .unwrap();
    let r = DQuat::from_array(eye.transform().decomposed().1.map(f64::from));
    assert!(from_gltf(r * DVec3::NEG_Z).dot(DVec3::X) > 0.99999);
    assert!(from_gltf(r * DVec3::Y).dot(DVec3::NEG_Z) > 0.99999);
}

/// Asserts that every controlled plate is cut on its node's hinge line, and returns how
/// many are cut inside their chord.
fn cuts_on_the_hinges(contract: &Contract) -> usize {
    let mut cut_inside = 0;
    for plate in &contract.plates {
        let Some((h, ratio)) = plate.control else {
            continue;
        };
        let hinge = &contract.hinges[h];
        let t = cut(plate, hinge, ratio);
        assert!((0.0..=plate.chord).contains(&t));
        if t > 0.0 && t < plate.chord {
            cut_inside += 1;
            let point = plate.leading_edge() - plate.chord_axis * t;
            let off = (point - hinge.pivot).cross(hinge.axis).length();
            assert!(off < 1e-9, "{}: {off} m off", hinge.node);
        }
    }
    cut_inside
}

#[test]
fn each_plate_is_cut_on_the_hinge_line_of_its_node() {
    // The moving part turns about the line it is cut on: no gap, no overlap.
    // The strips' hinges first: without the hinges aircraft.ron declares.
    let (mut data, _) = spad();
    for surface in &mut data.file.surfaces {
        for control in &mut surface.controls {
            control.hinge_m = None;
        }
    }
    let from_strips = contract(&data).unwrap();
    assert!(cuts_on_the_hinges(&from_strips) > 20);
    // The strips' rudder hinge is vertical, through the length-weighted middle of its
    // segments, worked out here from the panels themselves: on an unswept panel the hinge
    // runs at x = quarter chord − (0.75 − chord ratio) × chord, linear from root to tip.
    let rudder = from_strips
        .hinges
        .iter()
        .find(|h| h.node == "ctrl_rudder")
        .unwrap();
    assert!(rudder.axis.z.abs() > 0.999, "{rudder:?}");
    let fin = data.file.surfaces.iter().find(|s| s.name == "fin").unwrap();
    let (mut moment, mut length) = (0.0, 0.0);
    for control in &fin.controls {
        let panel = &fin.panels[control.panel];
        let hinge_x = |end: [f64; 3], chord: f64| end[0] - (0.75 - control.chord_ratio) * chord;
        let (root, tip) = (
            hinge_x(panel.root_m, panel.chord_root.si()),
            hinge_x(panel.tip_m, panel.chord_tip.si()),
        );
        let l = (tip - root).hypot(panel.tip_m[2] - panel.root_m[2]);
        moment += l * 0.5 * (root + tip);
        length += l;
    }
    assert!(
        (rudder.pivot.x - moment / length).abs() < 1e-3,
        "{rudder:?}, expected x {}",
        moment / length
    );
    // A declared hinge moves the cut: the rudder on a post at x −4.75.
    for surface in &mut data.file.surfaces {
        for control in surface
            .controls
            .iter_mut()
            .filter(|c| c.control == Control::Rudder)
        {
            control.hinge_m = Some([[-4.75, 0.0, -0.31], [-4.75, 0.0, -1.40]]);
        }
    }
    let declared = contract(&data).unwrap();
    assert!(cuts_on_the_hinges(&declared) > 20);
    let rudder = declared
        .hinges
        .iter()
        .find(|h| h.node == "ctrl_rudder")
        .unwrap();
    assert!((rudder.pivot.x + 4.75).abs() < 1e-12, "{rudder:?}");
}

fn liveries() -> LiveryFile {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    load_liveries(&assets, "spad7").unwrap().unwrap()
}

#[test]
fn the_model_data_is_valid_json_with_the_contract() {
    let (data, mockup) = spad();
    let text = export::model_json(&data, Some(&mockup), Some(&liveries())).unwrap();
    let json: gltf::json::Value = gltf::json::deserialize::from_str(&text).unwrap();
    assert_eq!(json["version"], export::VERSION);
    assert_eq!(json["id"], "spad7");
    let hinges = json["hinges"].as_array().unwrap();
    let rudder = hinges.iter().find(|h| h["node"] == "ctrl_rudder").unwrap();
    assert_eq!(rudder["regions"].as_array().unwrap().len(), 5);
    assert!((rudder["pivot"][0].as_f64().unwrap() + 4.70).abs() < 1e-9);
    assert_eq!(json["surfaces"].as_array().unwrap().len(), 4);
    assert_eq!(json["propeller"]["axis"][0], 1.0);
    assert_eq!(json["mockup"]["fuselage"].as_array().unwrap().len(), 52);
    assert_eq!(json["mockup"]["features"]["radiator"][0][2], 0.67);
    // The drawn outlines of the tail, as (span, leading edge, trailing edge) rows.
    let outlines = &json["mockup"]["outlines"];
    assert_eq!(outlines["tailplane"].as_array().unwrap().len(), 51);
    assert_eq!(outlines["fin"].as_array().unwrap().len(), 55);
    assert!(outlines["fin"][0][1].as_f64().unwrap() > outlines["fin"][0][2].as_f64().unwrap());
    // An outline must name a surface of aircraft.ron.
    let mut stray = mockup.clone();
    stray.outlines[0].surface = "canard".to_owned();
    let error = export::model_json(&data, Some(&stray), None).unwrap_err();
    assert!(error.contains("canard"), "{error}");
    // Without the optional files, null.
    let bare: gltf::json::Value =
        gltf::json::deserialize::from_str(&export::model_json(&data, None, None).unwrap()).unwrap();
    assert!(bare["mockup"].is_null() && bare["liveries"].is_null());
}

#[test]
fn the_liveries_paint_faces_and_nodes_the_aircraft_has() {
    let (data, mockup) = spad();
    let text = export::model_json(&data, Some(&mockup), Some(&liveries())).unwrap();
    let json: gltf::json::Value = gltf::json::deserialize::from_str(&text).unwrap();
    let l = &json["liveries"];
    assert_eq!(l["colours"]["toile"][0], 0.84);
    let first = &l["liveries"][0];
    assert_eq!(first["name"], "usine-1917");
    assert_eq!(first["marks"][0]["kind"], "roundel");
    assert_eq!(first["marks"][0]["face"], "upper_wing.top");
    assert_eq!(first["marks"][0]["diameter"], 1.0);
    assert_eq!(first["marks"][4]["kind"], "bands");
    // A roundel under a fin, on a fuselage's side, on a surface the aircraft lacks.
    for (face, accepted) in [
        ("fin.bottom", false),
        ("fuselage.left", true),
        ("fin.left", true),
        ("canard.top", false),
    ] {
        let mut file = liveries();
        if let MarkSpec::Roundel { face: f, .. } = &mut file.liveries[0].marks[0] {
            *f = face.to_owned();
        }
        let result = export::model_json(&data, Some(&mockup), Some(&file));
        assert_eq!(result.is_ok(), accepted, "{face}: {result:?}");
    }
    // Bands on a node that is not a control.
    let mut file = liveries();
    if let MarkSpec::Bands { node, .. } = &mut file.liveries[0].marks[4] {
        *node = "prop".to_owned();
    }
    let error = export::model_json(&data, Some(&mockup), Some(&file)).unwrap_err();
    assert!(error.contains("not a control node"), "{error}");
}

#[test]
fn only_a_model_written_here_may_be_overwritten() {
    let (data, mockup) = spad();
    assert!(written_here(&build_model(&data, &mockup).unwrap()));
    assert!(!written_here(b"not a glTF"));
}
