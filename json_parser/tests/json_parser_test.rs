use std::fs;
use std::path::Path;

fn parse_json_file(file_path: &Path) -> bool {
    let content = fs::read_to_string(file_path)
        .expect(&format!("Failed to read file {:?}", file_path));
    let tokens = your_crate::lexer::tokenize(&content);
    your_crate::parser::parse(&tokens)
}

#[test]
fn validate_step1_json_files() {
    let valid_dir = Path::new("tests/step1/valid");
    let invalid_dir = Path::new("tests/step1/invalid");

    // Testing valid JSON files
    for entry in fs::read_dir(valid_dir).expect("Failed to read valid directory") {
        let entry_path = entry.expect("Failed to read entry").path();
        assert!(
            parse_json_file(&entry_path),
            "File should be valid: {:?}",
            entry_path.file_name().unwrap()
        );
    }

    // Testing invalid JSON files
    for entry in fs::read_dir(invalid_dir).expect("Failed to read invalid directory") {
        let entry_path = entry.expect("Failed to read entry").path();
        assert!(
            !parse_json_file(&entry_path),
            "File should be invalid: {:?}",
            entry_path.file_name().unwrap()
        );
    }
}
