//! Panache blanc: the game binary.
//!
//! One plugin per domain (see CLAUDE.md, "Architecture"). M0 only has the
//! development tools (gamepad lab, capture mode) and a reference scene.

mod dev;
mod flight;
mod states;
mod world;

use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_egui::EguiPlugin;
use clap::Parser;

use dev::capture::{CaptureRequest, parse_size};
use states::AppState;

#[derive(Debug, Parser)]
#[command(version, about = "Panache blanc — WWI air combat")]
struct Cli {
    /// Open the gamepad diagnostic lab.
    #[arg(long, conflicts_with = "capture")]
    labo_manette: bool,

    /// Render a reference scene to a PNG file, then quit.
    #[arg(long, value_name = "SCENE")]
    capture: Option<String>,

    /// Frames rendered before the capture, to let the image settle.
    #[arg(long, default_value_t = 120, requires = "capture")]
    frames: u32,

    /// Capture size, as WIDTHxHEIGHT.
    #[arg(long, default_value = "1920x1080", value_parser = parse_size, requires = "capture")]
    size: (u32, u32),

    /// Output PNG path (default: target/captures/<SCENE>.png).
    #[arg(long, requires = "capture")]
    out: Option<PathBuf>,
}

fn main() -> AppExit {
    let cli = Cli::parse();

    let capture = match cli.capture {
        Some(scene) => match CaptureRequest::new(scene, cli.frames, cli.size, cli.out) {
            Ok(request) => Some(request),
            Err(message) => {
                eprintln!("error: {message}");
                return AppExit::from_code(2);
            }
        },
        None => None,
    };

    let start_state = match (&capture, cli.labo_manette) {
        (Some(_), _) => AppState::Capture,
        (None, true) => AppState::LaboManette,
        (None, false) => AppState::Vol,
    };

    let resolution = capture
        .as_ref()
        .map_or(WindowResolution::new(1600, 900), |c| {
            WindowResolution::new(c.size.0, c.size.1)
        });

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Panache blanc".into(),
            resolution,
            resizable: capture.is_none(),
            // Captures measure frame cost: vsync would cap them at the monitor rate.
            present_mode: if capture.is_some() {
                PresentMode::AutoNoVsync
            } else {
                PresentMode::AutoVsync
            },
            ..default()
        }),
        ..default()
    }))
    .add_plugins(EguiPlugin::default())
    .insert_state(start_state)
    .add_plugins((world::WorldPlugin, dev::DevPlugin));

    if let Some(request) = capture {
        app.insert_resource(request);
    }

    app.run()
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
    fn capture_options_require_a_scene() {
        assert!(Cli::try_parse_from(["escadrille", "--frames", "10"]).is_err());
        let cli = Cli::try_parse_from(["escadrille", "--capture", "m0-sol", "--size", "640x360"])
            .unwrap();
        assert_eq!(cli.size, (640, 360));
    }
}
