//! `fdm_lab`: flight tests without rendering, on top of `escadrille_fdm`.
//!
//! M0 only defines the command line; the commands are implemented in M1.

use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about = "Headless flight-test laboratory for Panache blanc")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Find the trimmed state of an aircraft at a given speed and altitude.
    Trim { aircraft: String },
    /// Run the flight-test campaign (essais.ron) of an aircraft.
    Essai { aircraft: String },
    /// Compute the linearised eigenmodes (phugoid, short period, dutch roll...).
    Modes { aircraft: String },
    /// Print the ISA sea-level constants used by the FDM.
    Constants,
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Constants => {
            use escadrille_fdm::constants::*;
            println!("g0 = {G0} m/s², T0 = {T0} K, p0 = {P0} Pa, rho0 = {RHO0} kg/m³");
            ExitCode::SUCCESS
        }
        Command::Trim { .. } | Command::Essai { .. } | Command::Modes { .. } => {
            eprintln!("not implemented yet: arrives with the flight model (milestone M1)");
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
        let cli = Cli::try_parse_from(["fdm_lab", "essai", "spad_vii"]).unwrap();
        assert!(matches!(cli.command, Command::Essai { aircraft } if aircraft == "spad_vii"));
    }
}
