use crate::{grammar::LuaExpression, typecheck::TypeGate};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct AndOperation {
    pub left: Box<LuaExpression>,
    pub right: Box<LuaExpression>,
}

impl AndOperation {
    pub fn get_type_gate(&self) -> Option<TypeGate> {
        let left_gate = self.left.get_type_gate();
        let right_gate = self.right.get_type_gate();

        match (left_gate, right_gate) {
            (None, None) => None,
            (Some(gate), None) | (None, Some(gate)) => Some(gate),
            (Some(left), Some(right)) => Some(TypeGate::new_and([left, right])),
        }
    }
}
