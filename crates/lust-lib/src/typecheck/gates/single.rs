use crate::typecheck::{GateRestriction, GateRestrictionKind, LustType};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SingleGate {
    pub restriction: [GateRestriction; 1],
}

impl SingleGate {
    pub fn new_intersect(varname: String, types: impl IntoIterator<Item = LustType>) -> Self {
        let restriction = GateRestriction {
            varname,
            kind: GateRestrictionKind::Intersect,
            types: types.into_iter().collect(),
        };
        Self {
            restriction: [restriction],
        }
    }

    pub fn new_exclude(varname: String, types: impl IntoIterator<Item = LustType>) -> Self {
        let restriction = GateRestriction {
            varname,
            kind: GateRestrictionKind::Exclude,
            types: types.into_iter().collect(),
        };
        Self {
            restriction: [restriction],
        }
    }

    pub fn get_restrictions(&self) -> &[GateRestriction] {
        &self.restriction
    }
}
