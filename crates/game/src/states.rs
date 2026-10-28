//! Top-level application states.
//!
//! The test flight and the development states; `Briefing`, `Debrief` and menus arrive
//! with the game loop (skill `bevy`, section 3). The pause stops the virtual clock.

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// The test flight (plan `m2-vol-essai.md`).
    #[default]
    Vol,
    /// Gamepad diagnostic screen.
    LaboManette,
    /// Renders a reference scene to disk, then quits.
    Capture,
}
