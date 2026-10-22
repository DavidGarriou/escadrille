//! Every aircraft of `assets/aircraft/` loads, builds into a flight model, has a valid
//! test file, and agrees with its mass build-up.

use std::path::PathBuf;

use escadrille_data::aircraft::{Loading, load_aircraft};
use escadrille_data::essais::load_essais;
use escadrille_data::masses::load_masses;

fn assets() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
}

fn aircraft_ids() -> Vec<String> {
    let mut ids: Vec<String> = std::fs::read_dir(assets().join("aircraft"))
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    ids.sort();
    ids
}

#[test]
fn every_aircraft_of_the_assets_is_consistent() {
    for id in aircraft_ids() {
        let data = load_aircraft(&assets(), &id).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(data.file.id, id);
        let essais = load_essais(&assets(), &id).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(essais.aircraft, id);
        for fuel_fraction in [0.0, essais.conditions.fuel_fraction, 1.0] {
            data.build(&Loading { fuel_fraction })
                .unwrap_or_else(|e| panic!("{id} at {fuel_fraction}: {e}"));
        }
        if let Some(masses) = load_masses(&assets(), &id).unwrap_or_else(|e| panic!("{id}: {e}")) {
            assert_eq!(masses.aircraft, id);
            assert_eq!(masses.datum, data.file.datum, "{id}: same datum");
            masses
                .check(&data.file.mass)
                .unwrap_or_else(|e| panic!("{id}: aircraft.ron and masses.ron differ: {e}"));
        }
    }
}
