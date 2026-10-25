//! `asset_check [aircraft…]`: checks the 3D models against the aircraft data and prints
//! each check (`ok*`: passed by an accepted deviation of `maquette.ron`); fails if one
//! does not pass.

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let assets = PathBuf::from("assets");
    let ids: Vec<String> = std::env::args().skip(1).collect();
    let reports = if ids.is_empty() {
        asset_check::check_all(&assets)
    } else {
        ids.iter()
            .map(|id| {
                asset_check::check_aircraft(&assets, id)?
                    .ok_or_else(|| format!("{id}: no model.glb"))
            })
            .collect()
    };
    let reports = match reports {
        Ok(r) => r,
        Err(message) => {
            eprintln!("asset_check: {message}");
            return ExitCode::FAILURE;
        }
    };
    let mut passed = true;
    for report in &reports {
        println!("{}", report.aircraft);
        for c in &report.checks {
            println!(
                "  {} {:<22} {}",
                match (c.passed, c.accepted) {
                    (false, _) => "ÉCHEC",
                    (true, true) => "ok* ",
                    (true, false) => "ok  ",
                },
                c.name,
                c.detail
            );
        }
        passed &= report.passed();
    }
    if passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
