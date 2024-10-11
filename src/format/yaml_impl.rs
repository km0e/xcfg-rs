use crate::error::Error;

#[macro_export]
macro_rules! yaml_ext {
    () => {
        "yaml" | "yml"
    };
}

pub fn from_str<T>(input: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_yml::from_str(input).map_err(Error::from)
}

pub fn to_string<T>(input: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    serde_yml::to_string(input).map_err(Error::from)
}
