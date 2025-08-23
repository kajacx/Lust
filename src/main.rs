use lalrpop_util::lalrpop_mod;

lalrpop_mod!(luasyn);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LuaStatement {
    Comment(LuaComment),
    VarDeclaration { name: String, value: LuaExpression },
}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
// pub struct LuaComment(Vec<LuaCommentToken>);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LuaComment {
    TypeAnnotation(LustType),
    Text(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LuaExpression {
    StringLiteral(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LustType {
    Number,
    String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum TestResult {
    Pass,
    Warn,
    Fail,
}

fn main() {
    test_it();

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
    let content = std::fs::read_to_string(filename).unwrap();
    analyze_file_content(&content)
}

fn analyze_file_content(content: &str) -> TestResult {
    let parser = luasyn::LuaStatementsParser::new();
    let statements = match parser.parse(content) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error when parsing the input file:");
            eprintln!("{err:?}");
            return TestResult::Fail; // TODO:
        }
    };

    println!("{statements:?}");
    TestResult::Pass
}

lalrpop_mod!(testsyn);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TestStatement {
    Comment(TestComment),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TestComment {
    TypeAnnotation(TestType),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TestType {
    Number,
    String,
}

fn test_it() {
    let content = std::fs::read_to_string("tests/testsyn.lua").unwrap();
    let parser = testsyn::TestStatementParser::new();
    let statement = parser.parse(&content).unwrap();
    println!("{statement:?}");
}
