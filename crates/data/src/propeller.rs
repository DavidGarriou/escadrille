//! Propeller tables (`assets/aircraft/<id>/propeller.ron` or shared): C_T(J) and C_P(J)
//! from the J = 0 static point, and the four-quadrant anchors (flight model,
//! `propulsion::propeller`). Nondimensional: the diameter belongs to the aircraft file.

use escadrille_fdm::propulsion::propeller::{PropellerPoint, QuadrantAnchors, QuadrantPoint};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct TablePoint {
    pub j: f64,
    pub ct: f64,
    pub cp: f64,
}

/// C_T* and C_Q* of an anchor.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct AnchorPoint {
    pub ct: f64,
    pub cq: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Anchors {
    pub stopped: AnchorPoint,
    pub reversed_flow: AnchorPoint,
    pub reversed_rotation: AnchorPoint,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Propeller")]
pub struct PropellerFile {
    pub name: String,
    /// Where the table comes from: research sheet and source number.
    pub source: String,
    pub points: Vec<TablePoint>,
    pub anchors: Anchors,
}

impl PropellerFile {
    pub fn table(&self) -> Vec<PropellerPoint> {
        self.points
            .iter()
            .map(|p| PropellerPoint {
                j: p.j,
                ct: p.ct,
                cp: p.cp,
            })
            .collect()
    }

    pub fn anchors(&self) -> QuadrantAnchors {
        let point = |a: AnchorPoint| QuadrantPoint { ct: a.ct, cq: a.cq };
        QuadrantAnchors {
            stopped: point(self.anchors.stopped),
            reversed_flow: point(self.anchors.reversed_flow),
            reversed_rotation: point(self.anchors.reversed_rotation),
        }
    }
}
