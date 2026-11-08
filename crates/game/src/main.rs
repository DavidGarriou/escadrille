//! Panache blanc: the game binary.
//!
//! One plugin per domain (see CLAUDE.md, "Architecture"): the test flight of M2 (flight,
//! input, realism, camera, ui, world), and the development tools (gamepad lab, capture
//! mode and its reference scenes).

mod camera;
mod dev;
mod flight;
mod input;
mod realism;
mod render;
mod states;
mod ui;
mod world;

use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_egui::EguiPlugin;
use clap::Parser;

use dev::capture::{CaptureRequest, parse_size};
use states::AppState;
use world::scenes::FlightScene;

/// The game's `assets/` directory, the workspace's: under `cargo run`, Bevy would look in
/// `crates/game/assets`, and next to the executable otherwise. Where a release finds its
/// assets is for packaging to decide.
pub const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets");

#[derive(Debug, Parser)]
#[command(version, about = "Panache blanc — WWI air combat")]
struct Cli {
    /// Open the gamepad diagnostic lab.
    #[arg(long, conflicts_with = "capture")]
    labo_manette: bool,

    /// Fly a reference scene of `assets/scenes` with the gamepad: its start, sun, clouds
    /// and camera (`traversee_nuage`, for instance).
    #[arg(long, value_name = "SCENE", conflicts_with_all = ["capture", "labo_manette"])]
    scene: Option<String>,

    /// Render a reference scene to a PNG file, then quit.
    #[arg(long, value_name = "SCENE")]
    capture: Option<String>,

    /// Frames rendered before the capture, to let the image settle (default: the scene's,
    /// or 120).
    #[arg(long, requires = "capture")]
    frames: Option<u32>,

    /// Capture size, as WIDTHxHEIGHT: 1440p, the definition of the performance budget.
    #[arg(long, default_value = "2560x1440", value_parser = parse_size, requires = "capture")]
    size: (u32, u32),

    /// Rendering quality: bas, moyen, haut (the reference) or ultra.
    #[arg(long, value_enum, default_value_t = render::quality::Quality::High)]
    quality: render::quality::Quality,

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
    let flight_scene = match (&capture, cli.scene) {
        (Some(request), _) => FlightScene::of_capture(request),
        (None, Some(name)) => match FlightScene::to_fly(name) {
            Ok(scene) => Some(scene),
            Err(message) => {
                eprintln!("error: {message}");
                return AppExit::from_code(2);
            }
        },
        (None, None) => None,
    };

    let start_state = match (&capture, cli.labo_manette) {
        (Some(_), _) => AppState::Capture,
        (None, true) => AppState::LaboManette,
        (None, false) => AppState::Vol,
    };

    // Captures render into an image of their own size (`dev::capture`), not into the window.
    let resolution = WindowResolution::new(1600, 900);

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
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
            })
            .set(AssetPlugin {
                file_path: ASSETS_DIR.into(),
                ..default()
            }),
    )
    .add_plugins(EguiPlugin::default())
    .insert_state(start_state)
    .add_plugins((
        world::WorldPlugin,
        flight::FlightPlugin,
        input::InputPlugin,
        realism::RealismPlugin,
        camera::CameraPlugin,
        ui::UiPlugin,
        render::RenderingPlugin,
        dev::DevPlugin,
    ));

    app.insert_resource(cli.quality);
    if let Some(scene) = flight_scene {
        app.insert_resource(scene);
    }
    if let Some(request) = capture {
        // The GPU time of each pass, for the capture's report.
        app.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
            .insert_resource(request);
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
    fn a_scene_is_flown_or_captured_not_both() {
        let cli = Cli::try_parse_from(["escadrille", "--scene", "traversee_nuage"]).unwrap();
        assert_eq!(cli.scene.as_deref(), Some("traversee_nuage"));
        assert!(Cli::try_parse_from(["escadrille", "--scene", "a", "--capture", "b"]).is_err());
        assert!(FlightScene::to_fly("traversee_nuage".to_owned()).is_ok());
        // A pose built in code is not flown; an unknown name lists the scenes.
        assert!(FlightScene::to_fly("avion-studio".to_owned()).is_err());
        let error = FlightScene::to_fly("nope".to_owned()).unwrap_err();
        assert!(error.contains("traversee_nuage"), "{error}");
    }

    #[test]
    fn capture_options_require_a_scene() {
        assert!(Cli::try_parse_from(["escadrille", "--frames", "10"]).is_err());
        let cli = Cli::try_parse_from(["escadrille", "--capture", "m0-sol", "--size", "640x360"])
            .unwrap();
        assert_eq!(cli.size, (640, 360));
    }
}
