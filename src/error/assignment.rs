use std::fmt::Display;

use crate::grammar::LuaType;

#[derive(PartialEq, Debug)]
pub struct LustAssignmentError {
    pub var_name: String,
    pub var_type: LuaType,
    pub expression: String,
    pub expression_type: LuaType,
}

impl Display for LustAssignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot assign value '{}' of type '{:?}' into the variable '{}' of type '{:?}'.",
            self.expression, self.expression_type, self.var_name, self.var_type
        )
    }
}
