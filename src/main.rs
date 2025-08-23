pub mod error;
pub mod parser;
pub mod typecheck;

use lalrpop_util::lalrpop_mod;

use crate::{
    error::LustError,
    typecheck::{TypecheckOutcome, TypecheckResult},
};

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

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LustType {
    Number,
    String,
}

fn main() {
    // test_it();

    let path = std::env::args().nth(1).expect("no path given");

    let expected = if path.ends_with("_pass.lua") {
        TypecheckResult::Pass
    } else if path.ends_with("_warn.lua") {
        TypecheckResult::Warn
    } else if path.ends_with("_fail.lua") {
        TypecheckResult::Fail
    } else {
        panic!("Unrecognized file name: {path}")
    };

    let outcome = analyze_file(&path);
    let result = outcome.result;

    for error in outcome.errors {
        eprintln!("{error}");
    }

    if expected != result {
        panic!("Expected result {expected:?} for test {path}, but got {result:?} instead.")
    } else {
        println!("Test {path} succeeded with result {result:?}.")
    }
}

fn analyze_file(filename: &str) -> TypecheckOutcome {
    let content = std::fs::read_to_string(filename).unwrap();
    let statements = crate::parser::parse_file_content(&content);

    let mut errors: Vec<LustError> = vec![];
    typecheck::analyze_statements(&statements, |error| errors.push(error));

    TypecheckOutcome {
        result: if errors.is_empty() {
            TypecheckResult::Pass
        } else {
            TypecheckResult::Fail
        },
        errors,
    }
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
