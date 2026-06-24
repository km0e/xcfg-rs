# xcfg-rs

A simple tool to adapt different configuration file formats.

## Features

- Load and save TOML, YAML, and JSON configuration files.
- Derive `XCfg` for configuration structs.
- Stream data from any `std::io::Read` / `std::io::Write` source.
- Deterministic format selection when multiple files match the same base name.

## Usage

First, add `serde` and `xcfg-rs` to your `Cargo.toml`:

```sh
cargo add serde -F derive
cargo add xcfg-rs -F full
```

Then, use `XCfg` to load configuration from different file formats:

```rust
use serde::{Deserialize, Serialize};
use xcfg::XCfg;

#[derive(Debug, Serialize, Deserialize, XCfg)]
struct Config {
    name: String,
    age: u32,
}

fn main() {
    let config = Config::load("config")
        .expect("Failed to load config.[toml|yaml|yml|json]")
        .into_inner();
    println!("{:?}", config);
}
```

This example is also available in the `xcfg-rs/example` directory. You can clone this [repo](https://github.com/km0e/xcfg-rs.git) and run the example:

```sh
cd xcfg-rs && cargo run --example full --features full
```

## Reader / Writer API

You can also serialize to and deserialize from any `std::io::Write` / `std::io::Read` source:

```rust
use std::io::Cursor;
use serde::{Deserialize, Serialize};
use xcfg::{Format, XCfg};

#[derive(XCfg, Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    name: String,
}

let mut buf = Vec::new();
Config { name: "foo".into() }.save_to_writer(&mut buf, Format::Json).unwrap();
let config = Config::load_from_reader(Cursor::new(&buf), Format::Json)
    .unwrap()
    .into_inner();
assert_eq!(config, Config { name: "foo".into() });
```

## Migrating to 0.4

- `Error::InvalidPath` and `Error::UnknownFileFormat` now carry the path. If you were matching these variants without fields, update your match arms.
- `File::to_string` is deprecated; use `File::serialize_to_string` instead.

## License

This project is licensed under the MIT License.
