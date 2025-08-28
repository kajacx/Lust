#[cfg(test)]
mod tests;

use lust_lib::{analyze_file, typecheck::TypecheckResult};

pub fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    let outcome = analyze_file(&path);

    for error in outcome.errors {
        eprintln!("{error}");
    }

    if outcome.result == TypecheckResult::Pass || outcome.result == TypecheckResult::Warn {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
