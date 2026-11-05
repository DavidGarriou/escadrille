//! Capture mode: render a reference scene, save it as PNG with a JSON report, quit.
//!
//! `just capture <scene>` → `target/captures/<scene>.png` and `.json`.
//! The PNG is what Claude looks at to validate any visual change (skill `capture`).
//!
//! A capture is the same image whatever the GPU's speed: the game's clock waits for the
//! scene to load, then each frame advances it by [`FRAME_STEP_SECS`].

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use bevy::camera::{CameraUpdateSystems, RenderTarget};
use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureFormat, TextureUsages};
use bevy::render::renderer::RenderAdapterInfo;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use escadrille_data::scene::load_scene;

use crate::ASSETS_DIR;
use crate::render::quality::Quality;
use crate::states::AppState;
use crate::world::scenes::{CODE_SCENES, known_scenes};

/// Extra frames allowed after the requested one before giving up.
const CAPTURE_TIMEOUT_FRAMES: u32 = 600;

/// Frames rendered before the capture, unless the scene or the command line say otherwise.
pub const DEFAULT_FRAMES: u32 = 120;

/// Game time of each frame of a capture, s: a game at 60 frames a second, two steps of
/// the simulation.
pub const FRAME_STEP_SECS: f64 = 1.0 / 60.0;

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
        frames: Option<u32>,
        size: (u32, u32),
        out: Option<PathBuf>,
    ) -> Result<Self, String> {
        let known = known_scenes();
        if !known.contains(&scene) {
            return Err(format!(
                "unknown scene `{scene}` (known: {})",
                known.join(", ")
            ));
        }
        let frames = match frames {
            Some(f) => f,
            None if CODE_SCENES.contains(&scene.as_str()) => DEFAULT_FRAMES,
            None => load_scene(Path::new(ASSETS_DIR), &scene)
                .map_err(|e| e.to_string())?
                .frames
                .unwrap_or(DEFAULT_FRAMES),
        };
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

/// What the scene is still loading (a 3D model, for instance): the frames before the
/// capture are counted once it is back to zero.
#[derive(Resource, Debug, Default)]
pub struct CaptureHold(pub u32);

/// Frame statistics gathered while the image settles.
#[derive(Resource, Debug, Default)]
struct CaptureStats {
    frames: u32,
    /// Wall-clock duration of each frame since the scene loaded, s.
    durations: Vec<f64>,
    /// When the last frame was counted: the game's own clocks are stepped by hand.
    last: Option<Instant>,
    requested: bool,
    /// Frames spent waiting for the scene to load.
    loading: u32,
}

impl CaptureStats {
    /// Mean duration of the second half of the frames, ms: the first ones compile
    /// pipelines and fill the temporal accumulations.
    fn settled_frame_ms(&self) -> f64 {
        let settled = &self.durations[self.durations.len() / 2..];
        1000.0 * settled.iter().sum::<f64>() / settled.len().max(1) as f64
    }
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CaptureStats>()
            .init_resource::<CaptureHold>()
            .add_systems(
                PreStartup,
                prepare_capture.run_if(resource_exists::<CaptureRequest>),
            )
            .add_systems(
                PostUpdate,
                retarget_cameras
                    .run_if(resource_exists::<CaptureTarget>)
                    .before(CameraUpdateSystems),
            )
            .add_systems(Update, drive_capture.run_if(in_state(AppState::Capture)));
    }
}

/// The image the cameras render into during a capture: exactly the requested size, which
/// a window cannot promise (the compositor fits it to the screen, minus its panels).
#[derive(Resource, Debug)]
struct CaptureTarget(Handle<Image>);

/// The capture's image, and its clock: stepped by hand, stopped until the scene is loaded.
fn prepare_capture(
    mut commands: Commands,
    request: Res<CaptureRequest>,
    mut images: ResMut<Assets<Image>>,
    mut clock: ResMut<Time<Virtual>>,
) {
    let (width, height) = request.size;
    let mut image = Image::new_target_texture(width, height, TextureFormat::Rgba8UnormSrgb, None);
    // The screenshot copies it out.
    image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
    commands.insert_resource(CaptureTarget(images.add(image)));
    commands.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        FRAME_STEP_SECS,
    )));
    clock.pause();
}

/// Every camera of the scene renders into the capture's image instead of the window.
fn retarget_cameras(
    target: Res<CaptureTarget>,
    mut cameras: Query<&mut RenderTarget, Added<Camera>>,
) {
    for mut render_target in &mut cameras {
        *render_target = RenderTarget::Image(target.0.clone().into());
    }
}

fn drive_capture(
    mut commands: Commands,
    request: Res<CaptureRequest>,
    target: Res<CaptureTarget>,
    mut stats: ResMut<CaptureStats>,
    hold: Res<CaptureHold>,
    mut clock: ResMut<Time<Virtual>>,
    mut exit: MessageWriter<AppExit>,
) {
    if hold.0 > 0 {
        stats.loading += 1;
        if stats.loading > CAPTURE_TIMEOUT_FRAMES {
            error!(
                "capture timed out: the scene is still loading after {} frames",
                stats.loading
            );
            exit.write(AppExit::error());
        }
        return;
    }
    // Loaded: the scene's clock starts on the next frame.
    clock.unpause();
    stats.frames += 1;
    let now = Instant::now();
    if let Some(last) = stats.last.replace(now) {
        stats.durations.push((now - last).as_secs_f64());
    }

    if !stats.requested && stats.frames >= request.frames {
        if let Some(dir) = request.png.parent()
            && let Err(err) = std::fs::create_dir_all(dir)
        {
            error!("cannot create {}: {err}", dir.display());
            exit.write(AppExit::error());
            return;
        }
        stats.requested = true;
        commands
            .spawn(Screenshot::image(target.0.clone()))
            .observe(save_to_disk(request.png.clone()))
            .observe(finish_capture);
    } else if stats.frames > request.frames + CAPTURE_TIMEOUT_FRAMES {
        error!("capture timed out: no image after {} frames", stats.frames);
        exit.write(AppExit::error());
    }
}

/// The GPU time of each render pass, ms, averaged over the diagnostics' history: the
/// passes `RenderDiagnosticsPlugin` times, by their path under `render/`.
fn gpu_passes(store: &DiagnosticsStore) -> Vec<(String, f64)> {
    let mut passes: Vec<(String, f64)> = store
        .iter()
        .filter_map(|d| {
            let path = d.path().as_str();
            let pass = path.strip_prefix("render/")?.strip_suffix("/elapsed_gpu")?;
            Some((pass.to_owned(), d.average()?))
        })
        .collect();
    passes.sort_by(|a, b| a.0.cmp(&b.0));
    passes
}

fn finish_capture(
    captured: On<ScreenshotCaptured>,
    request: Res<CaptureRequest>,
    stats: Res<CaptureStats>,
    quality: Res<Quality>,
    diagnostics: Option<Res<DiagnosticsStore>>,
    adapter: Option<Res<RenderAdapterInfo>>,
    mut exit: MessageWriter<AppExit>,
) {
    let passes = diagnostics.map_or_else(Vec::new, |d| gpu_passes(&d));
    let report = CaptureReport {
        scene: &request.scene,
        width: captured.image.width(),
        height: captured.image.height(),
        frames: stats.frames,
        mean_frame_ms: stats.settled_frame_ms(),
        quality: quality.name(),
        adapter: adapter.as_ref().map_or("unknown", |a| a.name.as_str()),
        passes: &passes,
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
    /// Mean of the second half of the frames.
    mean_frame_ms: f64,
    quality: &'a str,
    adapter: &'a str,
    /// GPU time of each pass, ms; the top-level ones (no `/` in their path) add up to the
    /// frame's.
    passes: &'a [(String, f64)],
}

impl CaptureReport<'_> {
    fn to_json(&self) -> String {
        let top: f64 = self
            .passes
            .iter()
            .filter(|(p, _)| !p.contains('/'))
            .map(|(_, ms)| ms)
            .sum();
        let passes = self
            .passes
            .iter()
            .map(|(p, ms)| format!("\n    \"{}\": {ms:.3}", json_escape(p)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\n  \"scene\": \"{}\",\n  \"width\": {},\n  \"height\": {},\n  \"frames\": {},\n  \"mean_frame_ms\": {:.3},\n  \"quality\": \"{}\",\n  \"adapter\": \"{}\",\n  \"game_version\": \"{}\",\n  \"gpu_frame_ms\": {top:.3},\n  \"gpu_passes_ms\": {{{passes}\n  }}\n}}\n",
            json_escape(self.scene),
            self.width,
            self.height,
            self.frames,
            self.mean_frame_ms,
            json_escape(self.quality),
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
        let r = CaptureRequest::new("m0-sol".into(), Some(10), (640, 360), None).unwrap();
        assert_eq!(r.png, Path::new("target/captures/m0-sol.png"));
        assert_eq!(r.json, Path::new("target/captures/m0-sol.json"));
    }

    #[test]
    fn every_known_scene_can_be_captured() {
        let known = known_scenes();
        for scene in &known {
            assert!(CaptureRequest::new(scene.clone(), None, (640, 360), None).is_ok());
        }
        assert!(known.contains(&"maquette-spad7".to_owned()));
        assert!(known.contains(&"terrain-repos".to_owned()));
    }

    #[test]
    fn a_scene_file_gives_its_own_frames_unless_told_otherwise() {
        let file = CaptureRequest::new("terrain-roulage".into(), None, (640, 360), None);
        assert_eq!(file.unwrap().frames, 420);
        let told = CaptureRequest::new("terrain-roulage".into(), Some(30), (640, 360), None);
        assert_eq!(told.unwrap().frames, 30);
        let code = CaptureRequest::new("m0-sol".into(), None, (640, 360), None);
        assert_eq!(code.unwrap().frames, DEFAULT_FRAMES);
    }

    #[test]
    fn rejects_unknown_scene_and_non_png_output() {
        assert!(CaptureRequest::new("nope".into(), Some(10), (640, 360), None).is_err());
        let out = Some(PathBuf::from("shot.jpg"));
        assert!(CaptureRequest::new("m0-sol".into(), Some(10), (640, 360), out).is_err());
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
            quality: "haut",
            adapter: "AMD \"RADV\" NAVI32",
            passes: &[
                ("main_opaque_pass_3d".to_owned(), 1.25),
                ("main_opaque_pass_3d/inner".to_owned(), 0.5),
                ("shadows".to_owned(), 0.75),
            ],
        };
        let json = report.to_json();
        assert!(json.contains("\"mean_frame_ms\": 16.667"));
        assert!(json.contains("AMD \\\"RADV\\\" NAVI32"));
        assert!(json.contains("\"gpu_frame_ms\": 2.000"), "{json}");
        assert!(json.contains("\"shadows\": 0.750"), "{json}");
        assert!(json.trim_end().ends_with('}'), "{json}");
        let opens = json.matches('{').count();
        assert_eq!(opens, json.matches('}').count(), "{json}");
    }
}
