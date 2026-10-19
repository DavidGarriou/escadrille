//! Flight dynamics model (FDM) for Panache blanc.
//!
//! This crate must never depend on Bevy or any other engine: it is used by the
//! game, the AI, the tools (`fdm_lab`) and the tests. All quantities are SI and
//! `f64`; body axes are x forward, y right, z down, and the local world frame is NED.
//!
//! M0 only provides the physical constants; the model itself arrives in M1.

pub mod constants;
