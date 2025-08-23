use lalrpop_util::lalrpop_mod;

lalrpop_mod!(luasyn);

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaStatement {
    Comment(LuaComment),
    VarDeclaration { name: String, value: LuaExpression },
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaComment {
    TypeAnnotation(LustType),
    Text(String),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaExpression {
    StringLiteral(String),
    NumberLiteral(f64),
}

#[derive(PartialEq, PartialOrd, Debug)]
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
    // test_it();

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
    analyze_statements(&statements)
}

fn analyze_statements(statements: &[LuaStatement]) -> TestResult {
    for slice in statements.windows(2) {
        if let LuaStatement::Comment(LuaComment::TypeAnnotation(var_type)) = &slice[0] {
            if let LuaStatement::VarDeclaration {
                name: var_name,
                value: val,
            } = &slice[1]
            {
                let val_type = get_type(val);
                if !can_assign(&val_type, var_type) {
                    eprintln!("Cannot assign value '{val:?}' of type '{val_type:?}' into the variable '{var_name}' of type '{var_type:?}'.");
                    return TestResult::Fail;
                }
            }
        }
    }
    TestResult::Pass
}

fn get_type(expr: &LuaExpression) -> LustType {
    match expr {
        LuaExpression::NumberLiteral(_) => LustType::Number,
        LuaExpression::StringLiteral(_) => LustType::String,
    }
}

fn can_assign(what: &LustType, into_what: &LustType) -> bool {
    what == into_what
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

#[allow(dead_code)]
fn test_it() {
    let content = std::fs::read_to_string("testsyn.lua").unwrap();
    let parser = testsyn::TestStatementsParser::new();
    let statements = parser.parse(&content).unwrap();

    println!();
    println!("--- TEST SUCCESS ---");
    println!("{statements:?}");
    println!();
}
