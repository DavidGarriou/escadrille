//! Top-level application states.
//!
//! M0 only needs the development states; `Briefing`, `Debrief`, menus and
//! pause arrive with the game loop (skill `bevy`, section 3).

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// Free view of the reference scene (no flight model yet).
    #[default]
    Vol,
    /// Gamepad diagnostic screen.
    LaboManette,
    /// Renders a reference scene to disk, then quits.
    Capture,
}
