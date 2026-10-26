//! `asset_check [aircraft…]`: checks the 3D models against the aircraft data and prints
//! each check (`ok*`: passed by an accepted deviation of `maquette.ron`); fails if one
//! does not pass. `asset_check <aircraft> --model <file.glb>` checks a model being built
//! as if it were the aircraft's `model.glb`.

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let assets = PathBuf::from("assets");
    let mut ids: Vec<String> = std::env::args().skip(1).collect();
    let model = ids
        .iter()
        .position(|a| a == "--model")
        .map(|i| (ids.remove(i), ids.get(i).cloned()));
    let reports = if let Some((_, path)) = model {
        match (path, ids.len()) {
            (Some(path), 2) => {
                ids.pop();
                asset_check::check_aircraft_model(&assets, &ids[0], &PathBuf::from(path))
                    .map(|r| vec![r])
            }
            _ => Err("usage: asset_check <aircraft> --model <file.glb>".to_owned()),
        }
    } else if ids.is_empty() {
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
