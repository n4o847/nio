#[test]
fn test_parse() {
    use std::fs;

    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        insta::glob!("inputs/*.nio", |path| {
            let input = fs::read_to_string(path).unwrap();
            let result = nio_parser::parse(&input);
            insta::assert_debug_snapshot!(&result);
        });
    });
}
