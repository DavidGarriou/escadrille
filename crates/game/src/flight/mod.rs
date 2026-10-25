//! Flight: the bridge between the flight model (`escadrille_fdm`) and the ECS.
//!
//! M2 brings the fixed step, the interpolation and the floating origin. For now the module
//! holds the frame conversions of ADR 0006, which everything else will go through.

// The `maquette-spad7` scene places a model with it; the rest serves the flight plugin
// from M2 on.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the flight plugin from M2 on (ADR 0006)")
)]
pub mod bridge;
