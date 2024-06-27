use super::error::{Error, ErrorKind};
use file_impl::FileType;

#[derive(Debug)]
pub struct File<T> {
    pub path: String,
    file_type: FileType,
    pub inner: T,
}
impl<T: Default> Default for File<T> {
    fn default() -> Self {
        Self {
            path: String::from(""),
            file_type: FileType::Unknown,
            inner: T::default(),
        }
    }
}
mod file_impl {
    #[derive(Debug, PartialEq)]
    pub enum FileType {
        Unknown,
        #[cfg(feature = "toml")]
        Toml,
        #[cfg(feature = "yaml")]
        Yaml,
        Any,
    }
    use std::env::current_dir;
    use std::fs::canonicalize;
    use std::path::Path;

    use super::Error;
    use super::ErrorKind;
    pub fn file_type(path: &str) -> FileType {
        match path.rsplit_once('.').map(|(_, ext)| ext) {
            #[cfg(feature = "toml")]
            Some("toml") => FileType::Toml,
            #[cfg(feature = "yaml")]
            Some("yaml") | Some("yml") => FileType::Yaml,
            Some("") => FileType::Any,
            _ => FileType::Unknown,
        }
    }
    pub fn any_load<T>(spath: &str) -> Result<(FileType, String, T), Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = Path::new(spath);
        let fname = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(Error::new(
                ErrorKind::FileTypeError,
                "Path is not a valid file name",
            ))?;
        let parent = if fname == spath {
            current_dir()
        } else {
            match path.parent() {
                None => current_dir(),
                Some(parent) => canonicalize(parent),
            }
        }
        .map_err(Error::from)?;
        let entries = std::fs::read_dir(parent).map_err(Error::from)?;
        for entry in entries {
            let entry = entry.map_err(Error::from)?;
            let path = entry.path();
            if path.is_file() {
                let name = match path.file_name().and_then(|name| name.to_str()) {
                    Some(name) => name,
                    None => continue,
                };
                if name.starts_with(fname) {
                    let ft = file_type(name);
                    if ft != FileType::Any && ft != FileType::Unknown {
                        let path = path
                            .to_str()
                            .ok_or(Error::new(
                                ErrorKind::FileTypeError,
                                "Path is not a valid file name",
                            ))?
                            .to_string();
                        let inner = load(&ft, &path)?;
                        return Ok((ft, path, inner));
                    }
                }
            }
        }
        Err(Error::new(ErrorKind::FileTypeError, "No file found"))
    }
    pub fn load<T>(ft: &FileType, path: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let buf = std::fs::read_to_string(path).map_err(Error::from)?;
        let inner = match ft {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::from_str(&buf)
                .map_err(|e| Error::new(ErrorKind::Deserialize, &e.to_string()))?,
            #[cfg(feature = "yaml")]
            FileType::Yaml => serde_yaml::from_str(&buf)
                .map_err(|e| Error::new(ErrorKind::Deserialize, &e.to_string()))?,
            _ => unreachable!(),
        };
        Ok(inner)
    }
}
impl<T> File<T> {
    pub fn path(mut self, path: &str) -> Self {
        self.file_type = file_impl::file_type(path);
        self.path = path.to_string();
        self
    }
}
impl<T: serde::de::DeserializeOwned> File<T> {
    pub fn load(&mut self) -> Result<(), Error> {
        match &self.file_type {
            FileType::Any => {
                let (ft, path, inner) = file_impl::any_load(&self.path)?;
                self.file_type = ft;
                self.path = path;
                self.inner = inner;
            }
            FileType::Unknown => {
                return Err(Error::new(ErrorKind::FileTypeError, "Unknown file type"));
            }
            ft => {
                self.inner = file_impl::load(ft, &self.path)?;
            }
        };
        Ok(())
    }
}
impl<T: serde::Serialize> File<T> {
    pub fn to_string(&self) -> Result<String, Error> {
        if self.file_type == FileType::Unknown {
            return Err(Error::new(ErrorKind::FileTypeError, "Unknown file type"));
        }
        let buf = match self.file_type {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::to_string(&self.inner)
                .map_err(|e| Error::new(ErrorKind::Serialize, &e.to_string()))?,
            #[cfg(feature = "yaml")]
            FileType::Yaml => serde_yaml::to_string(&self.inner)
                .map_err(|e| Error::new(ErrorKind::Serialize, &e.to_string()))?,
            _ => unreachable!(),
        };
        Ok(buf)
    }
    pub fn save(&self) -> Result<(), Error> {
        let buf = self.to_string()?;
        let parent = std::path::Path::new(&self.path).parent().ok_or(Error::new(
            ErrorKind::FileTypeError,
            "Path is not a valid file name",
        ))?;
        std::fs::create_dir_all(parent).map_err(Error::from)?;
        std::fs::write(&self.path, buf).map_err(Error::from)?;
        Ok(())
    }
}
pub fn load<T>(path: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let ft = file_impl::file_type(path);
    let inner = match ft {
        FileType::Any => {
            let (_, _, inner) = file_impl::any_load(path)?;
            inner
        }
        FileType::Unknown => {
            return Err(Error::new(ErrorKind::FileTypeError, "Unknown file type"));
        }
        ft => file_impl::load(&ft, path)?,
    };
    Ok(inner)
}
