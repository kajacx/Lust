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

    fn borrow(&mut self) -> impl ErrorCollector {
        |error| self.on_error(error)
    }
}

impl<T: FnMut(LustError)> ErrorCollector for T {
    fn on_error(&mut self, error: LustError) {
        self(error)
    }
}

// TODO: Conflicting implementation with &mut FnMut, refactor
// impl<T: ErrorCollector> ErrorCollector for &mut T {
//     fn on_error(&mut self, error: crate::error::LustError) {
//         (*self).on_error(error)
//     }
// }
