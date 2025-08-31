use crate::typecheck::LustType;

mod and;
mod truthy;

pub use and::*;
pub use truthy::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TypeGate {
    Truthy(TruthyGate),
    And(AndGate),
}

impl TypeGate {
    pub fn restrict_type(&self, varname: &str, original: &LustType) -> LustType {
        let restrictions = self.get_restrictions();
        let mut result = original.clone();
        for restriction in restrictions.iter().filter(|r| r.varname == varname) {
            match restriction.kind {
                GateRestrictionKind::Intersect => {
                    for t in &restriction.types {
                        result = result.intersect_type(t);
                    }
                }
                GateRestrictionKind::Exclude => {
                    for t in &restriction.types {
                        result = result.exclude_type(t);
                    }
                }
            }
        }
        return result;
    }

    pub fn new_truthy(varname: String, truthy: bool) -> Self {
        Self::Truthy(TruthyGate::new(varname, truthy))
    }

    pub fn new_and(gates: impl IntoIterator<Item = TypeGate>) -> Self {
        Self::And(AndGate::new(gates))
    }

    pub fn get_restrictions(&self) -> &[GateRestriction] {
        match self {
            TypeGate::Truthy(gate) => gate.get_restrictions(),
            TypeGate::And(gate) => gate.get_restrictions(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GateRestriction {
    pub varname: String,
    pub kind: GateRestrictionKind,
    pub types: Vec<LustType>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub enum GateRestrictionKind {
    Intersect,
    Exclude,
}
