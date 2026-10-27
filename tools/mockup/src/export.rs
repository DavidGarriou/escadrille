//! The aircraft's model data as JSON, for the Blender scripts of `art/` (ADR 0007): what
//! `aircraft.ron`, `maquette.ron`, `livrees.ron` and the model contract say about the
//! shape and its paint, so that the scripts never read the data files themselves nor work
//! out the contract again.
//!
//! Body axes from the datum (x forward, y right, z down), metres and radians. Written by
//! hand: a few dozen numbers do not call for a JSON dependency.

use std::fmt::Write;

use escadrille_data::aircraft::{AircraftData, Loading, SurfaceKind};
use escadrille_data::livery::{LiveryFile, MarkSpec, split_face};
use escadrille_data::mockup::MockupFile;
use escadrille_data::model::contract;
use glam::DVec3;

/// Format version, checked by the scripts.
pub const VERSION: u32 = 2;

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
pub fn model_json(
    data: &AircraftData,
    mockup: Option<&MockupFile>,
    liveries: Option<&LiveryFile>,
) -> Result<String, String> {
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
            let _ = writeln!(j, "  \"mockup\": null,");
        }
        Some(m) => {
            if let Some(o) = m
                .outlines
                .iter()
                .find(|o| !data.file.surfaces.iter().any(|s| s.name == o.surface))
            {
                return Err(format!(
                    "maquette.ron: outline of {}, a surface aircraft.ron does not have",
                    o.surface
                ));
            }
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
            let _ = writeln!(j, "    \"gun_muzzles\": [{}],", guns.join(", "));
            let outlines: Vec<String> = m
                .outlines
                .iter()
                .map(|o| {
                    let rows: Vec<String> = o.rows.iter().map(|r| array(*r)).collect();
                    format!("{}: [{}]", string(&o.surface), rows.join(", "))
                })
                .collect();
            let _ = writeln!(j, "    \"outlines\": {{{}}},", outlines.join(", "));
            let features: Vec<String> = m
                .features
                .iter()
                .map(|f| {
                    let rows: Vec<String> = f
                        .rows
                        .iter()
                        .map(|r| {
                            let values: Vec<String> = r.iter().map(|v| number(*v)).collect();
                            format!("[{}]", values.join(", "))
                        })
                        .collect();
                    format!("{}: [{}]", string(&f.name), rows.join(", "))
                })
                .collect();
            let _ = writeln!(j, "    \"features\": {{{}}}", features.join(", "));
            let _ = writeln!(j, "  }},");
        }
    }
    match liveries {
        None => {
            let _ = writeln!(j, "  \"liveries\": null");
        }
        Some(l) => {
            let _ = writeln!(j, "  \"liveries\": {}", liveries_json(data, &contract, l)?);
        }
    }
    let _ = writeln!(j, "}}");
    Ok(j)
}

/// The colours and liveries, the faces and nodes they paint checked against the aircraft:
/// a face names a surface of `aircraft.ron` (or the fuselage) and a side it has, a band a
/// control node of the contract.
fn liveries_json(
    data: &AircraftData,
    contract: &escadrille_data::model::Contract,
    file: &LiveryFile,
) -> Result<String, String> {
    let names = |colours: &[String]| -> String {
        let list: Vec<String> = colours.iter().map(|c| string(c)).collect();
        format!("[{}]", list.join(", "))
    };
    let colours: Vec<String> = file
        .colours
        .iter()
        .map(|c| format!("{}: {}", string(&c.name), array(c.srgb)))
        .collect();
    let mut liveries = Vec::new();
    for livery in &file.liveries {
        let mut marks = Vec::new();
        for mark in &livery.marks {
            match mark {
                MarkSpec::Roundel {
                    face,
                    centre_m,
                    diameter,
                    rings,
                } => {
                    let fits = split_face(face).is_some_and(|(surface, side)| {
                        let sides: &[&str] = if surface == "fuselage" {
                            &["top", "bottom", "right", "left"]
                        } else {
                            match data.file.surfaces.iter().find(|s| s.name == surface) {
                                Some(s) if s.kind == SurfaceKind::Horizontal => &["top", "bottom"],
                                Some(_) => &["right", "left"],
                                None => &[],
                            }
                        };
                        sides.contains(&side)
                    });
                    if !fits {
                        return Err(format!(
                            "livrees.ron, {}: face {face}, which the aircraft does not have",
                            livery.name
                        ));
                    }
                    marks.push(format!(
                        "{{\"kind\": \"roundel\", \"face\": {}, \"centre\": {}, \"diameter\": {}, \"rings\": {}}}",
                        string(face),
                        array(*centre_m),
                        number(diameter.si()),
                        names(rings)
                    ));
                }
                MarkSpec::Bands { node, colours } => {
                    if !contract.hinges.iter().any(|h| &h.node == node) {
                        return Err(format!(
                            "livrees.ron, {}: bands on {node}, which is not a control node",
                            livery.name
                        ));
                    }
                    marks.push(format!(
                        "{{\"kind\": \"bands\", \"node\": {}, \"colours\": {}}}",
                        string(node),
                        names(colours)
                    ));
                }
            }
        }
        liveries.push(format!(
            "{{\"name\": {}, \"fabric\": {}, \"metal\": {}, \"marks\": [{}]}}",
            string(&livery.name),
            string(&livery.fabric),
            string(&livery.metal),
            marks.join(", ")
        ));
    }
    Ok(format!(
        "{{\"colours\": {{{}}}, \"liveries\": [{}]}}",
        colours.join(", "),
        liveries.join(", ")
    ))
}
