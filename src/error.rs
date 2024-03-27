use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorKind {
    Unknown,
    FileTypeError,
    Serialize,
    Deserialize,
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => Self::new(ErrorKind::Unknown, "File not found"),
            _ => Self::new(ErrorKind::Unknown, &e.to_string()),
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}
impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Self {
        Self {
            kind,
            message: msg.to_string(),
        }
    }
}
