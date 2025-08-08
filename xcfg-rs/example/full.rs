use serde::{Deserialize, Serialize};
use xcfg::XCfg;

#[derive(Debug, Serialize, Deserialize, XCfg)]
struct Config {
    name: String,
    age: u32,
}

fn main() {
    let config = Config::load("config")
        .expect("Failed to load config.[toml|yaml|json]")
        .into_inner();
    println!("{:?}", config);
}
