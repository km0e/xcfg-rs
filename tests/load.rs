mod common;
#[cfg(feature = "toml")]
mod keep {
    use super::common::*;
    use xcfg::{load, File};
    #[test]
    fn main() {
        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );
        let path = "./test.toml";
        let mut f = File::default().path(path);
        f.inner = test.clone();
        f.save().unwrap();
        assert_eq!(load::<Test>(path).unwrap(), test);
        std::fs::remove_file(path).unwrap();
    }
}
