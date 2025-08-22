use lalrpop_util::lalrpop_mod;

lalrpop_mod!(luasyn);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum TestResult {
    Pass,
    Warn,
    Fail,
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    let expected = if path.ends_with("_pass.lua") {
        TestResult::Pass
    } else if path.ends_with("_warn.lua") {
        TestResult::Warn
    } else if path.ends_with("_fail.lua") {
        TestResult::Fail
    } else {
        panic!("Unrecognized file name: {path}")
    };

    let actual = analyze_file(&path);

    if actual != expected {
        panic!("Expected result {expected:?} for test {path}, but got {actual:?} instead.")
    } else {
        println!("Test {path} succeeded with result {actual:?}.")
    }
}

fn analyze_file(filename: &str) -> TestResult {
    let content = std::fs::read_to_string(filename).expect("Read file content");
    analyze_file_content(&content)
}

fn analyze_file_content(content: &str) -> TestResult {
    let parser = luasyn::TermParser::new();
    let parsed = match parser.parse(content) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error when parsing the input file:");
            eprintln!("{err:?}");
            return TestResult::Fail; // TODO:
        }
    };

    TestResult::Pass
}
