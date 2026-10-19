//! Capture mode: render a reference scene, save it as PNG with a JSON report, quit.
//!
//! `just capture <scene>` → `target/captures/<scene>.png` and `.json`.
//! The PNG is what Claude looks at to validate any visual change (skill `capture`).

use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk};

use crate::states::AppState;
use crate::world::scenes::KNOWN_SCENES;

/// Extra frames allowed after the requested one before giving up.
const CAPTURE_TIMEOUT_FRAMES: u32 = 600;

/// What to capture and where; inserted by `main` when `--capture` is given.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct CaptureRequest {
    pub scene: String,
    pub frames: u32,
    pub size: (u32, u32),
    pub png: PathBuf,
    pub json: PathBuf,
}

impl CaptureRequest {
    pub fn new(
        scene: String,
        frames: u32,
        size: (u32, u32),
        out: Option<PathBuf>,
    ) -> Result<Self, String> {
        if !KNOWN_SCENES.contains(&scene.as_str()) {
            return Err(format!(
                "unknown scene `{scene}` (known: {})",
                KNOWN_SCENES.join(", ")
            ));
        }
        let png = out.unwrap_or_else(|| Path::new("target/captures").join(format!("{scene}.png")));
        if png.extension().and_then(|e| e.to_str()) != Some("png") {
            return Err(format!("output must be a .png file: {}", png.display()));
        }
        let json = png.with_extension("json");
        Ok(Self {
            scene,
            frames: frames.max(1),
            size,
            png,
            json,
        })
    }
}

/// Parses `WIDTHxHEIGHT`, for the `--size` option.
pub fn parse_size(text: &str) -> Result<(u32, u32), String> {
    let (w, h) = text
        .split_once('x')
        .ok_or_else(|| format!("expected WIDTHxHEIGHT, got `{text}`"))?;
    let parse = |v: &str| {
        v.parse::<u32>()
            .ok()
            .filter(|&n| (16..=8192).contains(&n))
            .ok_or_else(|| format!("invalid dimension `{v}` (16 to 8192)"))
    };
    Ok((parse(w)?, parse(h)?))
}

/// Frame statistics gathered while the image settles.
#[derive(Resource, Debug, Default)]
struct CaptureStats {
    frames: u32,
    total_secs: f64,
    requested: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CaptureStats>()
            .add_systems(Update, drive_capture.run_if(in_state(AppState::Capture)));
    }
}

fn drive_capture(
    mut commands: Commands,
    request: Res<CaptureRequest>,
    mut stats: ResMut<CaptureStats>,
    time: Res<Time>,
    mut exit: MessageWriter<AppExit>,
) {
    stats.frames += 1;
    stats.total_secs += f64::from(time.delta_secs());

    if !stats.requested && stats.frames >= request.frames {
        if let Some(dir) = request.png.parent() {
            if let Err(err) = std::fs::create_dir_all(dir) {
                error!("cannot create {}: {err}", dir.display());
                exit.write(AppExit::error());
                return;
            }
        }
        stats.requested = true;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(request.png.clone()))
            .observe(finish_capture);
    } else if stats.frames > request.frames + CAPTURE_TIMEOUT_FRAMES {
        error!("capture timed out: no image after {} frames", stats.frames);
        exit.write(AppExit::error());
    }
}

fn finish_capture(
    captured: On<ScreenshotCaptured>,
    request: Res<CaptureRequest>,
    stats: Res<CaptureStats>,
    adapter: Option<Res<RenderAdapterInfo>>,
    mut exit: MessageWriter<AppExit>,
) {
    let report = CaptureReport {
        scene: &request.scene,
        width: captured.image.width(),
        height: captured.image.height(),
        frames: stats.frames,
        mean_frame_ms: 1000.0 * stats.total_secs / f64::from(stats.frames.max(1)),
        adapter: adapter.as_ref().map_or("unknown", |a| a.name.as_str()),
    };
    match std::fs::write(&request.json, report.to_json()) {
        Ok(()) => {
            info!("capture saved: {}", request.png.display());
            exit.write(AppExit::Success);
        }
        Err(err) => {
            error!("cannot write {}: {err}", request.json.display());
            exit.write(AppExit::error());
        }
    }
}

/// Written next to the PNG; read by Claude and used to track performance regressions.
struct CaptureReport<'a> {
    scene: &'a str,
    width: u32,
    height: u32,
    frames: u32,
    mean_frame_ms: f64,
    adapter: &'a str,
}

impl CaptureReport<'_> {
    fn to_json(&self) -> String {
        format!(
            "{{\n  \"scene\": \"{}\",\n  \"width\": {},\n  \"height\": {},\n  \"frames\": {},\n  \"mean_frame_ms\": {:.3},\n  \"adapter\": \"{}\",\n  \"game_version\": \"{}\"\n}}\n",
            json_escape(self.scene),
            self.width,
            self.height,
            self.frames,
            self.mean_frame_ms,
            json_escape(self.adapter),
            env!("CARGO_PKG_VERSION"),
        )
    }
}

fn json_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_paths_follow_scene_name() {
        let r = CaptureRequest::new("m0-sol".into(), 10, (640, 360), None).unwrap();
        assert_eq!(r.png, Path::new("target/captures/m0-sol.png"));
        assert_eq!(r.json, Path::new("target/captures/m0-sol.json"));
    }

    #[test]
    fn rejects_unknown_scene_and_non_png_output() {
        assert!(CaptureRequest::new("nope".into(), 10, (640, 360), None).is_err());
        let out = Some(PathBuf::from("shot.jpg"));
        assert!(CaptureRequest::new("m0-sol".into(), 10, (640, 360), out).is_err());
    }

    #[test]
    fn parses_sizes() {
        assert_eq!(parse_size("2560x1440"), Ok((2560, 1440)));
        assert!(parse_size("2560").is_err());
        assert!(parse_size("0x10").is_err());
    }

    #[test]
    fn report_is_valid_json_text() {
        let report = CaptureReport {
            scene: "m0-sol",
            width: 640,
            height: 360,
            frames: 120,
            mean_frame_ms: 16.6667,
            adapter: "AMD \"RADV\" NAVI32",
        };
        let json = report.to_json();
        assert!(json.contains("\"mean_frame_ms\": 16.667"));
        assert!(json.contains("AMD \\\"RADV\\\" NAVI32"));
    }
}
