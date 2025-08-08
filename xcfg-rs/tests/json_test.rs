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
        let path = "./test.json";
        defer! {
            std::fs::remove_file(path).unwrap();
        }
        test.save(path).unwrap();
        assert_eq!(
            Test::load(path)
                .expect("Failed to load or default")
                .into_inner(),
            test
        );
        std::fs::write(
            path,
            r#"
{
    "a": 1,
    "b": [0, 1, 2],
    "sub": {
        "c": ["ab", "cd"]
    }
}
"#,
        )
        .unwrap();

        assert_eq!(
            Test::load(path)
                .expect("Failed to load or default")
                .into_inner(),
            test
        );
    }
}
