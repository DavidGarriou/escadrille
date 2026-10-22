use std::path::PathBuf;

use escadrille_fdm::aircraft::PilotControls;
use escadrille_fdm::environment::StandardEnvironment;
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::{DMat3, DQuat, EulerRot};

use super::*;
use crate::from_checked_str;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/assets")
}

fn fixture() -> AircraftData {
    load_aircraft(&assets(), "test_biplane").unwrap()
}

const FULL: Loading = Loading { fuel_fraction: 1.0 };

/// The fixture's text with `old` replaced by `new` (exactly once).
fn edited(old: &str, new: &str) -> AircraftData {
    let path = assets().join("aircraft/test_biplane/aircraft.ron");
    let text = std::fs::read_to_string(path).unwrap();
    assert_eq!(text.matches(old).count(), 1, "{old}");
    let file = from_checked_str(&text.replace(old, new), "edited").unwrap();
    AircraftData { file, ..fixture() }
}

fn invalid_message(data: &AircraftData) -> String {
    match data.build(&FULL) {
        Err(DataError::Invalid { message, .. }) => message,
        other => panic!("expected an invalid aircraft, got {other:?}"),
    }
}

#[test]
fn builds_the_test_biplane_with_its_mass_and_centre_of_gravity() {
    let built = fixture().build(&FULL).unwrap();
    let s = &built.summary;
    let fuel = 100.0 * 0.72;
    let total = 500.0 + 80.0 + fuel + 15.0;
    assert!((s.mass - total).abs() < 1e-12);
    let moment = DVec3::new(-0.3, 0.0, 0.1) * 500.0
        + DVec3::new(-1.2, 0.0, -0.3) * 80.0
        + DVec3::new(-0.4, 0.0, -0.4) * fuel
        + DVec3::new(0.3, 0.0, -0.6) * 15.0;
    assert!((s.cg_from_datum - moment / total).length() < 1e-12);
    assert_eq!(built.aircraft.mass().mass(), s.mass);

    // Areas and spans: upper wing 2 × 4 × 1.4, lower wing tip to tip across the gap.
    let (name, area, span, ar) = &s.surfaces[0];
    assert_eq!(name, "upper");
    assert!((area - 11.2).abs() < 1e-12 && (span - 8.0).abs() < 1e-12);
    assert!((ar - 64.0 / 11.2).abs() < 1e-12);
    let lower = &s.surfaces[1];
    assert!((lower.2 - 7.6).abs() < 1e-12, "{lower:?}");
    assert_eq!(s.surfaces[3].3, 1.6);
}

#[test]
fn inertia_obeys_the_parallel_axis_theorem() {
    // About the datum, the built inertia (rotor added back) must equal the sum of the
    // parts, each moved from its own reference point.
    let built = fixture().build(&FULL).unwrap();
    let mass = built.aircraft.mass();
    let rotor = mass.rotor().unwrap();
    let total = mass.airframe_inertia()
        + DMat3::from_cols(
            rotor.axis * rotor.axis.x,
            rotor.axis * rotor.axis.y,
            rotor.axis * rotor.axis.z,
        ) * rotor.inertia;
    let about = |m: f64, d: DVec3| point_inertia(m, d).tensor();
    let cg = built.summary.cg_from_datum;
    let datum = total + about(mass.mass(), cg);
    let parts = Inertia {
        ixx: 900.0,
        iyy: 1_100.0,
        izz: 1_900.0,
        ixz: 30.0,
    }
    .tensor()
        + about(500.0, DVec3::new(-0.3, 0.0, 0.1))
        + about(80.0, DVec3::new(-1.2, 0.0, -0.3))
        + about(72.0, DVec3::new(-0.4, 0.0, -0.4))
        + about(15.0, DVec3::new(0.3, 0.0, -0.6));
    for (a, b) in datum.to_cols_array().iter().zip(parts.to_cols_array()) {
        assert!((a - b).abs() < 1e-9, "{datum} vs {parts}");
    }
    // Less fuel, lighter.
    let empty_tanks = fixture().build(&Loading { fuel_fraction: 0.0 }).unwrap();
    assert!((built.summary.mass - empty_tanks.summary.mass - 72.0).abs() < 1e-12);
}

fn flying(speed: f64, alpha: f64) -> RigidBodyState {
    RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -1_000.0),
        attitude: DQuat::from_euler(EulerRot::ZYX, 0.0, alpha, 0.0),
        velocity_body: DVec3::new(speed * alpha.cos(), 0.0, speed * alpha.sin()),
        ..Default::default()
    }
}

#[test]
fn control_signs_come_from_the_geometry() {
    let aircraft = fixture().build(&FULL).unwrap().aircraft;
    let env = StandardEnvironment::default();
    let state = flying(45.0, 0.05);
    let neutral = PilotControls::default();
    let moment = |c: PilotControls| aircraft.loads(&state, &c, &env).wrench.moment;
    let base = moment(neutral);
    let roll = moment(PilotControls {
        roll: 0.5,
        ..neutral
    }) - base;
    let pitch = moment(PilotControls {
        pitch: 0.5,
        ..neutral
    }) - base;
    let yaw = moment(PilotControls {
        yaw: 0.5,
        ..neutral
    }) - base;
    assert!(
        roll.x > 0.0 && pitch.y > 0.0 && yaw.z > 0.0,
        "{roll} {pitch} {yaw}"
    );

    // Mirrored surfaces: no lateral load in a symmetric glide (rotor at rest).
    let w = aircraft.loads(&state, &neutral, &env).wrench;
    let scale = w.force.length();
    assert!(w.force.y.abs() < 1e-9 * scale && w.moment.x.abs() < 1e-9 * scale);
    assert!(w.moment.z.abs() < 1e-9 * scale, "{w:?}");
}

#[test]
fn refuses_numbers_without_origin() {
    let path = assets().join("aircraft/test_biplane/aircraft.ron");
    let text = std::fs::read_to_string(path).unwrap();
    let old = "pilot: Kg(80.0),                                          // E: synthetic";
    let text = text.replace(old, "pilot: Kg(80.0),");
    match from_checked_str::<AircraftFile>(&text, "edited") {
        Err(DataError::MissingOrigin { missing, .. }) => {
            assert_eq!(missing.len(), 1);
            assert_eq!(missing[0].text, "pilot: Kg(80.0),");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn refuses_an_area_far_from_the_research_sheet() {
    let data = edited(
        "reference_area: Some(M2(11.2))",
        "reference_area: Some(M2(12.0))",
    );
    assert!(invalid_message(&data).contains("upper"));
}

#[test]
fn refuses_inconsistent_references() {
    let data = edited(
        "(control: Aileron, span_from: 0.6, span_to: 1.0",
        "(control: Aileron, span_from: 0.6, span_to: 0.61",
    );
    assert!(invalid_message(&data).contains("covers no strip"));
    let data = edited(r#"tail: ["tailplane"]"#, r#"tail: ["stabiliser"]"#);
    assert!(invalid_message(&data).contains("unknown surface stabiliser"));
    assert!(fixture().build(&Loading { fuel_fraction: 1.5 }).is_err());
    let data = edited("normal_force: 0.035,", "normal_force: 0.3,");
    assert!(invalid_message(&data).contains("normal_force"));
}
