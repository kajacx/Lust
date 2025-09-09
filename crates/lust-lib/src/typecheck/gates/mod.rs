use crate::typecheck::LustType;

mod and;
mod single;

pub use and::*;
pub use single::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TypeGate {
    Single(SingleGate),
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
        let falsy_types = [LustType::Nil, LustType::False];
        if truthy {
            Self::Single(SingleGate::new_exclude(varname, falsy_types))
        } else {
            Self::Single(SingleGate::new_intersect(varname, falsy_types))
        }
    }

    pub fn new_single(varname: String, types: impl IntoIterator<Item = LustType>) -> Self {
        Self::Single(SingleGate::new_intersect(varname, types))
    }

    pub fn new_and(gates: impl IntoIterator<Item = TypeGate>) -> Self {
        Self::And(AndGate::new(gates))
    }

    pub fn get_restrictions(&self) -> &[GateRestriction] {
        match self {
            TypeGate::Single(gate) => gate.get_restrictions(),
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
