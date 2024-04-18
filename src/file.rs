use std::{env::current_dir, fs::canonicalize, path::Path};

use super::error::{Error, ErrorKind};
#[derive(Debug, PartialEq)]
enum FileType {
    Unknown,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
    Any,
}

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
impl<T> File<T> {
    pub fn path(mut self, path: &str) -> Self {
        self.file_type = match path.rsplit_once('.').map(|(_, ext)| ext) {
            #[cfg(feature = "toml")]
            Some("toml") => FileType::Toml,
            #[cfg(feature = "yaml")]
            Some("yaml") | Some("yml") => FileType::Yaml,
            Some("") => FileType::Any,
            _ => FileType::Unknown,
        };
        self.path = path.to_string();
        self
    }
}
impl<T: serde::de::DeserializeOwned> File<T> {
    fn any_load(&mut self) -> Result<(), Error> {
        let parent = match Path::new(&self.path).parent() {
            Some(p) => canonicalize(p).map_err(Error::from)?,
            None => current_dir().map_err(Error::from)?,
        };
        let entries = std::fs::read_dir(parent).map_err(Error::from)?;
        for entry in entries {
            let entry = entry.map_err(Error::from)?;
            let path = entry.path();
            if path.is_file() {
                let path = path.to_str().ok_or(Error::new(
                    ErrorKind::FileTypeError,
                    "Path is not a valid UTF-8 string",
                ))?;
                let file_type = match path.split('.').last() {
                    #[cfg(feature = "toml")]
                    Some("toml") => FileType::Toml,
                    #[cfg(feature = "yaml")]
                    Some("yaml") | Some("yml") => FileType::Yaml,
                    _ => continue,
                };
                if file_type == self.file_type {
                    self.path = path.to_string();
                    return self.load();
                }
            }
        }
        Ok(())
    }
    pub fn load(&mut self) -> Result<(), Error> {
        if self.file_type == FileType::Unknown {
            return Err(Error::new(ErrorKind::FileTypeError, "Unknown file type"));
        }
        let buf = std::fs::read_to_string(&self.path).map_err(Error::from)?;
        self.inner = match self.file_type {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::from_str(&buf)
                .map_err(|e| Error::new(ErrorKind::Deserialize, &e.to_string()))?,
            #[cfg(feature = "yaml")]
            FileType::Yaml => serde_yaml::from_str(&buf)
                .map_err(|e| Error::new(ErrorKind::Deserialize, &e.to_string()))?,
            _ => unreachable!(),
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
        std::fs::write(&self.path, buf).map_err(Error::from)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_file() {
        let path = "test.";
        let mut f = Path::new(path).parent().unwrap().to_path_buf();
        println!("{:?}", f);
    }
}
