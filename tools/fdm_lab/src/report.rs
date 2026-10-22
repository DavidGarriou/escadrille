//! Flight-test report (`target/essais/<id>/rapport.md`), in French like the other
//! documents read by the player, and the time-step convergence check of the M1 plan.

use std::fmt::Write as _;

use escadrille_data::aircraft::Summary;
use escadrille_data::essais::Confidence;
use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;

use crate::campaign::{Outcome, validated};
use crate::modes::{Mode, StaticStability};
use crate::robustness::RobustnessOutcome;
use crate::trim::Trim;

/// Steady flight with its eigenmodes, for the report.
#[derive(Debug, Clone)]
pub struct Cruise {
    pub trim: Trim,
    pub commands: PilotControls,
    pub longitudinal: Vec<Mode>,
    pub lateral: Vec<Mode>,
    pub stability: Option<StaticStability>,
}

#[derive(Debug, Clone)]
pub struct Report<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub summary: &'a Summary,
    pub fuel_fraction: f64,
    pub accepted: Confidence,
    pub outcomes: &'a [Outcome],
    pub robustness: &'a [RobustnessOutcome],
    pub cruise: Option<Cruise>,
    /// Largest relative change when the time step is halved.
    pub time_step_change: Option<f64>,
}

fn mode_rows(text: &mut String, modes: &[Mode]) {
    for m in modes {
        let period = m.period.map_or("—".to_owned(), |p| format!("{p:.2} s"));
        let time = if m.stable() {
            format!("½ en {:.2} s", m.time_to_half_or_double)
        } else {
            format!("**×2 en {:.2} s**", m.time_to_half_or_double)
        };
        let _ = writeln!(
            text,
            "| {} | {:+.4} {:+.4} i | {:.3} | {} | {} | {} |",
            m.name, m.eigenvalue.re, m.eigenvalue.im, m.damping, period, time, m.dominant
        );
    }
}

/// The report, in Markdown.
pub fn markdown(r: &Report) -> String {
    let mut t = String::new();
    let _ = writeln!(t, "# Essais en vol — {} (`{}`)\n", r.name, r.id);
    let _ = writeln!(
        t,
        "Rapport produit par `fdm_lab essai {}` : modèle de vol `escadrille_fdm`, données `assets/aircraft/{}/`. Les essais de confiance {:?} ou meilleure bloquent la validation.\n",
        r.id, r.id, r.accepted
    );
    let _ = writeln!(t, "## Chargement\n");
    let s = r.summary;
    let _ = writeln!(
        t,
        "- Carburant : {:.0} % ; masse : {:.1} kg ; centre de gravité depuis la référence : ({:.3} ; {:.3} ; {:.3}) m",
        100.0 * r.fuel_fraction,
        s.mass,
        s.cg_from_datum.x,
        s.cg_from_datum.y,
        s.cg_from_datum.z
    );
    for (name, area, span, ar) in &s.surfaces {
        let _ = writeln!(
            t,
            "- {name} : surface {area:.2} m², envergure {span:.2} m, allongement {ar:.2}"
        );
    }

    let _ = writeln!(t, "\n## Essais quantitatifs\n");
    let verdict = if validated(r.outcomes) && r.robustness.iter().all(|o| o.passed) {
        "**validé**"
    } else {
        "**non validé**"
    };
    let _ = writeln!(t, "Verdict : {verdict}.\n");
    let _ = writeln!(
        t,
        "| Essai | Cible | Calculé | Écart | Tolérance | Confiance | Source | Résultat |"
    );
    let _ = writeln!(t, "|---|---|---|---|---|---|---|---|");
    for o in r.outcomes {
        let (target, unit) = o.unit.display(o.target);
        let computed = o.computed.map_or("—".to_owned(), |c| {
            format!("{:.1} {unit}", o.unit.display(c).0)
        });
        let error = o
            .error_pct()
            .map_or("—".to_owned(), |e| format!("{e:+.1} %"));
        let result = match (o.passed(), o.blocking) {
            (true, _) => "réussi",
            (false, true) => "**échec**",
            (false, false) => "échec (non bloquant)",
        };
        let _ = writeln!(
            t,
            "| {} | {target:.1} {unit} | {computed} | {error} | ±{:.0} % | {:?} | {} | {result} |",
            o.label, o.tolerance_pct, o.confidence, o.source
        );
    }
    for o in r.outcomes.iter().filter(|o| o.note.is_some()) {
        let _ = writeln!(t, "\n- {} : {}", o.label, o.note.as_deref().unwrap_or(""));
    }

    if let Some(c) = &r.cruise {
        let f = c.trim.flight;
        let _ = writeln!(t, "\n## Vol en palier et modes propres\n");
        let _ = writeln!(
            t,
            "Trim à {:.1} km/h, {:.0} m : incidence {:.2}°, dérapage {:.2}°, gaz {:.2}, régime {:.0} tr/min ; commandes : profondeur {:+.2}, ailerons {:+.2}, direction {:+.2}{}.\n",
            f.speed * 3.6,
            f.altitude,
            f.alpha.to_degrees(),
            f.beta.to_degrees(),
            f.throttle,
            f.rotor_speed * 30.0 / std::f64::consts::PI,
            c.commands.pitch,
            c.commands.roll,
            c.commands.yaw,
            if [c.commands.pitch, c.commands.roll, c.commands.yaw]
                .iter()
                .all(|v| v.abs() <= 1.0)
            {
                ""
            } else {
                " (**au-delà des débattements**)"
            }
        );
        if let Some(st) = c.stability {
            let _ = writeln!(
                t,
                "Stabilité statique : marge statique {:.1} % de la corde moyenne ({:.2} m), régime et déflexion en équilibre ; N_v = {:+.1} N·m/(m/s) ({}) ; L_v = {:+.1} N·m/(m/s) ({}).\n",
                100.0 * st.static_margin,
                st.mean_chord,
                st.n_v,
                if st.n_v > 0.0 {
                    "stable de route"
                } else {
                    "**instable de route**"
                },
                st.l_v,
                if st.l_v < 0.0 {
                    "effet de dièdre"
                } else {
                    "**dièdre négatif**"
                }
            );
        }
        let lanchester =
            std::f64::consts::PI * 2f64.sqrt() * f.speed / escadrille_fdm::constants::G0;
        let _ = writeln!(
            t,
            "Repère : période de phugoïde de Lanchester π √2 V / g = {lanchester:.1} s.\n"
        );
        let _ = writeln!(
            t,
            "| Mode | λ (1/s) | Amortissement ζ | Période | Évolution | État dominant |"
        );
        let _ = writeln!(t, "|---|---|---|---|---|---|");
        mode_rows(&mut t, &c.longitudinal);
        mode_rows(&mut t, &c.lateral);
    }

    if !r.robustness.is_empty() {
        let _ = writeln!(t, "\n## Robustesse\n");
        let _ = writeln!(
            t,
            "Réussi : la simulation reste finie (pas de NaN) et dans des bornes physiques. Ce que fait l'avion est rapporté, pas jugé.\n"
        );
        let _ = writeln!(t, "| Scénario | Résultat | Déroulement |");
        let _ = writeln!(t, "|---|---|---|");
        for o in r.robustness {
            let result = if o.passed { "réussi" } else { "**échec**" };
            let _ = writeln!(t, "| {} | {result} | {} |", o.name, o.details);
        }
    }

    if let Some(change) = r.time_step_change {
        let _ = writeln!(t, "\n## Convergence en pas de temps\n");
        let _ = writeln!(
            t,
            "Réponse à un créneau de profondeur pendant 20 s, à 120 Hz × 2 sous-pas puis au double : écart maximal relatif {:.3} % ({}).",
            100.0 * change,
            if change < 0.01 {
                "moins de 1 %, conforme"
            } else {
                "**plus de 1 %**"
            }
        );
    }
    t
}

/// Largest relative difference between a 20 s flight at `dt` and at `dt / 2`, starting
/// from a trim, with an elevator doublet (±0.2 for one second each): speed, altitude
/// change and pitch angle.
pub fn time_step_change(
    aircraft: &Aircraft,
    env: &dyn Environment,
    trim: &Trim,
    commands: PilotControls,
    dt: f64,
) -> f64 {
    let fly = |step: f64| {
        let mut state: RigidBodyState = trim.flight.state();
        let mut samples = Vec::new();
        // Whole numbers of steps per second, so that both runs switch the elevator at
        // exactly the same instants.
        let per_second = (1.0 / step).round() as u32;
        let steps = 20 * per_second;
        let per_sample = per_second / 10;
        for i in 0..steps {
            let doublet = if (per_second..2 * per_second).contains(&i) {
                0.2
            } else if (2 * per_second..3 * per_second).contains(&i) {
                -0.2
            } else {
                0.0
            };
            let controls = PilotControls {
                pitch: commands.pitch + doublet,
                ..commands
            };
            state = aircraft.step(&state, &controls, env, step);
            if (i + 1) % per_sample == 0 {
                let (_, pitch, _) = state.attitude.to_euler(glam::EulerRot::ZYX);
                samples.push((state.velocity_body.length(), -state.position_ned.z, pitch));
            }
        }
        samples
    };
    let (coarse, fine) = (fly(dt), fly(0.5 * dt));
    let altitude0 = trim.flight.altitude;
    coarse
        .iter()
        .zip(&fine)
        .map(|(a, b)| {
            let speed = (a.0 - b.0).abs() / b.0.abs().max(1.0);
            let climb = (a.1 - b.1).abs() / (b.1 - altitude0).abs().max(10.0);
            let pitch = (a.2 - b.2).abs() / b.2.abs().max(0.05);
            speed.max(climb).max(pitch)
        })
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};
    use crate::trim::{Flight, LEVEL_AT_SPEED, solve_robust};

    #[test]
    fn time_step_check_sees_a_coarse_step() {
        let a = biplane();
        let trim = solve_robust(
            &a,
            &STILL_AIR,
            Flight::guess(1_000.0, 37.0, 0.4),
            LEVEL_AT_SPEED,
        )
        .unwrap();
        let c = trim.commands(&a);
        let fine = time_step_change(&a, &STILL_AIR, &trim, c, 1.0 / 240.0);
        let coarse = time_step_change(&a, &STILL_AIR, &trim, c, 1.0 / 20.0);
        assert!(fine < 0.01, "{fine}");
        assert!(coarse > 10.0 * fine && coarse > 0.0, "{coarse} vs {fine}");
    }
}
