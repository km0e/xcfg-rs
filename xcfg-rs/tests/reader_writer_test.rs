mod common;

#[cfg(feature = "full")]
mod keep {
    use std::io::Cursor;

    use super::common::*;
    use xcfg::{Format, XCfg};

    #[test]
    fn test_toml_reader_writer() {
        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );

        let mut buf = Vec::new();
        test.save_to_writer(&mut buf, Format::Toml).unwrap();
        let file = Test::load_from_reader(Cursor::new(&buf), Format::Toml).unwrap();
        assert_eq!(file.into_inner(), test);
    }

    #[test]
    fn test_yaml_reader_writer() {
        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );

        let mut buf = Vec::new();
        test.save_to_writer(&mut buf, Format::Yaml).unwrap();
        let file = Test::load_from_reader(Cursor::new(&buf), Format::Yaml).unwrap();
        assert_eq!(file.into_inner(), test);
    }

    #[test]
    fn test_json_reader_writer() {
        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );

        let mut buf = Vec::new();
        test.save_to_writer(&mut buf, Format::Json).unwrap();
        let file = Test::load_from_reader(Cursor::new(&buf), Format::Json).unwrap();
        assert_eq!(file.into_inner(), test);
    }

    #[test]
    fn test_file_from_reader_with_fmt() {
        use xcfg::File;

        let test = Test::new(
            1,
            vec![0, 1, 2],
            SubTest::new(vec!["ab".to_string(), "cd".to_string()]),
        );

        let mut buf = Vec::new();
        test.save_to_writer(&mut buf, Format::Json).unwrap();

        let file = File::<Test, _>::from_reader_with_fmt(
            Cursor::new(&buf),
            std::path::PathBuf::from("config.json"),
            Format::Json,
        )
        .unwrap();
        assert_eq!(file.into_inner(), test);
    }
}
