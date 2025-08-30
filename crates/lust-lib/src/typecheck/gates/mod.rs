use crate::typecheck::LustType;

mod truthy;

pub use truthy::*;

pub enum TypeGate {
    Truthy(TruthyGate),
}

impl TypeGate {
    pub fn restrict_type(&self, varname: &str, original: &LustType) -> LustType {
        match self {
            TypeGate::Truthy(gate) => gate.restrict_type(varname, original),
        }
    }

    pub fn new_truthy(varname: String, truthy: bool) -> Self {
        Self::Truthy(TruthyGate::new(varname, truthy))
    }
}
