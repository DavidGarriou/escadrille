//! Weather for Panache blanc: the cloud density field and the noise textures it is built
//! from.
//!
//! Like `escadrille_fdm`, this crate never depends on Bevy or any other engine. The
//! density it computes is the reference of the cloud shader (`assets/shaders/clouds/`),
//! which must give the same values on the GPU (parity test of the game crate), and it
//! serves the game itself: a pilot hides in a cloud, the AI looks through it.
//!
//! The cloud frame is the game's world frame (Bevy axes): x east, y up, z south, in
//! metres; the conversion from the flight model's NED frame is the game's
//! (`crates/game/src/flight/bridge.rs`). Densities, noises and textures are `f32`, as on
//! the GPU; times and world positions are `f64` until they are wrapped into the period of
//! a texture.

pub mod clouds;
pub mod grid;
pub mod noise;
pub mod scattering;
pub mod sun;
