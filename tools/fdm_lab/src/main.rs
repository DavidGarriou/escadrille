//! `fdm_lab`: flight tests without rendering, on top of `escadrille_fdm` (M1 plan,
//! step 6). `just essai <id>` runs the campaign of `assets/aircraft/<id>/essais.ron` and
//! writes `target/essais/<id>/rapport.md`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use escadrille_data::aircraft::{Built, Loading, load_aircraft};
use escadrille_data::essais::{Confidence, Quantitative, load_essais};
use escadrille_data::masses::load_masses;
use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::environment::{Environment, StandardEnvironment};
use fdm_lab::campaign::{self, validated};
use fdm_lab::conditions::{NonStandardDay, time_to_pressure_altitude};
use fdm_lab::modes::{Mode, lateral, linearise, longitudinal, static_stability};
use fdm_lab::performance::{best_climb, ceiling, envelope, glide, max_level_speed, stall_speed_at};
use fdm_lab::plots::{self, Panel, Series};
use fdm_lab::polar::{AirframePolar, Reference, airframe_polar};
use fdm_lab::report::{Cruise, Report, markdown, time_step_change};
use fdm_lab::slipstream::{TN1146, swirl_balance, tail_off_roll, tn1146_model, tn1146_speed};
use fdm_lab::trim::{Flight, LEVEL_AT_SPEED, Trim, solve_robust};
use fdm_lab::{qualitative, robustness};

#[derive(Debug, Parser)]
#[command(version, about = "Headless flight-test laboratory for Panache blanc")]
struct Cli {
    /// Root of the data files.
    #[arg(long, global = true, default_value = "assets")]
    assets: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Find the trimmed level flight of an aircraft at a given speed and altitude.
    Trim {
        aircraft: String,
        #[arg(long, default_value_t = 150.0)]
        speed_kmh: f64,
        #[arg(long, default_value_t = 2_000.0)]
        altitude_m: f64,
    },
    /// Run the flight-test campaign (essais.ron) of an aircraft and write the report.
    Essai {
        aircraft: String,
        /// Lowest confidence that blocks the validation (M1: C, player's decision).
        #[arg(long, default_value = "C")]
        confidence: String,
        #[arg(long, default_value = "target/essais")]
        out: PathBuf,
    },
    /// Compute the linearised eigenmodes (phugoid, short period, dutch roll...).
    Modes {
        aircraft: String,
        #[arg(long, default_value_t = 150.0)]
        speed_kmh: f64,
        #[arg(long, default_value_t = 2_000.0)]
        altitude_m: f64,
    },
    /// Drag polar of the airframe from glides at idle (airframe without its propeller's
    /// forces and slipstream, the zero-thrust glides of NACA Report 304) and its
    /// Oswald factor.
    Polaire {
        aircraft: String,
        #[arg(long, default_value_t = 1_000.0)]
        altitude_m: f64,
    },
    /// Times to climb to the heights of essais.ron, and the ceiling, on days warmer or
    /// colder than the standard atmosphere (pressure altitudes, as a period altimeter),
    /// from sea level and from an airfield, its altimeter zeroed on the ground or not.
    Montee {
        aircraft: String,
        /// Temperature offsets to the standard atmosphere, K.
        #[arg(long, value_delimiter = ',', default_values_t = [0.0, 10.0, 20.0])]
        delta_t_k: Vec<f64>,
        /// Airfield altitude above sea level, m.
        #[arg(long, default_value_t = 0.0)]
        airfield_m: f64,
    },
    /// Without an aircraft: rolling moment of the slipstream on the powered model of NACA
    /// TN 1146, tail off, against the measurement (run with `--assets
    /// tools/fdm_lab/comparison`). With one: what the swirl does to it in a few flights.
    Souffle { aircraft: Option<String> },
    /// Empty mass, centre of gravity and inertia from the mass build-up (masses.ron), as
    /// the mass block of aircraft.ron, checked against it.
    Inertia { aircraft: String },
    /// Print the ISA sea-level constants used by the FDM.
    Constants,
}

const STILL_AIR: StandardEnvironment = StandardEnvironment {
    wind_ned: glam::DVec3::ZERO,
    ground_altitude: 0.0,
};

fn build(assets: &Path, id: &str, fuel_fraction: f64) -> Result<Built, String> {
    load_aircraft(assets, id)
        .and_then(|data| data.build(&Loading { fuel_fraction }))
        .map_err(|e| e.to_string())
}

/// Reference of the polar: the two planes of a biplane (interference) or the largest
/// horizontal surface.
fn reference(data: &escadrille_data::aircraft::AircraftData, built: &Built) -> Reference {
    let area_span = |name: &str| {
        built
            .summary
            .surfaces
            .iter()
            .find(|(n, ..)| n == name)
            .map(|&(_, area, span, _)| (area, span))
    };
    if let Some(i) = data.file.interference.first() {
        let (a, b) = (area_span(&i.planes.0), area_span(&i.planes.1));
        if let (Some((s1, b1)), Some((s2, b2))) = (a, b) {
            return Reference {
                area: s1 + s2,
                span: b1.max(b2),
                kappa: 0.5 * (1.0 + i.sigma),
            };
        }
    }
    let (_, area, span, _) =
        built
            .summary
            .surfaces
            .iter()
            .cloned()
            .fold((String::new(), 0.0, 0.0, 0.0), |best, s| {
                if s.1 > best.1 { s } else { best }
            });
    Reference {
        area,
        span,
        kappa: 1.0,
    }
}

fn print_polar(polar: &AirframePolar) {
    println!(
        "Polaire de la cellule à {:.0} m (vols planés au ralenti, hélice exclue) :",
        polar.altitude
    );
    for p in &polar.points {
        println!(
            "  {:6.1} km/h   C_L {:6.3}   C_D {:7.4}",
            p.speed * 3.6,
            p.cl,
            p.cd
        );
    }
    println!(
        "C_D = {:.4} + {:.4} C_L² ({} vols de C_L 0,2 à 0,8) ; allongement équivalent {:.2} ; coefficient d'Oswald {:.3}",
        polar.cd0,
        polar.k,
        polar.fitted,
        polar.reference.equivalent_aspect_ratio(),
        polar.oswald()
    );
}

fn level(
    aircraft: &Aircraft,
    env: &dyn Environment,
    speed: f64,
    altitude: f64,
) -> Result<Trim, String> {
    solve_robust(
        aircraft,
        env,
        Flight::guess(altitude, speed, 0.5),
        LEVEL_AT_SPEED,
    )
    .map_err(|e| format!("pas de vol en palier à {:.1} km/h : {e:?}", speed * 3.6))
}

fn print_modes(title: &str, modes: &[Mode]) {
    println!("{title}");
    for m in modes {
        println!(
            "  {:<26} λ = {:+.4} {:+.4} i   ζ = {:+.3}   période = {}   état dominant {}",
            m.name,
            m.eigenvalue.re,
            m.eigenvalue.im,
            m.damping,
            m.period.map_or("—".to_owned(), |p| format!("{p:.2} s")),
            m.dominant
        );
    }
}

/// Altitude of the airframe polar in the report, m.
const POLAR_ALTITUDE: f64 = 1_000.0;

/// Envelope step of the plots, m.
const PLOT_STEP: f64 = 250.0;

/// The report's plots, written to `dir`: (title, file name) of each.
fn figures(
    aircraft: &Aircraft,
    polar: Option<&AirframePolar>,
    essais: &escadrille_data::essais::EssaisFile,
    qualitative: &[fdm_lab::qualitative::QualitativeOutcome],
    dir: &Path,
) -> Result<Vec<(String, String)>, String> {
    let points = envelope(aircraft, &STILL_AIR, PLOT_STEP);
    let mut speeds = Vec::new();
    let mut rpms = Vec::new();
    let mut climbs = Vec::new();
    for q in &essais.quantitative {
        match q {
            Quantitative::MaxLevelSpeed {
                altitude,
                target,
                rpm,
                ..
            } => {
                speeds.push((target.si() * 3.6, altitude.si()));
                if let Some(r) = rpm {
                    rpms.push((r.target.si() * 30.0 / std::f64::consts::PI, altitude.si()));
                }
            }
            Quantitative::ClimbTime {
                from, to, target, ..
            } if from.si() == 0.0 => climbs.push((target.si() / 60.0, to.si())),
            _ => {}
        }
    }
    // Time to climb: trapezoids on 1 / rate along the envelope.
    let mut time = 0.0;
    let mut times = Vec::new();
    for (i, p) in points.iter().enumerate() {
        if i > 0 {
            let q = &points[i - 1];
            time += 0.5 * (1.0 / q.best_climb.rate + 1.0 / p.best_climb.rate) * PLOT_STEP;
        }
        times.push((time / 60.0, p.altitude));
    }
    let model = |f: fn(&fdm_lab::performance::EnvelopePoint) -> f64| {
        points
            .iter()
            .map(|p| (f(p), p.altitude))
            .collect::<Vec<_>>()
    };
    let one = |label: &str, model: Vec<(f64, f64)>, targets: Vec<(f64, f64)>| Panel {
        y_label: label.to_owned(),
        series: if targets.is_empty() {
            vec![Series::line("modèle", model)]
        } else {
            vec![
                Series::line("modèle", model),
                Series::dots("cible historique", targets),
            ]
        },
    };
    let mut written = Vec::new();
    let charts = [
        (
            "vitesse.png",
            "Vitesse maximale en palier, pleins gaz",
            "vitesse vraie (km/h)",
            one(
                "altitude (m)",
                model(|p| p.max_speed.flight.speed * 3.6),
                speeds,
            ),
        ),
        (
            "regime.png",
            "Régime à la vitesse maximale",
            "régime (tr/min)",
            one(
                "altitude (m)",
                model(|p| p.max_speed.flight.rotor_speed * 30.0 / std::f64::consts::PI),
                rpms,
            ),
        ),
        (
            "taux-de-montee.png",
            "Meilleur taux de montée, pleins gaz",
            "taux de montée (m/s)",
            one("altitude (m)", model(|p| p.best_climb.rate), Vec::new()),
        ),
        (
            "montee.png",
            "Temps de montée depuis le niveau de la mer",
            "temps (min)",
            one("altitude (m)", times, climbs),
        ),
    ];
    for (file, title, x_label, panel) in charts {
        plots::draw(&dir.join(file), title, x_label, &[panel])?;
        written.push((title.to_owned(), file.to_owned()));
    }
    if let Some(p) = polar {
        let title = "Polaire de la cellule (vols planés au ralenti, hélice exclue)";
        let fitted = (0..=20)
            .map(|i| {
                let cl = 0.05 * f64::from(i) * 1.1;
                (p.cd0 + p.k * cl * cl, cl)
            })
            .collect();
        let panel = Panel {
            y_label: "C_L".to_owned(),
            series: vec![
                Series::dots(
                    "vols planés du modèle",
                    p.points.iter().map(|q| (q.cd, q.cl)).collect(),
                ),
                Series::line("ajustement parabolique", fitted),
            ],
        };
        plots::draw(&dir.join("polaire.png"), title, "C_D", &[panel])?;
        written.push((title.to_owned(), "polaire.png".to_owned()));
    }
    for (i, o) in qualitative.iter().enumerate() {
        if let Some(Ok(observed)) = &o.flown
            && !observed.trace.is_empty()
        {
            let file = format!("qualitatif-{}.png", i + 1);
            let title = format!("Essai qualitatif : {}", o.name);
            plots::manoeuvre(&dir.join(&file), &title, &observed.trace)?;
            written.push((title, file));
        }
    }
    Ok(written)
}

fn essai(assets: &Path, id: &str, confidence: &str, out: &Path) -> Result<bool, String> {
    let accepted = match confidence {
        "A" => Confidence::A,
        "B" => Confidence::B,
        "C" => Confidence::C,
        "D" => Confidence::D,
        other => return Err(format!("niveau de confiance inconnu : {other}")),
    };
    let essais = load_essais(assets, id).map_err(|e| e.to_string())?;
    let built = build(assets, id, essais.conditions.fuel_fraction)?;
    let data = load_aircraft(assets, id).map_err(|e| e.to_string())?;
    let aircraft = &built.aircraft;
    let outcomes = campaign::run(aircraft, &STILL_AIR, &essais, accepted);

    // Cruise for the modes: 80 % of the maximum speed at the first speed-test altitude.
    let altitude = essais
        .quantitative
        .iter()
        .find_map(|q| match q {
            Quantitative::MaxLevelSpeed { altitude, .. } => Some(altitude.si()),
            _ => None,
        })
        .unwrap_or(2_000.0);
    let cruise = max_level_speed(aircraft, &STILL_AIR, altitude)
        .ok()
        .and_then(|vmax| level(aircraft, &STILL_AIR, 0.8 * vmax.flight.speed, altitude).ok())
        .map(|trim| {
            let linear = linearise(aircraft, &STILL_AIR, &trim);
            Cruise {
                commands: trim.commands(aircraft),
                longitudinal: longitudinal(&linear),
                lateral: lateral(&linear),
                stability: static_stability(aircraft, &linear),
                trim,
            }
        });
    let robustness: Vec<_> = essais
        .robustness
        .iter()
        .map(|scenario| robustness::run(aircraft, &STILL_AIR, scenario))
        .collect();
    let qualitative: Vec<_> = essais
        .qualitative
        .iter()
        .map(|scenario| qualitative::run(aircraft, &STILL_AIR, scenario))
        .collect();
    let time_step = cruise
        .as_ref()
        .map(|c| time_step_change(aircraft, &STILL_AIR, &c.trim, c.commands, 1.0 / 240.0));

    let dir = out.join(id);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let polar = airframe_polar(
        aircraft,
        &STILL_AIR,
        POLAR_ALTITUDE,
        reference(&data, &built),
    )
    .ok();
    let figures = figures(aircraft, polar.as_ref(), &essais, &qualitative, &dir)?;
    let report = Report {
        id,
        name: &data.file.name,
        summary: &built.summary,
        fuel_fraction: essais.conditions.fuel_fraction,
        accepted,
        outcomes: &outcomes,
        robustness: &robustness,
        qualitative: &qualitative,
        polar,
        figures: &figures,
        cruise,
        time_step_change: time_step,
    };
    let path = dir.join("rapport.md");
    std::fs::write(&path, markdown(&report)).map_err(|e| e.to_string())?;

    for o in &outcomes {
        let (target, unit) = o.unit.display(o.target);
        let computed = o
            .computed
            .map_or("—".to_owned(), |c| format!("{:.1}", o.unit.display(c).0));
        println!(
            "{:<40} cible {target:>7.1} {unit:<4} calculé {computed:>7} {}",
            o.label,
            if o.passed() {
                "ok"
            } else if o.blocking {
                "ÉCHEC"
            } else {
                "échec (non bloquant)"
            }
        );
    }
    for o in &robustness {
        println!("{:<40} {}", o.name, if o.passed { "ok" } else { "ÉCHEC" });
    }
    for o in &qualitative {
        println!("{:<40} {}", o.name, o.observed());
    }
    println!("rapport : {}", path.display());
    Ok(validated(&outcomes)
        && robustness.iter().all(|o| o.passed)
        && qualitative.iter().all(|o| o.finite()))
}

/// `fdm_lab montee`: climbs of essais.ron under other test conditions.
fn montee(assets: &Path, id: &str, deltas: &[f64], airfield: f64) -> Result<bool, String> {
    let essais = load_essais(assets, id).map_err(|e| e.to_string())?;
    let built = build(assets, id, essais.conditions.fuel_fraction)?;
    let aircraft = &built.aircraft;
    let mut climbs = Vec::new();
    let mut ceiling_target = None;
    for q in &essais.quantitative {
        match q {
            Quantitative::ClimbTime {
                from, to, target, ..
            } if from.si() == 0.0 => climbs.push((to.si(), target.si())),
            Quantitative::Ceiling {
                climb_rate, target, ..
            } if climb_rate.si() == 0.0 => ceiling_target = Some(target.si()),
            _ => {}
        }
    }
    let minutes = |t: f64| {
        let seconds = t.round();
        format!(
            "{:.0} min {:02.0} s",
            (seconds / 60.0).floor(),
            seconds % 60.0
        )
    };
    let gap = |t: f64, target: f64| format!("{:+.1} %", 100.0 * (t / target - 1.0));
    println!(
        "Montées de {id} ({:.0} % de carburant), altitudes-pression, meilleure vitesse ascensionnelle, sans le décollage :",
        100.0 * essais.conditions.fuel_fraction
    );
    let mut conventions = vec![("depuis 0 m", 0.0, 0.0)];
    if airfield != 0.0 {
        conventions.push(("terrain, altimètre sur la mer", airfield, 0.0));
        conventions.push(("terrain, altimètre à zéro au sol", airfield, airfield));
    }
    for &delta_t in deltas {
        let day = NonStandardDay::new(delta_t)
            .ok_or_else(|| format!("écart de température hors du climat : {delta_t} K"))?;
        let ceiling = ceiling(aircraft, &day, 0.0).map_err(|e| format!("{e:?}"))?;
        for &(name, start, zero) in &conventions {
            let mut line = format!("  ISA {delta_t:+.0} K, {name} :");
            for &(to, target) in &climbs {
                let t = time_to_pressure_altitude(aircraft, &day, start, zero + to)
                    .map_err(|e| format!("{e:?}"))?;
                line += &format!("  {:.0} m en {} ({})", to, minutes(t), gap(t, target));
            }
            let reading = ceiling.altitude - zero;
            line += &format!("  ; plafond {reading:.0} m");
            if let Some(target) = ceiling_target {
                line += &format!(" ({})", gap(reading, target));
            }
            println!("{line}");
        }
    }
    let targets: Vec<String> = climbs
        .iter()
        .map(|&(to, t)| format!("{to:.0} m en {}", minutes(t)))
        .collect();
    println!("  cibles : {}", targets.join(", "));
    Ok(true)
}

fn inertia(assets: &Path, id: &str) -> Result<bool, String> {
    let masses = load_masses(assets, id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| {
            format!("pas de bilan de masses pour {id} (assets/aircraft/{id}/masses.ron)")
        })?;
    let e = masses.empty();
    let i = e.inertia;
    println!(
        "Bilan de masses de {id} ({} composants) :",
        masses.components.len()
    );
    println!("        empty: Kg({:.1}),", e.mass);
    println!(
        "        cg_empty_m: ({:.3}, {:.3}, {:.3}), // C: fdm_lab inertia {id} (masses.ron)",
        e.cg_m[0], e.cg_m[1], e.cg_m[2]
    );
    println!(
        "        inertia_empty_kg_m2: (ixx: {:.0}, iyy: {:.0}, izz: {:.0}, ixz: {:.0}), // C: fdm_lab inertia {id} (masses.ron)",
        i.ixx, i.iyy, i.izz, i.ixz
    );
    match load_aircraft(assets, id) {
        Ok(data) => match masses.check(&data.file.mass) {
            Ok(()) => {
                println!("aircraft.ron concorde avec le bilan.");
                Ok(true)
            }
            Err(message) => {
                println!("aircraft.ron s'écarte du bilan : {message}");
                Ok(false)
            }
        },
        Err(error) => {
            println!("aircraft.ron non chargé : {error}");
            Ok(false)
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Constants => {
            use escadrille_fdm::constants::*;
            println!("g0 = {G0} m/s², T0 = {T0} K, p0 = {P0} Pa, rho0 = {RHO0} kg/m³");
            Ok(true)
        }
        Command::Trim {
            aircraft,
            speed_kmh,
            altitude_m,
        } => build(&cli.assets, &aircraft, 1.0).and_then(|b| {
            let trim = level(&b.aircraft, &STILL_AIR, speed_kmh / 3.6, altitude_m)?;
            println!("{:#?}", trim.flight);
            println!("{:#?}", trim.commands(&b.aircraft));
            Ok(trim.feasible(&b.aircraft))
        }),
        Command::Modes {
            aircraft,
            speed_kmh,
            altitude_m,
        } => build(&cli.assets, &aircraft, 1.0).and_then(|b| {
            let trim = level(&b.aircraft, &STILL_AIR, speed_kmh / 3.6, altitude_m)?;
            let linear = linearise(&b.aircraft, &STILL_AIR, &trim);
            print_modes("Modes longitudinaux", &longitudinal(&linear));
            print_modes("Modes latéraux", &lateral(&linear));
            if let Some(st) = static_stability(&b.aircraft, &linear) {
                println!(
                    "Marge statique {:.1} % ; N_v = {:+.1} N·m/(m/s) ; L_v = {:+.1} N·m/(m/s)",
                    100.0 * st.static_margin,
                    st.n_v,
                    st.l_v
                );
            }
            Ok(true)
        }),
        Command::Essai {
            aircraft,
            confidence,
            out,
        } => essai(&cli.assets, &aircraft, &confidence, &out),
        Command::Inertia { aircraft } => inertia(&cli.assets, &aircraft),
        Command::Montee {
            aircraft,
            delta_t_k,
            airfield_m,
        } => montee(&cli.assets, &aircraft, &delta_t_k, airfield_m),
        Command::Souffle { aircraft: None } => souffle(&cli.assets),
        Command::Souffle {
            aircraft: Some(aircraft),
        } => build(&cli.assets, &aircraft, 0.74).and_then(|built| swirl_report(&built.aircraft)),
        Command::Polaire {
            aircraft,
            altitude_m,
        } => load_aircraft(&cli.assets, &aircraft)
            .map_err(|e| e.to_string())
            .and_then(|data| {
                let built = data
                    .build(&Loading { fuel_fraction: 1.0 })
                    .map_err(|e| e.to_string())?;
                let polar = airframe_polar(
                    &built.aircraft,
                    &STILL_AIR,
                    altitude_m,
                    reference(&data, &built),
                )
                .map_err(|e| format!("{e:?}"))?;
                print_polar(&polar);
                Ok(true)
            }),
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

/// Prints what the swirl does to an aircraft at its maximum speed (2 000 m), best climb
/// (sea level), in a glide at idle and at the stall at full throttle (1 000 m).
fn swirl_report(aircraft: &Aircraft) -> Result<bool, String> {
    let e = |e: fdm_lab::performance::PerformanceError| format!("{e:?}");
    let flights = [
        (
            "Vmax 2 000 m",
            max_level_speed(aircraft, &STILL_AIR, 2_000.0).map_err(e)?,
        ),
        (
            "meilleure montée 0 m",
            best_climb(aircraft, &STILL_AIR, 0.0).map_err(e)?.trim,
        ),
        (
            "plané 35 m/s",
            glide(aircraft, &STILL_AIR, 1_000.0, 35.0).map_err(e)?,
        ),
        (
            "décrochage plein gaz",
            stall_speed_at(aircraft, &STILL_AIR, 1_000.0, 1.0)
                .map_err(e)?
                .trim,
        ),
    ];
    for (name, trim) in &flights {
        let b = swirl_balance(aircraft, &STILL_AIR, trim).ok_or("no propeller")?;
        let total: f64 = b.surfaces.iter().map(|s| s.1).sum();
        println!(
            "{name} : {:.1} m/s, couple {:.1} N·m ; roulis du tourbillon {:.2} Q ; puissance reprise {:.2} kW sur {:.2} kW d'énergie de rotation ({:.0} %)",
            trim.flight.speed,
            b.torque,
            total / b.torque,
            b.recovered_power / 1e3,
            b.rotational_power / 1e3,
            100.0 * b.recovered_power / b.rotational_power
        );
        for (surface, roll, power) in &b.surfaces {
            println!(
                "    {surface:<12} roulis {:+.2} Q, puissance {:+.2} kW",
                roll / b.torque,
                power / 1e3
            );
        }
        // Angular momentum: what the wings take back is no longer in the slipstream behind
        // them (aerodynamicist review of 2026-10-25).
        if let Some(d) = aircraft.downwash() {
            let names: Vec<&str> = d
                .wings
                .iter()
                .map(|&i| aircraft.lifting().surfaces()[i].name())
                .collect();
            let wings: f64 = b
                .surfaces
                .iter()
                .filter(|s| names.contains(&s.0.as_str()))
                .map(|s| s.1)
                .sum();
            println!(
                "    rotation laissée derrière les ailes : {:.2} par la donnée, {:.2} par le moment cinétique (1 − {:.2} Q repris par les ailes){}",
                d.swirl_left,
                (1.0 - wings / b.torque).clamp(0.0, 1.0),
                wings / b.torque,
                if total > b.torque {
                    " ; **la cellule reprend plus que le couple**"
                } else {
                    ""
                }
            );
        }
    }
    Ok(true)
}

/// Prints the TN 1146 comparison; `Ok(true)` when every blade angle falls in the
/// measured range.
fn souffle(assets: &Path) -> Result<bool, String> {
    let speed = tn1146_speed();
    println!("Maquette du NACA TN 1146, sans empennage, α = 0, V = {speed:.2} m/s");
    println!();
    println!(
        "| Calage | J | Couple (N·m) | Repris par la cellule | Roulis net / couple | Mesure |"
    );
    println!("|---|---|---|---|---|---|");
    let mut inside = true;
    for point in &TN1146 {
        let aircraft = tn1146_model(assets, point)?;
        let roll = tail_off_roll(&aircraft, &STILL_AIR, speed, point.advance_ratio)
            .ok_or("the model has no propeller")?;
        let (lo, hi) = point.measured;
        let net = roll.net_fraction();
        inside &= (lo..=hi).contains(&net);
        println!(
            "| {:.0}° | {:.3} | {:.3} | {:.0} % | {:.3} | {:.3} à {:.3} |",
            point.blade_angle,
            point.advance_ratio,
            roll.torque,
            100.0 * roll.recovered_fraction(),
            net,
            lo,
            hi
        );
    }
    Ok(inside)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_essai_command() {
        let cli = Cli::try_parse_from(["fdm_lab", "essai", "spad7"]).unwrap();
        assert!(matches!(cli.command, Command::Essai { aircraft, .. } if aircraft == "spad7"));
    }

    #[test]
    fn runs_the_campaign_of_the_test_biplane() {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/data/tests/fixtures/assets");
        let out = std::env::temp_dir().join(format!("fdm_lab_test_{}", std::process::id()));
        let ok = essai(&assets, "test_biplane", "C", &out).unwrap();
        let report = std::fs::read_to_string(out.join("test_biplane/rapport.md")).unwrap();
        assert!(
            report.contains("# Essais en vol — Test biplane"),
            "{report}"
        );
        assert!(report.contains("Convergence en pas de temps"));
        assert!(report.contains("## Essais qualitatifs"), "{report}");
        assert!(report.contains("](vitesse.png)"), "{report}");
        assert!(
            report.contains("coefficient d'Oswald de l'avion complet"),
            "{report}"
        );
        for file in [
            "vitesse.png",
            "regime.png",
            "montee.png",
            "qualitatif-1.png",
        ] {
            assert!(out.join("test_biplane").join(file).exists(), "{file}");
        }
        assert!(ok, "{report}");
        let _ = std::fs::remove_dir_all(&out);
    }
}
