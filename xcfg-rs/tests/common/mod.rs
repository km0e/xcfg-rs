use serde::{Deserialize, Serialize};
use xcfg::XCfg;
#[derive(XCfg, Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Test {
    pub a: i32,
    pub b: Vec<i32>,
    pub sub: SubTest,
}
impl Test {
    pub fn new(a: i32, b: Vec<i32>, sub: SubTest) -> Self {
        Self { a, b, sub }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SubTest {
    pub c: Vec<String>,
}
impl SubTest {
    pub fn new(c: Vec<String>) -> Self {
        Self { c }
    }
}
