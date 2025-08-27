pub mod error;
pub mod grammar;
pub mod parser;
pub mod typecheck;

use crate::{
    error::LustError,
    typecheck::{TypecheckOutcome, TypecheckResult},
};

lalrpop_util::lalrpop_mod!(luasyn);

fn main() {
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

    for error in &mut errors {
        error.location.filename = filename.to_string();
    }

    TypecheckOutcome {
        result: if errors.is_empty() {
            TypecheckResult::Pass
        } else {
            TypecheckResult::Fail
        },
        errors,
    }
}
