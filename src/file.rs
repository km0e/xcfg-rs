#[derive(PartialEq)]
enum FileType {
    Unknown,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
}
#[derive(Debug)]
pub struct Error {
    pub message: String,
}
impl Error {
    fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}
pub struct File<F> {
    pub path: String,
    file_type: FileType,
    pub inner: F,
}
impl<F: Default> File<F> {
    pub fn new() -> Self {
        Self {
            path: String::from(""),
            file_type: FileType::Unknown,
            inner: F::default(),
        }
    }
}
impl<F> File<F> {
    pub fn path(mut self, path: &str) -> Self {
        self.file_type = match path.split('.').last() {
            #[cfg(feature = "toml")]
            Some("toml") => FileType::Toml,
            #[cfg(feature = "yaml")]
            Some("yaml") | Some("yml") => FileType::Yaml,
            _ => FileType::Unknown,
        };
        self.path = path.to_string();
        self
    }
}
impl<F: serde::de::DeserializeOwned> File<F> {
    pub fn load(&mut self, path: &str) -> Result<(), Error> {
        self.path = String::from(path);
        if self.file_type == FileType::Unknown {
            return Err(Error::new("Unknown file type"));
        }
        let buf = std::fs::read_to_string(path).map_err(|e| Error {
            message: format!("Failed to read file {}: {}", path, e),
        })?;
        self.inner = match self.file_type {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::from_str(&buf).map_err(|e| Error {
                message: format!("Failed to parse file {}: {}", path, e),
            })?,
            #[cfg(feature = "yaml")]
            FileType::Yaml => serde_yaml::from_str(&buf).map_err(|e| Error {
                message: format!("Failed to parse file {}: {}", path, e),
            })?,
            _ => unreachable!(),
        };
        Ok(())
    }
}
impl<F: serde::Serialize> File<F> {
    fn to_string(&self) -> Result<String, Error> {
        if self.file_type == FileType::Unknown {
            return Err(Error::new("Unknown file type"));
        }
        let buf = match self.file_type {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::to_string(&self.inner).map_err(|e| Error {
                message: format!("Failed to serialize file {}: {}", self.path, e),
            })?,
            #[cfg(feature = "yaml")]
            FileType::Yaml => serde_yaml::to_string(&self.inner).map_err(|e| Error {
                message: format!("Failed to serialize file {}: {}", self.path, e),
            })?,
            _ => unreachable!(),
        };
        Ok(buf)
    }
    pub fn save(&self) -> Result<(), Error> {
        let buf = self.to_string()?;
        std::fs::write(&self.path, buf).map_err(|e| Error {
            message: format!("Failed to write file {}: {}", self.path, e),
        })?;
        Ok(())
    }
}
