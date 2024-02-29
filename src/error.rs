#[derive(Debug)]
pub enum ErrorKind {
    Unknown,
    FileTypeError,
    Serialize,
    Deserialize,
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

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Self {
        Self {
            kind,
            message: msg.to_string(),
        }
    }
}
