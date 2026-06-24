use std::io::{Read, Write};

use crate::error::Error;

pub fn from_str<T>(input: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + 'static,
{
    toml::from_str(input).map_err(Error::from)
}

pub fn from_reader<T, R>(mut reader: R) -> Result<T, Error>
where
    R: Read,
    T: serde::de::DeserializeOwned + 'static,
{
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    from_str(&buf)
}

pub fn to_string<T>(input: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    toml::to_string(input).map_err(Error::from)
}

pub fn to_writer<T, W>(mut writer: W, input: &T) -> Result<(), Error>
where
    W: Write,
    T: serde::Serialize,
{
    let s = to_string(input)?;
    writer.write_all(s.as_bytes()).map_err(Error::from)
}
