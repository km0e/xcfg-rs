use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::format::Format;

/// Priority order used by `File::any_load` when multiple configuration files
/// match the same base name.
const ANY_LOAD_PRIORITY: &[Format] = &[
    #[cfg(feature = "toml")]
    Format::Toml,
    #[cfg(feature = "yaml")]
    Format::Yaml,
    #[cfg(feature = "json")]
    Format::Json,
];

/// A configuration file handle.
///
/// `T` is the deserialized configuration type. `P` is the path type, defaulting
/// to [`PathBuf`].
#[derive(Debug)]
pub struct File<T, P = PathBuf>
where
    P: AsRef<Path>,
{
    pub path: P,
    pub fmt: Format,
    pub inner: T,
}

impl<T, P> Clone for File<T, P>
where
    T: Clone,
    P: AsRef<Path> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            fmt: self.fmt,
            inner: self.inner.clone(),
        }
    }
}

impl<T> File<T, PathBuf> {
    /// Load a file by matching the base name against supported extensions.
    ///
    /// If multiple files match (e.g. `config.toml` and `config.yaml`), the
    /// format is chosen according to the internal `ANY_LOAD_PRIORITY` constant.
    pub fn any_load<AsP>(path: AsP) -> Result<File<T, PathBuf>, Error>
    where
        AsP: AsRef<Path>,
        T: serde::de::DeserializeOwned + 'static,
    {
        let path_ref = path.as_ref();
        let mut parent = path_ref
            .parent()
            .ok_or_else(|| Error::invalid_path(path_ref, "path has no parent directory"))?;
        if parent.as_os_str().is_empty() {
            parent = Path::new(".");
        }

        let fname = path_ref
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::invalid_path(path_ref, "path has no valid file name"))?;

        // Collect candidates without constructing PathBuf for every entry.
        let mut candidates: Vec<(PathBuf, Format)> = Vec::new();
        for entry in std::fs::read_dir(parent)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }

            let name = entry.file_name();
            let Some(name_str) = name.to_str() else {
                continue;
            };
            if !name_str.starts_with(fname) {
                continue;
            }

            if let LoadFormat::Format(fmt) = load_fmt(name_str) {
                candidates.push((entry.path(), fmt));
            }
        }

        // Choose the highest-priority format.
        for &preferred in ANY_LOAD_PRIORITY {
            if let Some((path, _)) = candidates.iter().find(|(_, fmt)| *fmt == preferred) {
                return File::with_fmt(path.clone(), preferred);
            }
        }

        Err(Error::invalid_path(
            path_ref,
            "no matching configuration file found",
        ))
    }
}

impl<T, P> File<T, P>
where
    P: AsRef<Path>,
{
    /// Consume the file handle and return the inner configuration value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Create a new file handle without loading.
    ///
    /// The path extension must be known. This function only validates the
    /// extension and wraps the value; it does not read from disk.
    pub fn new(path: P, inner: T) -> Result<Self, Error> {
        match load_fmt(&path) {
            LoadFormat::Unknown | LoadFormat::Any => Err(Error::unknown_format(path.as_ref())),
            LoadFormat::Format(fmt) => Ok(Self { path, fmt, inner }),
        }
    }

    /// Load a file with an explicit format.
    pub fn with_fmt(path: P, fmt: Format) -> Result<Self, Error>
    where
        T: serde::de::DeserializeOwned + 'static,
    {
        let inner = load(fmt, path.as_ref())?;
        Ok(Self { path, fmt, inner })
    }

    /// Load from a reader with an explicit format and path.
    ///
    /// The path is stored in the returned handle but is not read from.
    pub fn from_reader_with_fmt<R>(reader: R, path: P, fmt: Format) -> Result<Self, Error>
    where
        R: Read,
        T: serde::de::DeserializeOwned + 'static,
    {
        let inner = fmt.deserialize_from_reader(reader)?;
        Ok(Self { path, fmt, inner })
    }

    /// Reload the file contents from disk into the existing handle.
    pub fn load(mut self) -> Result<(), Error>
    where
        T: serde::de::DeserializeOwned + 'static,
    {
        self.inner = load(self.fmt, self.path.as_ref())?;
        Ok(())
    }

    /// Serialize the inner value to a string in the file's format.
    pub fn serialize_to_string(&self) -> Result<String, Error>
    where
        T: serde::Serialize,
    {
        self.fmt.serialize(&self.inner)
    }

    /// Deprecated alias for [`Self::serialize_to_string`].
    #[deprecated(since = "0.4.0", note = "use `serialize_to_string` instead")]
    pub fn to_string(&self) -> Result<String, Error>
    where
        T: serde::Serialize,
    {
        self.serialize_to_string()
    }

    /// Write the serialized value to disk, creating parent directories as needed.
    pub fn save(&self) -> Result<(), Error>
    where
        T: serde::Serialize,
    {
        let buf = self.serialize_to_string()?;
        let parent = self.path.as_ref().parent().ok_or_else(|| {
            Error::invalid_path(self.path.as_ref(), "path has no parent directory")
        })?;
        std::fs::create_dir_all(parent)?;
        std::fs::write(self.path.as_ref(), buf)?;
        Ok(())
    }

    /// Serialize the inner value to a writer in the file's format.
    pub fn save_to_writer<W: Write>(&self, writer: W) -> Result<(), Error>
    where
        T: serde::Serialize,
    {
        self.fmt.serialize_to_writer(writer, &self.inner)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LoadFormat {
    Unknown,
    Any,
    Format(Format),
}

pub fn load_fmt<P: AsRef<Path>>(path: P) -> LoadFormat {
    match path.as_ref().extension() {
        Some(ext) => match ext.to_str() {
            Some("") => LoadFormat::Any,
            None => LoadFormat::Unknown,
            Some(ext) => match Format::match_ext(ext) {
                Some(fmt) => LoadFormat::Format(fmt),
                _ => LoadFormat::Unknown,
            },
        },
        None => LoadFormat::Any,
    }
}

pub fn load<T, P: AsRef<Path>>(fmt: Format, path: P) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + 'static,
{
    fmt.deserialize(&std::fs::read_to_string(path)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_fmt() {
        let path = Path::new("test.toml");
        assert_eq!(load_fmt(path), LoadFormat::Format(Format::Toml));
        let path = Path::new("test");
        assert_eq!(load_fmt(path), LoadFormat::Any);
        let path = Path::new("test.");
        assert_eq!(load_fmt(path), LoadFormat::Any);
        let path = Path::new("test.unknown");
        assert_eq!(load_fmt(path), LoadFormat::Unknown);
    }
}
