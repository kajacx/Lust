use std::fs;
use std::path::Path;

fn main() {
    let test_dir = Path::new("tests");
    let mut test_fns = String::from("\n");

    for entry in fs::read_dir(test_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("lua") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            test_fns.push_str(&format!(
                r#"#[test]
fn test_{stem}() {{
    test_file("{path}");
}}

"#,
                stem = stem,
                path = path.display().to_string().replace('\\', "/"),
            ));
        }
    }

    // Read the template test file
    let test_file_path = Path::new("src/tests.rs");
    let contents = fs::read_to_string(test_file_path).unwrap();

    // Replace the block between markers
    let new_contents = replace_between(
        &contents,
        "// AUTOMATIC GENERATION START",
        "// AUTOMATIC GENERATION END",
        &test_fns,
    );

    fs::write(test_file_path, new_contents).unwrap();
}

fn replace_between(contents: &str, start: &str, end: &str, replacement: &str) -> String {
    let mut result = String::new();
    let mut inside = false;

    for line in contents.lines() {
        if line.trim() == start {
            result.push_str(start);
            result.push('\n');
            result.push_str(replacement);
            inside = true;
            continue;
        }
        if line.trim() == end {
            result.push_str(end);
            result.push('\n');
            inside = false;
            continue;
        }
        if !inside {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}
