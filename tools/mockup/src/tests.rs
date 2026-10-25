use std::path::PathBuf;

use escadrille_data::aircraft::load_aircraft;
use escadrille_data::frames::from_gltf;
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
