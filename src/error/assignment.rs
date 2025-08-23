use std::fmt::Display;

use crate::LustType;

#[derive(Debug)]
pub struct LustAssignmentError {
    var_name: String,
    var_type: LustType,
    expression: String,
    expression_type: LustType,
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
