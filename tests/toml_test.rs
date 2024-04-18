mod common;
#[cfg(feature = "toml")]
mod keep {
    use super::common::*;
    use xcfg::File;
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
        f.inner = Test::default();
        f.load().unwrap();
        assert_eq!(f.inner, test);
        std::fs::remove_file(path).unwrap();
    }
}
