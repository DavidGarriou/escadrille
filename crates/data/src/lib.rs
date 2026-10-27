//! Data files of Panache blanc: RON schemas, loading and validation.
//!
//! Period units (mph, ft, hp…) are allowed in data files only; this crate converts
//! them to SI when loading ([`units`]). Every number of a data file carries its origin
//! ([`origin`]); files without it are refused.

pub mod aircraft;
pub mod airfoil;
pub mod essais;
pub mod frames;
pub mod livery;
pub mod masses;
pub mod mockup;
pub mod model;
pub mod origin;
pub mod propeller;
pub mod units;

use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::origin::{MissingOrigin, missing_origins};

/// Error raised while reading or parsing a data file.
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("cannot read {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid RON in {origin} at {position}: {message}")]
    Ron {
        origin: String,
        position: ron::error::Position,
        message: String,
    },
    #[error("{origin}: numbers without an origin (// S:, // C: or // E:) on lines {}", lines(.missing))]
    MissingOrigin {
        origin: String,
        missing: Vec<MissingOrigin>,
    },
    #[error("{origin}: {message}")]
    Invalid { origin: String, message: String },
}

fn lines(missing: &[MissingOrigin]) -> String {
    missing
        .iter()
        .map(|m| m.line.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Parses RON text. `origin` names the source (file path, test name) in error messages.
pub fn from_ron_str<T: DeserializeOwned>(text: &str, origin: &str) -> Result<T, DataError> {
    ron::from_str(text).map_err(|err| DataError::Ron {
        origin: origin.to_owned(),
        position: err.span.start,
        message: err.code.to_string(),
    })
}

fn read(path: &Path) -> Result<String, DataError> {
    std::fs::read_to_string(path).map_err(|source| DataError::Io {
        path: path.to_owned(),
        source,
    })
}

/// Reads and parses a RON file.
pub fn load_ron<T: DeserializeOwned>(path: &Path) -> Result<T, DataError> {
    from_ron_str(&read(path)?, &path.display().to_string())
}

/// Parses RON text after checking that every number has an origin.
pub fn from_checked_str<T: DeserializeOwned>(text: &str, origin: &str) -> Result<T, DataError> {
    let missing = missing_origins(text);
    if !missing.is_empty() {
        return Err(DataError::MissingOrigin {
            origin: origin.to_owned(),
            missing,
        });
    }
    from_ron_str(text, origin)
}

/// Reads a data file whose numbers must all have an origin, and parses it.
pub fn load_checked<T: DeserializeOwned>(path: &Path) -> Result<T, DataError> {
    from_checked_str(&read(path)?, &path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Sample {
        name: String,
        value_si: f64,
    }

    #[test]
    fn parses_valid_ron() {
        let sample: Sample = from_ron_str("(name: \"sample\", value_si: 1.5)", "test").unwrap();
        assert_eq!(
            sample,
            Sample {
                name: "sample".into(),
                value_si: 1.5
            }
        );
    }

    #[test]
    fn reports_origin_and_position_on_invalid_ron() {
        let err =
            from_ron_str::<Sample>("(name: \"sample\",\n value_si: oops)", "spad.ron").unwrap_err();
        let text = err.to_string();
        assert!(text.contains("spad.ron"), "{text}");
        assert!(text.contains("2:"), "line number missing: {text}");
    }

    #[test]
    fn reports_missing_file() {
        let err = load_ron::<Sample>(Path::new("does/not/exist.ron")).unwrap_err();
        assert!(matches!(err, DataError::Io { .. }));
    }
}
