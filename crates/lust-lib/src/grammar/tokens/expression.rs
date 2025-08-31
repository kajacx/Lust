use crate::{grammar::OrOperation, typecheck::TypeGate};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LuaExpression {
    Nil,
    BooleanLiteral(bool),
    NumberLiteral(f64),
    StringLiteral(String),
    VarName(String),
    OrOperation(OrOperation),
}

impl LuaExpression {
    pub fn new_or(left: LuaExpression, right: LuaExpression) -> Self {
        Self::OrOperation(OrOperation {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn get_top_level_gate(&self) -> Option<TypeGate> {
        match self {
            LuaExpression::VarName(name) => Some(TypeGate::new_truthy(name.clone(), true)),
            _ => None,
        }
    }
}

impl std::fmt::Display for LuaExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::BooleanLiteral(true) => write!(f, "true"),
            Self::BooleanLiteral(false) => write!(f, "false"),
            Self::NumberLiteral(number) => write!(f, "{}", number),
            Self::StringLiteral(text) => write!(f, "\"{}\"", text),
            Self::VarName(name) => write!(f, "{}", name),
            Self::OrOperation(op) => write!(f, "({} or {})", op.left, op.right),
        }
    }
}

// impl SpanIterator for LuaExpression {
//     fn list_spans(&mut self, mut visitor: impl FnMut(&mut super::Span)) {
//         match self {
//             Self::StringLiteral(text) => visitor(&mut text.span),
//             Self::NumberLiteral(number) => visitor(&mut number.span),
//         }
//     }
// }
