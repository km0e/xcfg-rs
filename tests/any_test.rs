mod common;
#[cfg(feature = "full")]
mod any {
    use super::common::*;
    use xcfg::File;
    #[test]
    fn any_test() {
        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );
        let path = "./test.toml";
        let mut f = File::default().path(path);
        f.inner = test.clone();
        f.save().unwrap();
        let path = "./test.";
        let mut f = File::default().path(path);
        f.inner = Test::default();
        f.load().unwrap();
        assert_eq!(f.inner, test);
        let path = "test.";
        let mut f = File::default().path(path);
        f.inner = Test::default();
        f.load().unwrap();
        assert_eq!(f.inner, test);
        std::fs::remove_file(f.path).unwrap();
    }
}
