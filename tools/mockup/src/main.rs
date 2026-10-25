//! `mockup <aircraft>`: writes the provisional 3D model of an aircraft,
//! `assets/aircraft/<id>/model.glb`, from its `aircraft.ron` and `maquette.ron`.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use escadrille_data::aircraft::load_aircraft;
use escadrille_data::mockup::load_mockup;

#[derive(Debug, Parser)]
#[command(about = "Provisional 3D model of an aircraft (boxes and plates), as binary glTF")]
struct Cli {
    /// Aircraft identifier (directory under assets/aircraft).
    aircraft: String,
    /// Assets root.
    #[arg(long, default_value = "assets")]
    assets: PathBuf,
    /// Output file (default: assets/aircraft/<id>/model.glb).
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let run = || -> Result<PathBuf, String> {
        let data = load_aircraft(&cli.assets, &cli.aircraft).map_err(|e| e.to_string())?;
        let mockup = load_mockup(&cli.assets, &cli.aircraft)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("no maquette.ron for {}", cli.aircraft))?;
        let glb = mockup::build_model(&data, &mockup)?;
        let out = cli.out.clone().unwrap_or_else(|| {
            cli.assets
                .join("aircraft")
                .join(&cli.aircraft)
                .join("model.glb")
        });
        std::fs::write(&out, glb).map_err(|e| format!("{}: {e}", out.display()))?;
        Ok(out)
    };
    match run() {
        Ok(out) => {
            println!("{}", out.display());
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("mockup: {message}");
            ExitCode::FAILURE
        }
    }
}
