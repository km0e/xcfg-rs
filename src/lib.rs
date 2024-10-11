//! xcfg_rs is a simple configuration file loader and saver.
//!
//! # Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use xcfg::XCfg;
//! #[derive(XCfg, Serialize, Deserialize, PartialEq, Debug, Clone)]
//! pub struct Test {
//!     a: i32,
//!     b: Vec<i32>,
//!     sub: SubTest,
//! }
//! #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
//! pub struct SubTest {
//!     c: Vec<String>,
//! }
//! let test = Test {
//!     a: 1,
//!     b: vec![0, 1, 2],
//!     sub: SubTest {
//!         c: vec!["ab".to_string(), "cd".to_string()],
//!     },
//! };
//! let path = "./test.toml";
//! test.save(path).unwrap();
//! assert_eq!(Test::load(path).unwrap().into_inner(), test);
//! std::fs::remove_file(path).unwrap();

mod error;
mod format;
pub use error::Error;
pub use format::File;
pub use format::XCfg;
pub use xcfg_derive::XCfg;
