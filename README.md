# config-rs
a simple tool to adapt different configuration file format

## plan

- [x] intergrate with toml, yaml, json.
...

## usage
First, we need to add `serde` and `xcfg` to our `Cargo.toml`:
```sh
cargo add serde -F derive
cargo add xcfg -F full
```
Then, we can use `XCfg` to load configuration from different file formats:
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
This example is also available in the `example` directory. You can clone this [repo](https://github.com/km0e/xcfg-rs.git) and run the example:
```sh
cd example && cargo r --example full --features full
```