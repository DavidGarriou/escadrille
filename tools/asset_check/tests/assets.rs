//! `just test` checks every model of the game's assets (skill `pipeline-assets`).

use std::path::PathBuf;

#[test]
fn every_model_of_the_assets_passes() {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let reports = asset_check::check_all(&assets).unwrap();
    assert!(
        reports.iter().any(|r| r.aircraft == "spad7"),
        "the SPAD has a model: {reports:#?}"
    );
    for report in &reports {
        assert!(report.passed(), "{report:#?}");
    }
}
