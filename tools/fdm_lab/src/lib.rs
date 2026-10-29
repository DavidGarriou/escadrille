//! `fdm_lab`: flight tests without rendering, on top of `escadrille_fdm` (M1 plan,
//! step 6): trim, performance, eigenmodes and reports. The binary (`main.rs`) is the
//! command line around this library.

pub mod assist;
pub mod campaign;
pub mod conditions;
pub mod modes;
pub mod performance;
pub mod plots;
pub mod polar;
pub mod qualitative;
pub mod report;
pub mod robustness;
pub mod slipstream;
pub mod trim;
