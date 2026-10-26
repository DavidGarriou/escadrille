//! The aircraft's model data as JSON, for the Blender scripts of `art/` (ADR 0007): what
//! `aircraft.ron`, `maquette.ron` and the model contract say about the shape, so that the
//! scripts never read the data files themselves nor work out the contract again.
//!
//! Body axes from the datum (x forward, y right, z down), metres and radians. Written by
//! hand: a few dozen numbers do not call for a JSON dependency.

use std::fmt::Write;

use escadrille_data::aircraft::{AircraftData, Loading, SurfaceKind};
use escadrille_data::mockup::MockupFile;
use escadrille_data::model::contract;
use glam::DVec3;

/// Format version, checked by the scripts.
pub const VERSION: u32 = 1;

fn number(x: f64) -> String {
    if x == 0.0 {
        "0".to_owned()
    } else {
        format!("{x:.6}")
    }
}

fn vector(v: DVec3) -> String {
    format!("[{}, {}, {}]", number(v.x), number(v.y), number(v.z))
}

fn array(v: [f64; 3]) -> String {
    vector(DVec3::from_array(v))
}

fn string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The model data of an aircraft, as a JSON document.
pub fn model_json(data: &AircraftData, mockup: Option<&MockupFile>) -> Result<String, String> {
    let file = &data.file;
    let contract = contract(data).map_err(|e| e.to_string())?;
    let cg = data
        .build(&Loading { fuel_fraction: 1.0 })
        .map_err(|e| e.to_string())?
        .summary
        .cg_from_datum;
    let mut j = String::new();
    let _ = writeln!(j, "{{");
    let _ = writeln!(j, "  \"version\": {VERSION},");
    let _ = writeln!(j, "  \"id\": {},", string(&file.id));
    let _ = writeln!(j, "  \"name\": {},", string(&file.name));
    match &file.dimensions {
        Some(d) => {
            let _ = writeln!(
                j,
                "  \"dimensions\": {{\"span\": {}, \"length\": {}, \"height\": {}}},",
                number(d.span.si()),
                number(d.length.si()),
                number(d.height.si())
            );
        }
        None => {
            let _ = writeln!(j, "  \"dimensions\": null,");
        }
    }
    let _ = writeln!(j, "  \"cg_from_datum\": {},", vector(cg));
    let p = contract.propeller;
    let _ = writeln!(
        j,
        "  \"propeller\": {{\"hub\": {}, \"axis\": {}, \"radius\": {}}},",
        vector(p.hub),
        vector(p.axis),
        number(p.radius)
    );
    let required: Vec<String> = contract.required.iter().map(|r| string(r)).collect();
    let _ = writeln!(j, "  \"required\": [{}],", required.join(", "));

    let _ = writeln!(j, "  \"surfaces\": [");
    for (i, s) in file.surfaces.iter().enumerate() {
        let panels: Vec<String> = s
            .panels
            .iter()
            .map(|p| {
                format!(
                    "{{\"root\": {}, \"tip\": {}, \"chord_root\": {}, \"chord_tip\": {}, \"incidence\": {}, \"twist\": {}}}",
                    array(p.root_m),
                    array(p.tip_m),
                    number(p.chord_root.si()),
                    number(p.chord_tip.si()),
                    number(p.incidence.si()),
                    number(p.twist.map_or(0.0, |t| t.si()))
                )
            })
            .collect();
        let kind = match s.kind {
            SurfaceKind::Horizontal => "horizontal",
            SurfaceKind::Vertical => "vertical",
        };
        let _ = writeln!(
            j,
            "    {{\"name\": {}, \"kind\": \"{kind}\", \"mirrored\": {}, \"panels\": [{}]}}{}",
            string(&s.name),
            s.mirrored,
            panels.join(", "),
            if i + 1 < file.surfaces.len() { "," } else { "" }
        );
    }
    let _ = writeln!(j, "  ],");

    let _ = writeln!(j, "  \"hinges\": [");
    for (i, h) in contract.hinges.iter().enumerate() {
        let regions: Vec<String> = h
            .regions
            .iter()
            .map(|r| {
                format!(
                    "{{\"surface\": {}, \"panel\": {}, \"side\": {}, \"span\": [{}, {}], \"chord_ratio\": {}}}",
                    string(&file.surfaces[r.surface].name),
                    r.panel,
                    number(r.side),
                    number(r.span.0),
                    number(r.span.1),
                    number(r.chord_ratio)
                )
            })
            .collect();
        let _ = writeln!(
            j,
            "    {{\"node\": {}, \"line\": [{}, {}], \"pivot\": {}, \"axis\": {}, \"regions\": [{}]}}{}",
            string(&h.node),
            vector(h.line.0),
            vector(h.line.1),
            vector(h.pivot),
            vector(h.axis),
            regions.join(", "),
            if i + 1 < contract.hinges.len() {
                ","
            } else {
                ""
            }
        );
    }
    let _ = writeln!(j, "  ],");

    match mockup {
        None => {
            let _ = writeln!(j, "  \"mockup\": null");
        }
        Some(m) => {
            let sections: Vec<String> = m
                .fuselage
                .iter()
                .map(|s| {
                    format!(
                        "{{\"x\": {}, \"width\": {}, \"top\": {}, \"bottom\": {}}}",
                        number(s.x_m),
                        number(s.width.si()),
                        number(s.top_m),
                        number(s.bottom_m)
                    )
                })
                .collect();
            let struts: Vec<String> = m
                .struts
                .iter()
                .map(|s| format!("[{}, {}]", number(s.x_m), number(s.y_m)))
                .collect();
            let guns: Vec<String> = m.gun_muzzles_m.iter().map(|g| array(*g)).collect();
            let _ = writeln!(j, "  \"mockup\": {{");
            let _ = writeln!(j, "    \"fuselage\": [{}],", sections.join(", "));
            let _ = writeln!(
                j,
                "    \"wheels\": {{\"centre\": {}, \"diameter\": {}, \"width\": {}}},",
                array(m.wheels.centre_m),
                number(m.wheels.diameter.si()),
                number(m.wheels.width.si())
            );
            let _ = writeln!(j, "    \"skid\": {},", array(m.skid_m));
            let _ = writeln!(j, "    \"struts\": [{}],", struts.join(", "));
            let _ = writeln!(j, "    \"pilot_eye\": {},", array(m.pilot_eye_m));
            let _ = writeln!(j, "    \"gun_muzzles\": [{}]", guns.join(", "));
            let _ = writeln!(j, "  }}");
        }
    }
    let _ = writeln!(j, "}}");
    Ok(j)
}
