//! Data files of Panache blanc: RON schemas, loading and validation.
//!
//! Period units (mph, ft, hp…) are allowed in data files only; this crate converts
//! them to SI when loading. M0 only provides the generic RON loader.

use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

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
}

/// Parses RON text. `origin` names the source (file path, test name) in error messages.
pub fn from_ron_str<T: DeserializeOwned>(text: &str, origin: &str) -> Result<T, DataError> {
    ron::from_str(text).map_err(|err| DataError::Ron {
        origin: origin.to_owned(),
        position: err.span.start,
        message: err.code.to_string(),
    })
}

/// Reads and parses a RON file.
pub fn load_ron<T: DeserializeOwned>(path: &Path) -> Result<T, DataError> {
    let text = std::fs::read_to_string(path).map_err(|source| DataError::Io {
        path: path.to_owned(),
        source,
    })?;
    from_ron_str(&text, &path.display().to_string())
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
