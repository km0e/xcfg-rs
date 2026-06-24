use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid path `{path}`: {reason}")]
    InvalidPath { path: PathBuf, reason: &'static str },
    #[error("unknown file format for path `{path}`")]
    UnknownFileFormat { path: PathBuf },
    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[cfg(feature = "yaml")]
    #[error(transparent)]
    Yaml(#[from] serde_yml::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl Error {
    /// Helper to construct an `InvalidPath` error with the given reason.
    pub fn invalid_path<P: Into<PathBuf>>(path: P, reason: &'static str) -> Self {
        Self::InvalidPath {
            path: path.into(),
            reason,
        }
    }

    /// Helper to construct an `UnknownFileFormat` error.
    pub fn unknown_format<P: Into<PathBuf>>(path: P) -> Self {
        Self::UnknownFileFormat { path: path.into() }
    }
}
