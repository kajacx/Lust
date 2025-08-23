use crate::error::LustError;

#[derive(PartialEq, Debug)]
pub struct TypecheckOutcome {
    pub result: TypecheckResult,
    pub errors: Vec<LustError>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TypecheckResult {
    Pass,
    Warn,
    Fail,
    Syntax,
}

pub trait ErrorCollector {
    fn on_error(&mut self, error: LustError);
}

impl<T: FnMut(LustError)> ErrorCollector for T {
    fn on_error(&mut self, error: LustError) {
        self(error)
    }
}
