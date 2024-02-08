use config_rs::File;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Test {
    a: i32,
    b: Vec<i32>,
    sub: SubTest,
}
impl Default for Test {
    fn default() -> Self {
        Self {
            a: 0,
            b: vec![],
            sub: SubTest::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SubTest {
    c: Vec<String>,
}
impl Default for SubTest {
    fn default() -> Self {
        Self { c: vec![] }
    }
}
#[test]
fn main() {
    let test = Test {
        a: 1,
        b: vec![0, 1, 2],
        sub: SubTest {
            c: vec!["ab".to_string(), "cd".to_string()],
        },
    };
    let path = "./test.yaml";
    let mut f = File::new().path(path);
    f.inner = test.clone();
    f.save().unwrap();
    f.inner = Test::default();
    f.load().unwrap();
    assert_eq!(f.inner, test);
    std::fs::remove_file(path).unwrap();
}
