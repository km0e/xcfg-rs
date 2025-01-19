use std::path::{Path, PathBuf};

use super::error::Error;

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
// ugly but works
macro_rules! fmt_impl {
    ($([$name:literal, $mod:ident, $fmt:ident, $ext:pat]),*) => {
        $(
            #[cfg(feature = $name)]
            mod $mod;
        )*

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Format {
            $(
                #[cfg(feature = $name)]
                $fmt,
            )*
        }
        impl Format {
            pub fn match_ext(ext: &str) -> Option<Self> {
                match ext {
                    $(
                        #[cfg(feature = $name)]
                        $ext => Some(Self::$fmt),
                    )*
                    _ => None,
                }
            }
            pub fn serialize<T>(&self, input: &T) -> Result<String, Error>
            where
                T: serde::Serialize,
            {
                match self {
                    $(
                        #[cfg(feature = $name)]
                        Self::$fmt => $mod::to_string(input),
                    )*
                }
            }
            pub fn deserialize<T>(&self, input: &str) -> Result<T, Error>
            where
                T: serde::de::DeserializeOwned,
            {
                match self {
                    $(
                        #[cfg(feature = $name)]
                        Self::$fmt => $mod::from_str(input),
                    )*
                }
            }
        }
    };
}
fmt_impl!(
    ["toml", toml_impl, Toml, "toml"],
    ["yaml", yaml_impl, Yaml, "yaml" | "yml"],
    ["json", json_impl, Json, "json"]
);

mod file_impl {
    use super::Format;
    #[derive(Debug, PartialEq, Clone)]
    pub enum LoadFormat {
        Unknown,
        Any,
        Format(Format),
    }
    use std::path::Path;

    use crate::error::Error;

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
        T: serde::de::DeserializeOwned,
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
}
impl<T> File<T, PathBuf> {
    pub fn any_load<AsP>(path: AsP) -> Result<File<T, PathBuf>, Error>
    where
        AsP: AsRef<Path>,
        T: serde::de::DeserializeOwned,
    {
        let mut parent = path.as_ref().parent().ok_or(Error::InvalidPath)?;
        if parent.as_os_str().is_empty() {
            parent = Path::new(".");
        }
        let fname = path
            .as_ref()
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(Error::InvalidPath)?;
        for entry in std::fs::read_dir(parent)? {
            let entry_path = entry?.path();
            if !entry_path.is_file() {
                continue;
            }
            let name = match entry_path.file_name().and_then(|name| name.to_str()) {
                Some(name) => name,
                None => continue,
            };
            if !name.starts_with(fname) {
                continue;
            }
            let load_fmt = file_impl::load_fmt(name);
            match load_fmt {
                file_impl::LoadFormat::Unknown | file_impl::LoadFormat::Any => continue,
                file_impl::LoadFormat::Format(fmt) => {
                    return File::with_fmt(entry_path, fmt);
                }
            }
        }
        Err(Error::InvalidPath)
    }
}
impl<T, P> File<T, P>
where
    P: AsRef<Path>,
{
    pub fn into_inner(self) -> T {
        self.inner
    }
    pub fn new(path: P, inner: T) -> Result<Self, Error> {
        match file_impl::load_fmt(&path) {
            file_impl::LoadFormat::Unknown | file_impl::LoadFormat::Any => {
                Err(Error::UnknownFileFormat)
            }
            file_impl::LoadFormat::Format(fmt) => Ok(Self { path, fmt, inner }),
        }
    }
    pub fn with_fmt(path: P, fmt: Format) -> Result<Self, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let inner = file_impl::load(fmt, path.as_ref())?;
        Ok(Self { path, fmt, inner })
    }
    pub fn load(mut self) -> Result<(), Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.inner = file_impl::load(self.fmt, self.path.as_ref())?;
        Ok(())
    }
    pub fn to_string(&self) -> Result<String, Error>
    where
        T: serde::Serialize,
    {
        let buf = self.fmt.serialize(&self.inner)?;
        Ok(buf)
    }
    pub fn save(&self) -> Result<(), Error>
    where
        T: serde::Serialize,
    {
        let buf = self.to_string()?;
        let parent = self.path.as_ref().parent().ok_or(Error::InvalidPath)?;
        std::fs::create_dir_all(parent)?;
        std::fs::write(self.path.as_ref(), buf)?;
        Ok(())
    }
}

pub trait XCfg {
    fn with_format<P: AsRef<Path>>(path: P, fmt: Format) -> Result<File<Self, P>, Error>
    where
        Self: serde::de::DeserializeOwned,
    {
        File::with_fmt(path, fmt)
    }
    /// # Example
    ///
    /// ```rust
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
    fn load<P: AsRef<Path>>(path: P) -> Result<File<Self, PathBuf>, Error>
    where
        Self: serde::de::DeserializeOwned,
    {
        use file_impl::LoadFormat;
        let inner = match file_impl::load_fmt(&path) {
            LoadFormat::Any => File::any_load(path)?,
            LoadFormat::Unknown => {
                return Err(Error::UnknownFileFormat);
            }
            LoadFormat::Format(fmt) => {
                let inner = file_impl::load(fmt, path.as_ref())?;
                let path = path.as_ref().to_path_buf();
                File { path, fmt, inner }
            }
        };
        Ok(inner)
    }
    /// # Example
    ///
    /// ```rust
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
    fn load_or_default<P: AsRef<Path>>(path: P) -> Result<File<Self, P>, Error>
    where
        Self: Default + serde::de::DeserializeOwned,
    {
        use file_impl::LoadFormat;
        let inner = match file_impl::load_fmt(&path) {
            LoadFormat::Format(fmt) => {
                let inner = file_impl::load(fmt, path.as_ref()).unwrap_or_default();
                File { path, fmt, inner }
            }
            _ => {
                return Err(Error::UnknownFileFormat);
            }
        };
        Ok(inner)
    }
    /// # Example
    ///
    /// ```rust
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
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>
    where
        Self: serde::Serialize,
    {
        File::new(path, self)?.save()
    }
    /// # Example
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
    ///    },
    /// };
    /// let right = r#"a = 1
    /// b = [0, 1, 2]
    ///
    /// [sub]
    /// c = ["ab", "cd"]
    /// "#;
    /// assert_eq!(test.fmt_to_string(Format::Toml).unwrap(), right);
    fn fmt_to_string(&self, fmt: Format) -> Result<String, Error>
    where
        Self: serde::Serialize,
    {
        fmt.serialize(&self)
    }
}
