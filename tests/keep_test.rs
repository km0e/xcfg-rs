use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use xcfg::{keep::Keeper, File};
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
    let path = "./test.toml";
    let mut f = File::new().path(path);
    f.inner = test.clone();
    let amf = Arc::new(Mutex::new(f));
    let move_amf = amf.clone();
    spawn(|| {
        let mut keeper = Keeper::new(move_amf);
        loop {
            use xcfg::keep::Action;
            match keeper.run() {
                Ok(Action::TermSave) => {}
                Ok(Action::Load) => {}
                Err(_) => {
                    panic!("unexcept");
                }
            }
        }
    });
    sleep(Duration::from_millis(250));
    let mut test_f = File::new().path(path);
    let pid = std::process::id() as i32;
    unsafe {
        libc::kill(pid, libc::SIGTERM);
    }
    sleep(Duration::from_millis(50));
    test_f.load().unwrap();
    sleep(Duration::from_millis(50));
    assert_eq!(amf.lock().unwrap().inner, test_f.inner);
    test_f.inner.a = 2;
    test_f.save().unwrap();
    sleep(Duration::from_millis(50));
    unsafe {
        libc::kill(pid, libc::SIGHUP);
    }
    sleep(Duration::from_millis(50));
    assert_eq!(amf.lock().unwrap().inner, test_f.inner);
    std::fs::remove_file(path).unwrap();
}
