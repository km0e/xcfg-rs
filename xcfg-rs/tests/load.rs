mod common;
#[cfg(feature = "toml")]
mod keep {
    use super::common::*;
    use scopeguard::defer;
    use xcfg::XCfg;
    #[test]
    fn main() {
        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );
        let path = "./test.toml".to_string();
        defer! {
            std::fs::remove_file(&path).unwrap();
        }
        test.save(&path).unwrap();
        assert_eq!(
            Test::load(path.clone())
                .expect("Failed to load or default")
                .into_inner(),
            test
        );
    }
}
