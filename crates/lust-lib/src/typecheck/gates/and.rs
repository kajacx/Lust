use crate::typecheck::{GateRestriction, TypeGate};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AndGate {
    pub gates: Vec<TypeGate>,
    pub restrictions: Vec<GateRestriction>,
}

impl AndGate {
    pub fn new(gates: impl IntoIterator<Item = TypeGate>) -> Self {
        let gates = gates.into_iter().collect::<Vec<_>>();
        let restrictions = gates
            .iter()
            .map(|g| g.get_restrictions())
            .flatten()
            .cloned()
            .collect();

        Self {
            gates,
            restrictions,
        }
    }

    pub fn get_restrictions(&self) -> &[GateRestriction] {
        &self.restrictions
    }
}
