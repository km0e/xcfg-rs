use crate::error::Error;

#[macro_export]
macro_rules! json_ext {
    () => {
        "json"
    };
}

pub fn from_str<T>(input: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(input).map_err(Error::from)
}

pub fn to_string<T>(input: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    serde_json::to_string(input).map_err(Error::from)
}
