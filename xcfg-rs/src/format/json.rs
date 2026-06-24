use std::io::{Read, Write};

use crate::error::Error;

pub fn from_str<T>(input: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + 'static,
{
    serde_json::from_str(input).map_err(Error::from)
}

pub fn from_reader<T, R>(reader: R) -> Result<T, Error>
where
    R: Read,
    T: serde::de::DeserializeOwned + 'static,
{
    serde_json::from_reader(reader).map_err(Error::from)
}

pub fn to_string<T>(input: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    serde_json::to_string(input).map_err(Error::from)
}

pub fn to_writer<T, W>(writer: W, input: &T) -> Result<(), Error>
where
    W: Write,
    T: serde::Serialize,
{
    serde_json::to_writer(writer, input).map_err(Error::from)
}
