//! All the lifting surfaces of an aircraft, with the mutual induction between the
//! planes of a biplane or triplane (skill `simulation-vol`, `modele-aero.md`).
//!
//! Prandtl's biplane theory (*Induced drag of multiplanes*, NACA TN 182, 1924): the
//! induced drag of two planes is D_i = (L₁²/b₁² + 2σ L₁L₂/(b₁b₂) + L₂²/b₂²) / (π q), with
//! σ the interference factor (a function of gap / span, from Prandtl's table). The
//! induced angle of plane i is then α_i = Σ_j k_ij C_L,j, with k_ii = 1 / (π e AR_i) (each
//! surface's own [`InducedModel`](super::surface::InducedModel)) and, for j ≠ i,
//! k_ij = σ S_j / (π b_i b_j): the [`MutualInduction`] factors, computed from the
//! aircraft data (C:).
//!
//! Each strip keeps its own induction k_ii C_L (strip) and receives, from every other
//! plane, k_ij × that plane's lift coefficient C̄_L,j. The coupled equations are solved by
//! fixed-point iteration on the C̄_L,j: the coupling k_ij a / (1 + k_ii a) is small (about
//! 0.1 for a 1917 biplane), so it converges in a few iterations; the iteration count is
//! capped, which keeps the result deterministic. In separated flow the mutual induction
//! fades out with the strip's own (see [`super::surface`]).
//!
//! Not modelled here (to be carried by the data, C: or E:): the different lift of the two
//! planes in two-dimensional interference (Millikan, NACA Report 362), the effect of
//! stagger on the load sharing (Munk, NACA Report 151), and the wing downwash on the
//! tail, which needs a lag and is handled by the aircraft model.

use glam::DVec3;

use super::surface::{ControlDeflections, LiftingSurface, Strip, StripFlow, SurfaceLoads};
use crate::rigid_body::Wrench;

/// Induction of one plane on another: induced angle on `on` = `factor` × C_L of `from`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MutualInduction {
    pub on: usize,
    pub from: usize,
    /// k_ij = σ S_j / (π b_i b_j), rad per unit lift coefficient.
    pub factor: f64,
}

/// Why a lifting system was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemError {
    /// Index out of range, a surface inducing on itself, or the same pair twice.
    BadPair,
    BadFactor,
    /// Both surfaces of a pair need an induced model (finite-span planes).
    NoInducedModel,
}

/// Maximum fixed-point iterations on the planes' lift coefficients.
const MUTUAL_MAX_ITERATIONS: usize = 40;

/// Change of lift coefficient below which the iteration stops.
const MUTUAL_TOLERANCE: f64 = 1e-13;

#[derive(Debug, Clone, PartialEq)]
pub struct LiftingSystem {
    surfaces: Vec<LiftingSurface>,
    mutual: Vec<MutualInduction>,
}

/// Loads of the whole system.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemLoads {
    pub wrench: Wrench,
    /// Force and moment of each surface about the centre of gravity, body axes.
    pub surface_wrenches: Vec<Wrench>,
    /// The part of each surface's wrench that the swirl's incidence adds: what it takes
    /// from the slipstream's rotation ([`super::surface::SurfaceLoads::swirl`]).
    pub swirl_wrenches: Vec<Wrench>,
    /// Lift coefficient of each surface (strip dynamic pressure × area weighted).
    pub lift_coefficients: Vec<f64>,
    /// Induced angle each surface receives from the others, rad.
    pub mutual_angles: Vec<f64>,
}

impl LiftingSystem {
    pub fn new(
        surfaces: Vec<LiftingSurface>,
        mutual: Vec<MutualInduction>,
    ) -> Result<Self, SystemError> {
        for (n, m) in mutual.iter().enumerate() {
            let duplicate = mutual[..n]
                .iter()
                .any(|other| (other.on, other.from) == (m.on, m.from));
            if m.on >= surfaces.len() || m.from >= surfaces.len() || m.on == m.from || duplicate {
                return Err(SystemError::BadPair);
            }
            if !(m.factor.is_finite() && m.factor >= 0.0) {
                return Err(SystemError::BadFactor);
            }
            if surfaces[m.on].induced().is_none() || surfaces[m.from].induced().is_none() {
                return Err(SystemError::NoInducedModel);
            }
        }
        Ok(Self { surfaces, mutual })
    }

    pub fn surfaces(&self) -> &[LiftingSurface] {
        &self.surfaces
    }

    pub fn mutual(&self) -> &[MutualInduction] {
        &self.mutual
    }

    /// Force and moment of all the surfaces about the centre of gravity, body axes.
    ///
    /// Same conventions as [`LiftingSurface::loads`]; `local_flow(i, strip)` is the extra
    /// air velocity at a strip of surface `i` (downwash on the tail only, for instance).
    pub fn loads<F: Into<StripFlow>>(
        &self,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        controls: &ControlDeflections,
        local_flow: impl Fn(usize, &Strip) -> F,
    ) -> SystemLoads {
        self.loads_of(
            |_| true,
            air_velocity,
            angular_velocity,
            density,
            controls,
            local_flow,
        )
    }

    /// Same as [`Self::loads`] for the surfaces `i` with `include(i)` only, the others
    /// having no loads, no lift and no induced angle: the wings before the tail, which flies
    /// in the slipstream they leave. A plane left out induces nothing on the others, so
    /// each pair of [`MutualInduction`] should be included or left out together.
    pub fn loads_of<F: Into<StripFlow>>(
        &self,
        include: impl Fn(usize) -> bool,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        controls: &ControlDeflections,
        local_flow: impl Fn(usize, &Strip) -> F,
    ) -> SystemLoads {
        let n = self.surfaces.len();
        let surface_loads = |i: usize, mutual: f64| {
            self.surfaces[i].loads_with_mutual(
                air_velocity,
                angular_velocity,
                density,
                controls,
                |strip| local_flow(i, strip),
                mutual,
            )
        };
        let mutual_angles = |cl: &[f64]| {
            let mut angles = vec![0.0; n];
            for m in &self.mutual {
                angles[m.on] += m.factor * cl[m.from];
            }
            angles
        };

        let mut cl = vec![0.0; n];
        let coupled: Vec<usize> = (0..n)
            .filter(|&i| include(i) && self.mutual.iter().any(|m| m.on == i || m.from == i))
            .collect();
        for _ in 0..MUTUAL_MAX_ITERATIONS {
            let angles = mutual_angles(&cl);
            let mut change: f64 = 0.0;
            for &i in &coupled {
                let new = surface_loads(i, angles[i]).lift_coefficient;
                change = change.max((new - cl[i]).abs());
                cl[i] = new;
            }
            if change < MUTUAL_TOLERANCE {
                break;
            }
        }

        let mut angles = mutual_angles(&cl);
        let mut wrench = Wrench::default();
        let mut surface_wrenches = Vec::with_capacity(n);
        let mut swirl_wrenches = Vec::with_capacity(n);
        for (i, angle) in angles.iter_mut().enumerate() {
            let loads = if include(i) {
                surface_loads(i, *angle)
            } else {
                *angle = 0.0;
                SurfaceLoads::default()
            };
            wrench += loads.wrench;
            surface_wrenches.push(loads.wrench);
            swirl_wrenches.push(loads.swirl);
            cl[i] = loads.lift_coefficient;
        }
        SystemLoads {
            wrench,
            surface_wrenches,
            swirl_wrenches,
            lift_coefficients: cl,
            mutual_angles: angles,
        }
    }
}

impl SystemLoads {
    /// The loads of two disjoint sets of surfaces of the same system, as
    /// [`LiftingSystem::loads_of`] gives them, put together.
    pub fn with(self, other: Self) -> Self {
        let add = |a: Vec<f64>, b: Vec<f64>| a.iter().zip(&b).map(|(x, y)| x + y).collect();
        let add_wrenches =
            |a: Vec<Wrench>, b: Vec<Wrench>| a.iter().zip(&b).map(|(x, y)| *x + *y).collect();
        Self {
            wrench: self.wrench + other.wrench,
            surface_wrenches: add_wrenches(self.surface_wrenches, other.surface_wrenches),
            swirl_wrenches: add_wrenches(self.swirl_wrenches, other.swirl_wrenches),
            lift_coefficients: add(self.lift_coefficients, other.lift_coefficients),
            mutual_angles: add(self.mutual_angles, other.mutual_angles),
        }
    }
}

#[cfg(test)]
mod tests;
