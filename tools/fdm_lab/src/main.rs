//! `fdm_lab`: flight tests without rendering, on top of `escadrille_fdm` (M1 plan,
//! step 6). `just essai <id>` runs the campaign of `assets/aircraft/<id>/essais.ron` and
//! writes `target/essais/<id>/rapport.md`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use escadrille_data::aircraft::{Built, Loading, load_aircraft};
use escadrille_data::essais::{Confidence, Quantitative, load_essais};
use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::environment::{Environment, StandardEnvironment};
use fdm_lab::campaign::{self, validated};
use fdm_lab::modes::{Mode, lateral, linearise, longitudinal, static_stability};
use fdm_lab::performance::max_level_speed;
use fdm_lab::report::{Cruise, Report, markdown, time_step_change};
use fdm_lab::trim::{Flight, LEVEL_AT_SPEED, Trim, solve_robust};
use fdm_lab::{qualitative, robustness};

#[derive(Debug, Parser)]
#[command(version, about = "Headless flight-test laboratory for Panache blanc")]
struct Cli {
    /// Root of the data files.
    #[arg(long, global = true, default_value = "assets")]
    assets: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Find the trimmed level flight of an aircraft at a given speed and altitude.
    Trim {
        aircraft: String,
        #[arg(long, default_value_t = 150.0)]
        speed_kmh: f64,
        #[arg(long, default_value_t = 2_000.0)]
        altitude_m: f64,
    },
    /// Run the flight-test campaign (essais.ron) of an aircraft and write the report.
    Essai {
        aircraft: String,
        /// Lowest confidence that blocks the validation (M1: C, player's decision).
        #[arg(long, default_value = "C")]
        confidence: String,
        #[arg(long, default_value = "target/essais")]
        out: PathBuf,
    },
    /// Compute the linearised eigenmodes (phugoid, short period, dutch roll...).
    Modes {
        aircraft: String,
        #[arg(long, default_value_t = 150.0)]
        speed_kmh: f64,
        #[arg(long, default_value_t = 2_000.0)]
        altitude_m: f64,
    },
    /// Print the ISA sea-level constants used by the FDM.
    Constants,
}

const STILL_AIR: StandardEnvironment = StandardEnvironment {
    wind_ned: glam::DVec3::ZERO,
    ground_altitude: 0.0,
};

fn build(assets: &Path, id: &str, fuel_fraction: f64) -> Result<Built, String> {
    load_aircraft(assets, id)
        .and_then(|data| data.build(&Loading { fuel_fraction }))
        .map_err(|e| e.to_string())
}

fn level(
    aircraft: &Aircraft,
    env: &dyn Environment,
    speed: f64,
    altitude: f64,
) -> Result<Trim, String> {
    solve_robust(
        aircraft,
        env,
        Flight::guess(altitude, speed, 0.5),
        LEVEL_AT_SPEED,
    )
    .map_err(|e| format!("pas de vol en palier à {:.1} km/h : {e:?}", speed * 3.6))
}

fn print_modes(title: &str, modes: &[Mode]) {
    println!("{title}");
    for m in modes {
        println!(
            "  {:<26} λ = {:+.4} {:+.4} i   ζ = {:+.3}   période = {}   état dominant {}",
            m.name,
            m.eigenvalue.re,
            m.eigenvalue.im,
            m.damping,
            m.period.map_or("—".to_owned(), |p| format!("{p:.2} s")),
            m.dominant
        );
    }
}

fn essai(assets: &Path, id: &str, confidence: &str, out: &Path) -> Result<bool, String> {
    let accepted = match confidence {
        "A" => Confidence::A,
        "B" => Confidence::B,
        "C" => Confidence::C,
        "D" => Confidence::D,
        other => return Err(format!("niveau de confiance inconnu : {other}")),
    };
    let essais = load_essais(assets, id).map_err(|e| e.to_string())?;
    let built = build(assets, id, essais.conditions.fuel_fraction)?;
    let data = load_aircraft(assets, id).map_err(|e| e.to_string())?;
    let aircraft = &built.aircraft;
    let outcomes = campaign::run(aircraft, &STILL_AIR, &essais, accepted);

    // Cruise for the modes: 80 % of the maximum speed at the first speed-test altitude.
    let altitude = essais
        .quantitative
        .iter()
        .find_map(|q| match q {
            Quantitative::MaxLevelSpeed { altitude, .. } => Some(altitude.si()),
            _ => None,
        })
        .unwrap_or(2_000.0);
    let cruise = max_level_speed(aircraft, &STILL_AIR, altitude)
        .ok()
        .and_then(|vmax| level(aircraft, &STILL_AIR, 0.8 * vmax.flight.speed, altitude).ok())
        .map(|trim| {
            let linear = linearise(aircraft, &STILL_AIR, &trim);
            Cruise {
                commands: trim.commands(aircraft),
                longitudinal: longitudinal(&linear),
                lateral: lateral(&linear),
                stability: static_stability(aircraft, &linear),
                trim,
            }
        });
    let robustness: Vec<_> = essais
        .robustness
        .iter()
        .map(|scenario| robustness::run(aircraft, &STILL_AIR, scenario))
        .collect();
    let qualitative: Vec<_> = essais
        .qualitative
        .iter()
        .map(|scenario| qualitative::run(aircraft, &STILL_AIR, scenario))
        .collect();
    let time_step = cruise
        .as_ref()
        .map(|c| time_step_change(aircraft, &STILL_AIR, &c.trim, c.commands, 1.0 / 240.0));

    let report = Report {
        id,
        name: &data.file.name,
        summary: &built.summary,
        fuel_fraction: essais.conditions.fuel_fraction,
        accepted,
        outcomes: &outcomes,
        robustness: &robustness,
        qualitative: &qualitative,
        cruise,
        time_step_change: time_step,
    };
    let dir = out.join(id);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("rapport.md");
    std::fs::write(&path, markdown(&report)).map_err(|e| e.to_string())?;

    for o in &outcomes {
        let (target, unit) = o.unit.display(o.target);
        let computed = o
            .computed
            .map_or("—".to_owned(), |c| format!("{:.1}", o.unit.display(c).0));
        println!(
            "{:<40} cible {target:>7.1} {unit:<4} calculé {computed:>7} {}",
            o.label,
            if o.passed() {
                "ok"
            } else if o.blocking {
                "ÉCHEC"
            } else {
                "échec (non bloquant)"
            }
        );
    }
    for o in &robustness {
        println!("{:<40} {}", o.name, if o.passed { "ok" } else { "ÉCHEC" });
    }
    for o in &qualitative {
        println!("{:<40} {}", o.name, o.observed());
    }
    println!("rapport : {}", path.display());
    Ok(validated(&outcomes)
        && robustness.iter().all(|o| o.passed)
        && qualitative.iter().all(|o| o.finite()))
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Constants => {
            use escadrille_fdm::constants::*;
            println!("g0 = {G0} m/s², T0 = {T0} K, p0 = {P0} Pa, rho0 = {RHO0} kg/m³");
            Ok(true)
        }
        Command::Trim {
            aircraft,
            speed_kmh,
            altitude_m,
        } => build(&cli.assets, &aircraft, 1.0).and_then(|b| {
            let trim = level(&b.aircraft, &STILL_AIR, speed_kmh / 3.6, altitude_m)?;
            println!("{:#?}", trim.flight);
            println!("{:#?}", trim.commands(&b.aircraft));
            Ok(trim.feasible(&b.aircraft))
        }),
        Command::Modes {
            aircraft,
            speed_kmh,
            altitude_m,
        } => build(&cli.assets, &aircraft, 1.0).and_then(|b| {
            let trim = level(&b.aircraft, &STILL_AIR, speed_kmh / 3.6, altitude_m)?;
            let linear = linearise(&b.aircraft, &STILL_AIR, &trim);
            print_modes("Modes longitudinaux", &longitudinal(&linear));
            print_modes("Modes latéraux", &lateral(&linear));
            if let Some(st) = static_stability(&b.aircraft, &linear) {
                println!(
                    "Marge statique {:.1} % ; N_v = {:+.1} N·m/(m/s) ; L_v = {:+.1} N·m/(m/s)",
                    100.0 * st.static_margin,
                    st.n_v,
                    st.l_v
                );
            }
            Ok(true)
        }),
        Command::Essai {
            aircraft,
            confidence,
            out,
        } => essai(&cli.assets, &aircraft, &confidence, &out),
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_essai_command() {
        let cli = Cli::try_parse_from(["fdm_lab", "essai", "spad7"]).unwrap();
        assert!(matches!(cli.command, Command::Essai { aircraft, .. } if aircraft == "spad7"));
    }

    #[test]
    fn runs_the_campaign_of_the_test_biplane() {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/data/tests/fixtures/assets");
        let out = std::env::temp_dir().join(format!("fdm_lab_test_{}", std::process::id()));
        let ok = essai(&assets, "test_biplane", "C", &out).unwrap();
        let report = std::fs::read_to_string(out.join("test_biplane/rapport.md")).unwrap();
        assert!(
            report.contains("# Essais en vol — Test biplane"),
            "{report}"
        );
        assert!(report.contains("Convergence en pas de temps"));
        assert!(report.contains("## Essais qualitatifs"), "{report}");
        assert!(ok, "{report}");
        let _ = std::fs::remove_dir_all(&out);
    }
}
