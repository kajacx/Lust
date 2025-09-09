use crate::{
    grammar::LuaExpression,
    typecheck::{LustType, TypeGate},
};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct EqualsOperation {
    pub left: Box<LuaExpression>,
    pub right: Box<LuaExpression>,
}

impl EqualsOperation {
    pub fn new(left: LuaExpression, right: LuaExpression) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn get_type_gate(&self) -> Option<TypeGate> {
        // panic!("Hellxo? {:?}", self);
        // type(varname) == "string"
        match (&*self.left, &*self.right) {
            (LuaExpression::FunctionCall(call), LuaExpression::StringLiteral(value))
            | (LuaExpression::StringLiteral(value), LuaExpression::FunctionCall(call)) => {
                let var_name = if call.function_name == "type" && call.arguments.len() == 1 {
                    if let LuaExpression::VarName(name) = &call.arguments[0] {
                        name
                    } else {
                        return None;
                    }
                } else {
                    return None;
                };

                let lust_type = runtime_string_to_type(value)?;
                Some(TypeGate::new_single(var_name.clone(), [lust_type.clone()]))
            }
            _ => None,
        }
    }
}

fn runtime_string_to_type(value: &str) -> Option<LustType> {
    match value {
        "nil" => Some(LustType::Nil),
        "boolean" => Some(LustType::boolean()),
        "number" => Some(LustType::Number),
        "string" => Some(LustType::String),
        _ => None,
    }
}

impl std::fmt::Display for EqualsOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} == {})", self.left, self.right)
    }
}
