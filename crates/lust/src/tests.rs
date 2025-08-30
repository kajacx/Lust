// AUTOMATIC GENERATION START

#[test]
fn test_gate_pass() {
    test_file("tests/gate_pass.lua");
}

#[test]
fn test_union_fail() {
    test_file("tests/union_fail.lua");
}

#[test]
fn test_union_pass() {
    test_file("tests/union_pass.lua");
}

#[test]
fn test_variable_fail() {
    test_file("tests/variable_fail.lua");
}

#[test]
fn test_variable_pass() {
    test_file("tests/variable_pass.lua");
}

// AUTOMATIC GENERATION END

fn test_file(path: &str) {
    let output = std::process::Command::new("cargo")
        .args(["run", "-p", "lust", "--quiet", "--", path])
        .output()
        .unwrap();

    if path.ends_with("_pass.lua") {
        assert!(
            output.status.success(),
            "Test {path} should pass but failed with:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else if path.ends_with("_fail.lua") {
        assert!(
            !output.status.success(),
            "Test {path} should fail but passed."
        );
    } else {
        panic!("Unrecognized file name: {path}");
    }

    let golden_path = format!("{path}.golden");

    if should_update() {
        std::fs::write(golden_path, &output.stderr).unwrap();
    } else {
        let expected = std::fs::read_to_string(golden_path).unwrap();
        let actual = String::from_utf8(output.stderr).unwrap();
        pretty_assertions::assert_eq!(expected, actual, "Output for test {path} differs.");
    }
}

fn should_update() -> bool {
    // std::env::args().any(|arg| arg == "--update")
    std::env::var("UPDATE_GOLDEN").is_ok()
}
