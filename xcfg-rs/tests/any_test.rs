mod common;
#[cfg(feature = "full")]
mod any {
    use std::env::temp_dir;

    use super::common::*;
    use scopeguard::defer;
    use xcfg::XCfg;
    #[test]
    fn any_test() {
        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );
        let temp_dir = temp_dir();
        let path = temp_dir.join("test.toml");
        defer! {
            std::fs::remove_file(&path).unwrap();
        }
        test.save(&path).unwrap();
        assert_eq!(
            Test::load(temp_dir.join("test."))
                .expect("Failed to load or default")
                .into_inner(),
            test
        );
        assert_eq!(
            Test::load(temp_dir.join("test"))
                .expect("Failed to load or default")
                .into_inner(),
            test
        );
    }
}
