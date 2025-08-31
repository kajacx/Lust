use crate::typecheck::{GateRestriction, GateRestrictionKind, LustType};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TruthyGate {
    pub restriction: [GateRestriction; 1],
}

impl TruthyGate {
    pub fn new(varname: String, is_truthy: bool) -> Self {
        let restriction = GateRestriction {
            varname,
            kind: if is_truthy {
                GateRestrictionKind::Exclude
            } else {
                GateRestrictionKind::Intersect
            },
            types: vec![LustType::Nil, LustType::False],
        };
        Self {
            restriction: [restriction],
        }
    }

    pub fn get_restrictions(&self) -> &[GateRestriction] {
        &self.restriction
    }
}
