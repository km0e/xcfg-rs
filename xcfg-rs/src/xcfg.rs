use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::file::{File, LoadFormat, load_fmt};
use crate::format::Format;

/// Trait providing convenient `load` / `save` methods for configuration structs.
///
/// Types deriving this trait via `#[derive(XCfg)]` receive default
/// implementations. The type must also implement `serde::Serialize` and
/// `serde::Deserialize` for the methods to be usable.
pub trait XCfg {
    /// Load a file with an explicit format.
    fn with_format<P: AsRef<Path>>(path: P, fmt: Format) -> Result<File<Self, P>, Error>
    where
        Self: serde::de::DeserializeOwned + 'static,
    {
        File::with_fmt(path, fmt)
    }

    /// Load from a reader with an explicit format.
    ///
    /// The returned file handle uses an empty [`PathBuf`] as placeholder because
    /// the data did not originate from a path.
    fn load_from_reader<R: Read>(reader: R, fmt: Format) -> Result<File<Self, PathBuf>, Error>
    where
        Self: serde::de::DeserializeOwned + 'static,
    {
        File::from_reader_with_fmt(reader, PathBuf::new(), fmt)
    }

    /// Save to a writer in the given format.
    fn save_to_writer<W: Write>(&self, writer: W, fmt: Format) -> Result<(), Error>
    where
        Self: serde::Serialize + Sized,
    {
        fmt.serialize_to_writer(writer, self)
    }

    /// # Example
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// use xcfg::XCfg;
    /// #[derive(XCfg, Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct Test {
    ///     a: i32,
    ///     b: Vec<i32>,
    ///     sub: SubTest,
    /// }
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct SubTest {
    ///     c: Vec<String>,
    /// }
    ///
    /// let test = Test {
    ///     a: 1,
    ///     b: vec![0, 1, 2],
    ///     sub: SubTest {
    ///         c: vec!["ab".to_string(), "cd".to_string()],
    ///     },
    /// };
    /// let path = "./test.toml";
    /// test.save(path).unwrap();
    /// assert_eq!(Test::load(path).unwrap().into_inner(), test);
    /// std::fs::remove_file(path).unwrap();
    /// ```
    fn load<P: AsRef<Path>>(path: P) -> Result<File<Self, PathBuf>, Error>
    where
        Self: serde::de::DeserializeOwned + 'static,
    {
        let inner = match load_fmt(&path) {
            LoadFormat::Any => File::any_load(path)?,
            LoadFormat::Unknown => {
                return Err(Error::unknown_format(path.as_ref()));
            }
            LoadFormat::Format(fmt) => {
                let inner = crate::file::load(fmt, path.as_ref())?;
                let path = path.as_ref().to_path_buf();
                File { path, fmt, inner }
            }
        };
        Ok(inner)
    }

    /// # Example
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// use xcfg::XCfg;
    /// #[derive(XCfg, Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct Test {
    ///     a: i32,
    ///     b: Vec<i32>,
    ///     sub: SubTest,
    /// }
    /// impl Default for Test {
    ///     fn default() -> Self {
    ///         Self {
    ///             a: 0,
    ///             b: vec![],
    ///             sub: SubTest::default(),
    ///         }
    ///     }
    /// }
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct SubTest {
    ///     c: Vec<String>,
    /// }
    /// impl Default for SubTest {
    ///     fn default() -> Self {
    ///         Self { c: vec![] }
    ///     }
    /// }
    ///
    /// let test = Test {
    ///     a: 1,
    ///     b: vec![0, 1, 2],
    ///     sub: SubTest {
    ///         c: vec!["ab".to_string(), "cd".to_string()],
    ///     },
    /// };
    /// let path = "./test.toml";
    /// let mut f = Test::load_or_default(path).unwrap();
    /// assert_eq!(f.inner, Test::default());
    /// f.inner = test.clone();
    /// f.save().unwrap();
    /// assert_eq!(Test::load(path).unwrap().into_inner(), test);
    /// std::fs::remove_file(path).unwrap();
    /// ```
    fn load_or_default<P: AsRef<Path>>(path: P) -> Result<File<Self, P>, Error>
    where
        Self: Default + serde::de::DeserializeOwned + 'static,
    {
        use std::io;

        let inner = match load_fmt(&path) {
            LoadFormat::Format(fmt) => {
                let inner = match crate::file::load(fmt, path.as_ref()) {
                    Ok(v) => v,
                    Err(Error::Io(e)) if e.kind() == io::ErrorKind::NotFound => Self::default(),
                    Err(e) => return Err(e),
                };
                File { path, fmt, inner }
            }
            _ => {
                return Err(Error::unknown_format(path.as_ref()));
            }
        };
        Ok(inner)
    }

    /// # Example
    ///
    /// ```rust,no_run
    /// use serde::{Deserialize, Serialize};
    /// use xcfg::XCfg;
    /// #[derive(XCfg, Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct Test {
    ///     a: i32,
    ///     b: Vec<i32>,
    ///     sub: SubTest,
    /// }
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct SubTest {
    ///     c: Vec<String>,
    /// }
    ///
    /// let test = Test {
    ///     a: 1,
    ///     b: vec![0, 1, 2],
    ///     sub: SubTest {
    ///         c: vec!["ab".to_string(), "cd".to_string()],
    ///     },
    /// };
    /// let path = "./test.toml";
    /// test.save(path).unwrap();
    /// std::fs::remove_file(path).unwrap();
    /// ```
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>
    where
        Self: serde::Serialize,
    {
        File::new(path, self)?.save()
    }

    /// # Example
    ///
    /// ```rust
    /// use serde::{Deserialize, Serialize};
    /// use xcfg::{XCfg, Format};
    /// #[derive(XCfg, Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct Test {
    ///     a: i32,
    ///     b: Vec<i32>,
    ///     sub: SubTest,
    /// }
    /// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    /// pub struct SubTest {
    ///     c: Vec<String>,
    /// }
    /// let test = Test {
    ///     a: 1,
    ///     b: vec![0, 1, 2],
    ///     sub: SubTest {
    ///         c: vec!["ab".to_string(), "cd".to_string()],
    ///     },
    /// };
    /// let right = r#"a = 1
    /// b = [0, 1, 2]
    ///
    /// [sub]
    /// c = ["ab", "cd"]
    /// "#;
    /// assert_eq!(test.fmt_to_string(Format::Toml).unwrap(), right);
    /// ```
    fn fmt_to_string(&self, fmt: Format) -> Result<String, Error>
    where
        Self: serde::Serialize,
    {
        fmt.serialize(&self)
    }
}
