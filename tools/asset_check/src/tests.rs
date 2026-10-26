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
    let report = check_model(&data, &glb, "", "", &[]).unwrap();
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
    let report = check_model(&data, &glb, "", "", &moved).unwrap();
    assert_eq!(failed(&report), ["height"], "{report:#?}");
    // A span and a length within tolerance must not keep a deviation.
    let stale = [
        deviation("span", 7.82),
        deviation("length", 6.037),
        deviation("height", 2.271),
    ];
    let report = check_model(&data, &glb, "", "", &stale).unwrap();
    assert_eq!(failed(&report), ["span", "length"], "{report:#?}");
    // Only the dimensions can be accepted.
    let pivot = [deviation("ctrl_rudder", 0.1)];
    let report = check_model(&data, &glb, "", "", &pivot).unwrap();
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
    let report = check_model(&data, &mirrored, "", "", &accepted()).unwrap();
    let failed = failed(&report);
    assert!(failed.contains(&"ctrl_aileron_r_upper"), "{report:#?}");
    assert!(failed.contains(&"ctrl_aileron_l_upper"), "{report:#?}");
    // A mirror by a negative scale is refused outright.
    let flipped = edited(&glb, |root| root_node(root).scale = Some([-1.0, 1.0, 1.0]));
    assert!(check_model(&data, &flipped, "", "", &accepted()).is_err());
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
    let report = check_model(&data, &moved, "", "", &accepted()).unwrap();
    assert_eq!(failed(&report), ["ctrl_aileron_r_upper"], "{report:#?}");
}

#[test]
fn a_model_turned_back_to_front_fails() {
    // As if the half-turn of ADR 0006 were applied twice.
    let (data, glb) = spad();
    let turned = edited(&glb, |root| {
        root_node(root).rotation = Some(gltf::json::scene::UnitQuaternion([0.0, 1.0, 0.0, 0.0]));
    });
    let report = check_model(&data, &turned, "", "", &accepted()).unwrap();
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
    let report = check_model(&data, &shifted, "", "", &accepted()).unwrap();
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
    let report = check_model(&data, &foreign, "", path, &accepted()).unwrap();
    assert_eq!(failed(&report), ["licence"]);
    let listed = format!("| `{path}` | someone | … | CC-BY 4.0 |");
    assert!(
        check_model(&data, &foreign, &listed, path, &accepted())
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
