use crate::{grammar::OrOperation, typecheck::TypeGate};

#[derive(PartialEq, PartialOrd, Debug)]
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

// impl SpanIterator for LuaExpression {
//     fn list_spans(&mut self, mut visitor: impl FnMut(&mut super::Span)) {
//         match self {
//             Self::StringLiteral(text) => visitor(&mut text.span),
//             Self::NumberLiteral(number) => visitor(&mut number.span),
//         }
//     }
// }
