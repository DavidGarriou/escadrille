//! Aircraft files (`assets/aircraft/<id>/aircraft.ron`, schema in `format-avion.md`):
//! loading, checks, and assembly of the flight model's [`Aircraft`].
//!
//! Positions in the file are body axes (x forward, y right, z down) from a datum the
//! file describes; the builder computes the centre of gravity for a loading and moves
//! every position to it. Surfaces are given as straight panels along their quarter-chord
//! line and split into strips; control-surface signs are derived from the geometry
//! (a positive command must give the pilot's positive moment), so the data cannot get
//! them wrong.

use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::path::Path;

use escadrille_fdm::aero::body::SlenderBody;
use escadrille_fdm::aero::drag::DragItem;
use escadrille_fdm::aero::polar::viterna_cd_max;
use escadrille_fdm::aero::surface::{
    ControlChannel, InducedModel, LiftingSurface, Strip, StripControl, flap_effectiveness,
};
use escadrille_fdm::aero::system::{LiftingSystem, MutualInduction};
use escadrille_fdm::aircraft::{Aircraft, ControlTravel, Downwash, Propulsion};
use escadrille_fdm::propulsion::engine::{AltitudeLaw, EngineError, EnginePoint, InlineEngine};
use escadrille_fdm::propulsion::propeller::{Propeller, PropellerGeometry};
use escadrille_fdm::rigid_body::{Inertia, MassProperties, Rotor};
use glam::DVec3;
use serde::Deserialize;

use crate::airfoil::AirfoilFile;
use crate::propeller::PropellerFile;
use crate::units::{Angle, Area, Length, Mass, Power, RotationSpeed, Volume};
use crate::{DataError, load_checked};

/// Tolerance on a surface's area against the area quoted by the research sheet.
pub const AREA_TOLERANCE: f64 = 0.02;

/// Largest plausible propeller normal-force coefficient c_n (Ribner and DATCOM give
/// C_Nα ≈ 0.1 per radian; c_n is smaller).
pub const MAX_NORMAL_FORCE: f64 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct InertiaSpec {
    pub ixx: f64,
    pub iyy: f64,
    pub izz: f64,
    /// ∫ x z dm (Stevens and Lewis convention, `format-avion.md`).
    pub ixz: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct FuelSpec {
    pub capacity: Volume,
    pub density_kg_per_l: f64,
    pub position_m: [f64; 3],
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PointMassSpec {
    pub name: String,
    pub mass: Mass,
    pub position_m: [f64; 3],
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MassSpec {
    /// Empty aircraft, rotor included (locked).
    pub empty: Mass,
    pub cg_empty_m: [f64; 3],
    /// About `cg_empty_m`, body axes, rotor included.
    pub inertia_empty_kg_m2: InertiaSpec,
    pub pilot: Mass,
    pub pilot_m: [f64; 3],
    pub fuel: FuelSpec,
    /// Guns, ammunition, instruments… not in the empty mass.
    #[serde(default)]
    pub payload: Vec<PointMassSpec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SurfaceKind {
    /// Wings and tailplane: lift up.
    Horizontal,
    /// Fin: positive lift to the right.
    Vertical,
}

/// Straight panel along the quarter-chord line, from root to tip.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PanelSpec {
    pub root_m: [f64; 3],
    pub tip_m: [f64; 3],
    pub chord_root: Length,
    pub chord_tip: Length,
    /// Positive: leading edge towards the lift side.
    pub incidence: Angle,
    /// Tip incidence minus root incidence, linear along the panel.
    #[serde(default)]
    pub twist: Option<Angle>,
    pub strips: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Control {
    Aileron,
    Elevator,
    Rudder,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ControlSpec {
    pub control: Control,
    #[serde(default)]
    pub panel: usize,
    /// Fractions of the panel length, root to tip.
    pub span_from: f64,
    pub span_to: f64,
    /// Control surface chord over local chord.
    pub chord_ratio: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SurfaceSpec {
    pub name: String,
    pub kind: SurfaceKind,
    /// The panels describe the right half; a mirror image makes the left one.
    pub mirrored: bool,
    pub panels: Vec<PanelSpec>,
    /// Path from the assets root.
    pub airfoil: String,
    pub span_efficiency: f64,
    /// Effective aspect ratio of the induced model (end plates, fuselage); by default
    /// span² / area of the surface.
    #[serde(default)]
    pub aspect_ratio: Option<f64>,
    /// Broadside drag coefficient; by default Viterna-Corrigan's for the aspect ratio.
    #[serde(default)]
    pub cd_max: Option<f64>,
    #[serde(default)]
    pub controls: Vec<ControlSpec>,
    /// Area quoted by the research sheet, checked against the strips.
    #[serde(default)]
    pub reference_area: Option<Area>,
}

/// Prandtl's biplane interference between two surfaces (NACA TN 182).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InterferenceSpec {
    pub planes: (String, String),
    /// σ, from gap / span.
    pub sigma: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DownwashSpec {
    pub wings: Vec<String>,
    pub tail: Vec<String>,
    /// dε / dC_L of the wings, rad.
    pub factor: f64,
    pub tail_arm: Length,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DragSpec {
    pub name: String,
    pub cd: f64,
    pub reference_area: Area,
    pub position_m: [f64; 3],
}

/// Fuselage as a slender body (`escadrille_fdm::aero::body`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FuselageSpec {
    /// Centroid of the projected areas.
    pub position_m: [f64; 3],
    pub volume_m3: f64,
    /// k₂ − k₁ (Lamb, from the fineness ratio).
    pub apparent_mass: f64,
    pub plan_area: Area,
    pub side_area: Area,
    /// η C_dc.
    pub crossflow_drag: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PowerPoint {
    pub speed: RotationSpeed,
    pub power: Power,
}

/// Friction and pumping power of the engine.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum FrictionSpec {
    /// A fraction k of the brake power curve: the k of the altitude law σ − k (1 − σ).
    Fraction(f64),
    /// Measured by motoring the engine.
    Curve(Vec<PowerPoint>),
}

/// How the indicated power falls with altitude (`escadrille_fdm::propulsion::engine`).
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum AltitudeLawSpec {
    /// Proportional to the density ratio.
    Density,
    /// Indicated power proportional to the pressure ratio to this power (not the exponent
    /// of a measured brake-power law: see `AltitudeLaw::Pressure`); on a non-standard day,
    /// also to (T_std / T)^m, m being the engine's `temperature_exponent`.
    Pressure(f64),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InlineEngineSpec {
    pub name: String,
    /// Sea-level, full-throttle brake power.
    pub power_curve: Vec<PowerPoint>,
    pub friction: FrictionSpec,
    /// Chosen from the engine's research sheet.
    pub altitude_law: AltitudeLawSpec,
    /// Fraction of the bench indicated power delivered in flight (1 by default).
    #[serde(default = "full_power")]
    pub installed_power: f64,
    /// Exponent m of (T_std / T) on the indicated power at constant pressure, pressure
    /// altitude law only (0 by default: no temperature effect).
    #[serde(default)]
    pub temperature_exponent: f64,
    pub idle_throttle: f64,
    pub stall_speed: RotationSpeed,
}

fn full_power() -> f64 {
    1.0
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum EngineSpec {
    Inline(InlineEngineSpec),
}

/// Direction of rotation seen from behind the propeller, looking forward (from the
/// cockpit of a tractor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PropellerSpec {
    /// Path from the assets root.
    pub table: String,
    pub diameter: Length,
    pub hub_m: [f64; 3],
    pub rotation: Rotation,
    /// P-factor: shift of the thrust line per unit of V⊥ V_blade / V_r², m (E:).
    pub p_factor_m: f64,
    /// Normal-force coefficient c_n (Ribner): in-plane force ½ ρ V_r A c_n V⊥. From
    /// Ribner's C_Nα (per radian, on q∞ and the disc area): c_n = C_Nα J / √(J² + (0.7π)²).
    /// About 0.03 to 0.05 for a two-bladed wooden propeller (blade-element estimate of the
    /// aerodynamicist, 2026-10-22); more than [`MAX_NORMAL_FORCE`] is refused.
    pub normal_force: f64,
    /// Propeller, crankshaft and everything turning with them, kg·m².
    pub rotor_inertia_kg_m2: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct TravelSpec {
    pub aileron: Angle,
    pub elevator: Angle,
    pub rudder: Angle,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Aircraft")]
pub struct AircraftFile {
    pub id: String,
    pub name: String,
    pub research: String,
    /// What the positions are measured from.
    pub datum: String,
    pub mass: MassSpec,
    pub surfaces: Vec<SurfaceSpec>,
    #[serde(default)]
    pub interference: Vec<InterferenceSpec>,
    #[serde(default)]
    pub downwash: Option<DownwashSpec>,
    #[serde(default)]
    pub parasite_drag: Vec<DragSpec>,
    #[serde(default)]
    pub fuselage: Option<FuselageSpec>,
    pub engine: EngineSpec,
    pub propeller: PropellerSpec,
    pub travel: TravelSpec,
}

/// An aircraft file with the aerofoil and propeller files it refers to.
#[derive(Debug, Clone, PartialEq)]
pub struct AircraftData {
    pub file: AircraftFile,
    pub airfoils: BTreeMap<String, AirfoilFile>,
    pub propeller: PropellerFile,
    origin: String,
}

/// Fuel and load for a flight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loading {
    /// 0 (empty tanks) to 1 (full).
    pub fuel_fraction: f64,
}

/// What the builder computed, for reports and checks.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub mass: f64,
    /// Centre of gravity from the datum, m.
    pub cg_from_datum: DVec3,
    /// Per surface: name, area (m²), span (m), aspect ratio used.
    pub surfaces: Vec<(String, f64, f64, f64)>,
}

/// A built aircraft and what the builder computed.
#[derive(Debug, Clone, PartialEq)]
pub struct Built {
    pub aircraft: Aircraft,
    pub summary: Summary,
}

/// Loads `assets/aircraft/<id>/aircraft.ron` and the files it refers to, checking that
/// every number has an origin.
pub fn load_aircraft(assets: &Path, id: &str) -> Result<AircraftData, DataError> {
    let path = assets.join("aircraft").join(id).join("aircraft.ron");
    let file: AircraftFile = load_checked(&path)?;
    let mut airfoils = BTreeMap::new();
    for surface in &file.surfaces {
        if !airfoils.contains_key(&surface.airfoil) {
            let airfoil = load_checked(&assets.join(&surface.airfoil))?;
            airfoils.insert(surface.airfoil.clone(), airfoil);
        }
    }
    let propeller = load_checked(&assets.join(&file.propeller.table))?;
    Ok(AircraftData {
        file,
        airfoils,
        propeller,
        origin: path.display().to_string(),
    })
}

fn vec3(v: [f64; 3]) -> DVec3 {
    DVec3::from_array(v)
}

/// Inertia of a point mass `m` at `d` from the reference point.
fn point_inertia(m: f64, d: DVec3) -> Inertia {
    Inertia {
        ixx: m * (d.y * d.y + d.z * d.z),
        iyy: m * (d.x * d.x + d.z * d.z),
        izz: m * (d.x * d.x + d.y * d.y),
        ixz: m * d.x * d.z,
    }
}

fn add(a: Inertia, b: Inertia) -> Inertia {
    Inertia {
        ixx: a.ixx + b.ixx,
        iyy: a.iyy + b.iyy,
        izz: a.izz + b.izz,
        ixz: a.ixz + b.ixz,
    }
}

/// A strip with the panel it comes from and its position along it (0 root, 1 tip).
struct PlacedStrip {
    strip: Strip,
    panel: usize,
    along: f64,
}

/// Strips of one surface (positions from the datum), its area and its span.
struct Generated {
    strips: Vec<PlacedStrip>,
    area: f64,
    /// Largest distance between panel ends across the flow (tip to tip, gaps for the
    /// fuselage included).
    span: f64,
}

fn surface_strips(spec: &SurfaceSpec) -> Result<Generated, String> {
    let mut strips = Vec::new();
    let mut ends = Vec::new();
    for (index, panel) in spec.panels.iter().enumerate() {
        let (root, tip) = (vec3(panel.root_m), vec3(panel.tip_m));
        let line = tip - root;
        if panel.strips == 0 || line.length() <= 0.0 {
            return Err(format!("{}: empty panel {index}", spec.name));
        }
        let axis = line / line.length();
        let chord_base = (DVec3::X - axis * axis.x).normalize();
        let mut normal_base = chord_base.cross(axis);
        let flip = match spec.kind {
            SurfaceKind::Horizontal => normal_base.z > 0.0,
            SurfaceKind::Vertical => normal_base.y < 0.0,
        };
        if flip {
            normal_base = -normal_base;
        }
        let (c0, c1) = (panel.chord_root.si(), panel.chord_tip.si());
        let twist = panel.twist.map_or(0.0, Angle::si);
        let width = line.length() / f64::from(panel.strips);
        for k in 0..panel.strips {
            let t = (f64::from(k) + 0.5) / f64::from(panel.strips);
            let incidence = panel.incidence.si() + twist * t;
            let (s, c) = incidence.sin_cos();
            let chord = c0 + t * (c1 - c0);
            let strip = Strip {
                position: root + line * t,
                chord_axis: chord_base * c + normal_base * s,
                normal: normal_base * c - chord_base * s,
                area: chord * width,
                chord,
                control: None,
            };
            strips.push(PlacedStrip {
                strip,
                panel: index,
                along: t,
            });
        }
        ends.extend([root, tip]);
    }
    if spec.mirrored {
        let mirror = |v: DVec3| DVec3::new(v.x, -v.y, v.z);
        let left: Vec<_> = strips
            .iter()
            .map(|p| PlacedStrip {
                strip: Strip {
                    position: mirror(p.strip.position),
                    chord_axis: mirror(p.strip.chord_axis),
                    normal: mirror(p.strip.normal),
                    ..p.strip
                },
                ..*p
            })
            .collect();
        strips.extend(left);
        let mirrored_ends: Vec<_> = ends.iter().map(|&e| mirror(e)).collect();
        ends.extend(mirrored_ends);
    }
    let across = |v: DVec3| DVec3::new(0.0, v.y, v.z);
    let span = ends
        .iter()
        .flat_map(|&a| ends.iter().map(move |&b| across(a - b).length()))
        .fold(0.0, f64::max);
    let area = strips.iter().map(|p| p.strip.area).sum();
    Ok(Generated { strips, area, span })
}

impl AircraftData {
    fn invalid(&self, message: impl Into<String>) -> DataError {
        DataError::Invalid {
            origin: self.origin.clone(),
            message: message.into(),
        }
    }

    /// Builds the flight model's aircraft for a loading.
    pub fn build(&self, loading: &Loading) -> Result<Built, DataError> {
        let f = &self.file;
        let m = &f.mass;
        if !(0.0..=1.0).contains(&loading.fuel_fraction) {
            return Err(self.invalid("fuel fraction outside [0, 1]"));
        }

        // Mass, centre of gravity and inertia for this loading.
        let fuel = m.fuel.capacity.litres() * m.fuel.density_kg_per_l * loading.fuel_fraction;
        let mut points = vec![
            (m.pilot.si(), vec3(m.pilot_m)),
            (fuel, vec3(m.fuel.position_m)),
        ];
        points.extend(m.payload.iter().map(|p| (p.mass.si(), vec3(p.position_m))));
        let empty = m.empty.si();
        let cg_empty = vec3(m.cg_empty_m);
        let total: f64 = empty + points.iter().map(|(mass, _)| mass).sum::<f64>();
        let cg = (cg_empty * empty
            + points
                .iter()
                .fold(DVec3::ZERO, |acc, (mass, r)| acc + *r * *mass))
            / total;
        let i = m.inertia_empty_kg_m2;
        let mut inertia = add(
            Inertia {
                ixx: i.ixx,
                iyy: i.iyy,
                izz: i.izz,
                ixz: i.ixz,
            },
            point_inertia(empty, cg_empty - cg),
        );
        for (mass, r) in &points {
            inertia = add(inertia, point_inertia(*mass, *r - cg));
        }

        let p = &f.propeller;
        let right_handed = p.rotation == Rotation::Clockwise;
        let rotor = Rotor {
            axis: if right_handed { DVec3::X } else { -DVec3::X },
            inertia: p.rotor_inertia_kg_m2,
        };
        let mass = MassProperties::new(total, inertia, Some(rotor))
            .map_err(|e| self.invalid(format!("mass properties: {e:?}")))?;

        // Lifting surfaces.
        let mut surfaces = Vec::new();
        let mut summary_surfaces = Vec::new();
        let mut spans = Vec::new();
        for spec in &f.surfaces {
            let Generated {
                mut strips,
                area,
                span,
            } = surface_strips(spec).map_err(|e| self.invalid(e))?;
            if let Some(reference) = spec.reference_area {
                let quoted = reference.si();
                if ((area - quoted) / quoted).abs() > AREA_TOLERANCE {
                    return Err(self.invalid(format!(
                        "{}: strips give {area:.3} m², the research sheet {quoted:.3} m²",
                        spec.name
                    )));
                }
            }
            for p in &mut strips {
                p.strip.position -= cg;
            }
            for control in &spec.controls {
                let (channel, axis) = match control.control {
                    Control::Aileron => (ControlChannel::Aileron, DVec3::X),
                    Control::Elevator => (ControlChannel::Elevator, DVec3::Y),
                    Control::Rudder => (ControlChannel::Rudder, DVec3::Z),
                };
                let tau = flap_effectiveness(control.chord_ratio);
                let mut used = false;
                for PlacedStrip {
                    strip,
                    panel,
                    along,
                } in &mut strips
                {
                    if *panel != control.panel
                        || !(control.span_from..=control.span_to).contains(along)
                    {
                        continue;
                    }
                    // More lift on this strip turns the aircraft about `axis` this way.
                    let effect = strip.position.cross(strip.normal).dot(axis);
                    if effect.abs() < 1e-6 {
                        return Err(self.invalid(format!(
                            "{}: {:?} strip without effect",
                            spec.name, control.control
                        )));
                    }
                    strip.control = Some(StripControl {
                        channel,
                        alpha_per_rad: tau * effect.signum(),
                    });
                    used = true;
                }
                if !used {
                    return Err(self.invalid(format!(
                        "{}: {:?} covers no strip",
                        spec.name, control.control
                    )));
                }
            }
            let aspect_ratio = spec.aspect_ratio.unwrap_or(span * span / area);
            let airfoil = &self.airfoils[&spec.airfoil];
            let polar = airfoil
                .polar(spec.cd_max.unwrap_or(viterna_cd_max(aspect_ratio)))
                .map_err(|e| self.invalid(e))?;
            let induced = InducedModel::new(aspect_ratio, spec.span_efficiency)
                .ok_or_else(|| self.invalid(format!("{}: invalid induced model", spec.name)))?;
            let strips = strips.into_iter().map(|p| p.strip).collect();
            let surface = LiftingSurface::new(spec.name.clone(), strips, polar, Some(induced))
                .map_err(|e| self.invalid(format!("{}: {e:?}", spec.name)))?;
            surfaces.push(surface);
            spans.push((span, area));
            summary_surfaces.push((spec.name.clone(), area, span, aspect_ratio));
        }
        let index_of = |name: &str| {
            f.surfaces
                .iter()
                .position(|s| s.name == name)
                .ok_or_else(|| self.invalid(format!("unknown surface {name}")))
        };
        let mut mutual = Vec::new();
        for interference in &f.interference {
            let (a, b) = (
                index_of(&interference.planes.0)?,
                index_of(&interference.planes.1)?,
            );
            let ((ba, sa), (bb, sb)) = (spans[a], spans[b]);
            let sigma = interference.sigma;
            mutual.push(MutualInduction {
                on: a,
                from: b,
                factor: sigma * sb / (PI * ba * bb),
            });
            mutual.push(MutualInduction {
                on: b,
                from: a,
                factor: sigma * sa / (PI * ba * bb),
            });
        }
        let lifting = LiftingSystem::new(surfaces, mutual)
            .map_err(|e| self.invalid(format!("interference: {e:?}")))?;

        let downwash = match &f.downwash {
            Some(d) => Some(Downwash {
                wings: d
                    .wings
                    .iter()
                    .map(|n| index_of(n))
                    .collect::<Result<_, _>>()?,
                tail: d
                    .tail
                    .iter()
                    .map(|n| index_of(n))
                    .collect::<Result<_, _>>()?,
                factor: d.factor,
                tail_arm: d.tail_arm.si(),
            }),
            None => None,
        };

        let drag = f
            .parasite_drag
            .iter()
            .map(|d| DragItem {
                name: d.name.clone(),
                drag_area: d.cd * d.reference_area.si(),
                position: vec3(d.position_m) - cg,
            })
            .collect();

        if p.normal_force.is_nan() || p.normal_force > MAX_NORMAL_FORCE {
            return Err(self.invalid(format!(
                "propeller: normal_force {} above {MAX_NORMAL_FORCE} (Ribner, DATCOM)",
                p.normal_force
            )));
        }
        let propeller = Propeller::new(
            self.propeller.table(),
            self.propeller.anchors(),
            PropellerGeometry {
                diameter: p.diameter.si(),
                hub: vec3(p.hub_m) - cg,
                thrust_axis: DVec3::X,
                right_handed,
                p_factor: p.p_factor_m,
                normal_force: p.normal_force,
            },
        )
        .map_err(|e| self.invalid(format!("propeller: {e:?}")))?;
        let EngineSpec::Inline(e) = &f.engine;
        let points = |curve: &[PowerPoint], scale: f64| -> Vec<EnginePoint> {
            curve
                .iter()
                .map(|p| EnginePoint {
                    speed: p.speed.si(),
                    power: p.power.si() * scale,
                })
                .collect()
        };
        let friction = match &e.friction {
            FrictionSpec::Fraction(k) => points(&e.power_curve, *k),
            FrictionSpec::Curve(curve) => points(curve, 1.0),
        };
        let altitude_law = match e.altitude_law {
            AltitudeLawSpec::Density => AltitudeLaw::Density,
            AltitudeLawSpec::Pressure(n) => AltitudeLaw::Pressure(n),
        };
        let engine = InlineEngine::new(
            points(&e.power_curve, 1.0),
            friction,
            e.idle_throttle,
            e.stall_speed.si(),
            altitude_law,
        )
        .and_then(|engine| engine.with_installed_power(e.installed_power))
        .and_then(|engine| match e.altitude_law {
            AltitudeLawSpec::Pressure(_) => {
                engine.with_temperature_exponent(e.temperature_exponent)
            }
            AltitudeLawSpec::Density if e.temperature_exponent == 0.0 => Ok(engine),
            AltitudeLawSpec::Density => Err(EngineError::BadTemperatureExponent),
        })
        .map_err(|err| self.invalid(format!("engine {}: {err:?}", e.name)))?;

        let travel = ControlTravel {
            aileron: f.travel.aileron.si(),
            elevator: f.travel.elevator.si(),
            rudder: f.travel.rudder.si(),
        };
        let bodies = f
            .fuselage
            .iter()
            .map(|b| SlenderBody {
                name: "fuselage".into(),
                position: vec3(b.position_m) - cg,
                volume: b.volume_m3,
                apparent_mass: b.apparent_mass,
                plan_area: b.plan_area.si(),
                side_area: b.side_area.si(),
                crossflow_drag: b.crossflow_drag,
            })
            .collect();
        let aircraft = Aircraft::new(
            mass,
            lifting,
            drag,
            bodies,
            Some(Propulsion { propeller, engine }),
            downwash,
            travel,
        )
        .map_err(|e| self.invalid(format!("aircraft: {e:?}")))?;
        Ok(Built {
            aircraft,
            summary: Summary {
                mass: total,
                cg_from_datum: cg,
                surfaces: summary_surfaces,
            },
        })
    }
}

#[cfg(test)]
mod tests;
