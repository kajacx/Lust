use std::fmt::Display;

use crate::{grammar::LuaExpression, typecheck::LustType};

#[derive(PartialEq, Debug)]
pub struct LustAssignmentError {
    pub var_name: String,
    pub var_type: LustType,
    pub expression: LuaExpression,
    pub expression_type: LustType,
}

impl Display for LustAssignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot assign value {} of type {} into the variable {} of type {}.",
            self.expression, self.expression_type, self.var_name, self.var_type
        )
    }
}
