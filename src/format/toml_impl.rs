use crate::error::Error;

#[macro_export]
macro_rules! toml_ext {
    () => {
        "toml"
    };
}

pub fn from_str<T>(input: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    toml::from_str(input).map_err(Error::from)
}

pub fn to_string<T>(input: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    toml::to_string(input).map_err(Error::from)
}
