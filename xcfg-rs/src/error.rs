use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Don't contain a valid file name")]
    InvalidPath,
    #[error("Unknown file format")]
    UnknownFileFormat,
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
