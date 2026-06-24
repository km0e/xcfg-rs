//! Format support for TOML, YAML and JSON.

use std::io::{Read, Write};

use crate::error::Error;

macro_rules! fmt_impl {
    ($([$name:literal, $mod:ident, $fmt:ident, $ext:pat]),*) => {
        $(
            #[cfg(feature = $name)]
            mod $mod;
        )*

        /// Supported configuration file formats.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Format {
            $(
                #[cfg(feature = $name)]
                $fmt,
            )*
        }

        impl Format {
            /// Match a file extension to a format.
            pub fn match_ext(ext: &str) -> Option<Self> {
                match ext {
                    $(
                        #[cfg(feature = $name)]
                        $ext => Some(Self::$fmt),
                    )*
                    _ => None,
                }
            }

            /// Serialize a value to a string in this format.
            pub fn serialize<T>(&self, input: &T) -> Result<String, Error>
            where
                T: serde::Serialize,
            {
                match self {
                    $(
                        #[cfg(feature = $name)]
                        Self::$fmt => $mod::to_string(input),
                    )*
                }
            }

            /// Deserialize a value from a string in this format.
            pub fn deserialize<T>(&self, input: &str) -> Result<T, Error>
            where
                T: serde::de::DeserializeOwned + 'static,
            {
                match self {
                    $(
                        #[cfg(feature = $name)]
                        Self::$fmt => $mod::from_str(input),
                    )*
                }
            }

            /// Serialize a value to a writer in this format.
            pub fn serialize_to_writer<T, W>(&self, writer: W, input: &T) -> Result<(), Error>
            where
                W: Write,
                T: serde::Serialize,
            {
                match self {
                    $(
                        #[cfg(feature = $name)]
                        Self::$fmt => $mod::to_writer(writer, input),
                    )*
                }
            }

            /// Deserialize a value from a reader in this format.
            pub fn deserialize_from_reader<T, R>(&self, reader: R) -> Result<T, Error>
            where
                R: Read,
                T: serde::de::DeserializeOwned + 'static,
            {
                match self {
                    $(
                        #[cfg(feature = $name)]
                        Self::$fmt => $mod::from_reader(reader),
                    )*
                }
            }
        }
    };
}

fmt_impl!(
    ["toml", toml, Toml, "toml"],
    ["yaml", yaml, Yaml, "yaml" | "yml"],
    ["json", json, Json, "json"]
);
