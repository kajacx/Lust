pub mod error;
pub mod grammar;
pub mod parser;
pub mod typecheck;

use crate::{
    error::LustError,
    typecheck::{TypecheckOutcome, TypecheckResult},
};

lalrpop_util::lalrpop_mod!(luasyn);
// mod luasyn {
//     include!(concat!(std::env!("OUT_DIR"), "/grammar.rs"));
// }

pub fn analyze_file(filename: &str) -> TypecheckOutcome {
    let content = std::fs::read_to_string(filename).unwrap();
    let statements = crate::parser::parse_file_content(&content);

    let mut errors: Vec<LustError> = vec![];
    let mut analyzer = typecheck::Analyzer::new();
    analyzer.analyze_statements(&statements, &mut |error| errors.push(error));

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
