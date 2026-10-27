//! `mockup <aircraft>`: writes the provisional 3D model of an aircraft,
//! `assets/aircraft/<id>/model.glb`, from its `aircraft.ron` and `maquette.ron`.
//! `mockup <aircraft> --json <file>`: writes the model data for the Blender scripts of
//! `art/` instead (ADR 0007).

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use escadrille_data::aircraft::load_aircraft;
use escadrille_data::livery::load_liveries;
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
    #[arg(long, conflicts_with = "json")]
    out: Option<PathBuf>,
    /// Write the model data as JSON to this file, for the Blender scripts, and no model.
    #[arg(long)]
    json: Option<PathBuf>,
    /// Overwrite a model.glb that this tool did not write (a modelled one).
    #[arg(long)]
    force: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let run = || -> Result<PathBuf, String> {
        let data = load_aircraft(&cli.assets, &cli.aircraft).map_err(|e| e.to_string())?;
        let mockup = load_mockup(&cli.assets, &cli.aircraft).map_err(|e| e.to_string())?;
        if let Some(path) = &cli.json {
            let liveries = load_liveries(&cli.assets, &cli.aircraft).map_err(|e| e.to_string())?;
            let json = mockup::export::model_json(&data, mockup.as_ref(), liveries.as_ref())?;
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            }
            std::fs::write(path, json).map_err(|e| format!("{}: {e}", path.display()))?;
            return Ok(path.clone());
        }
        let mockup = mockup.ok_or_else(|| format!("no maquette.ron for {}", cli.aircraft))?;
        let glb = mockup::build_model(&data, &mockup)?;
        let out = cli.out.clone().unwrap_or_else(|| {
            cli.assets
                .join("aircraft")
                .join(&cli.aircraft)
                .join("model.glb")
        });
        if !cli.force
            && let Ok(existing) = std::fs::read(&out)
            && !mockup::written_here(&existing)
        {
            return Err(format!(
                "{} is not a provisional model (it was not written by this tool): \
                 --out elsewhere, or --force to replace it",
                out.display()
            ));
        }
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
