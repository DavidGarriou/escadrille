//! `fdm_lab`: flight tests without rendering, on top of `escadrille_fdm` (M1 plan,
//! step 6): trim, performance, eigenmodes and reports. The binary (`main.rs`) is the
//! command line around this library.

pub mod campaign;
pub mod modes;
pub mod performance;
pub mod plots;
pub mod qualitative;
pub mod report;
pub mod robustness;
pub mod trim;
