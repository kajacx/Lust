use crate::grammar::OrOperation;

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
}

// impl SpanIterator for LuaExpression {
//     fn list_spans(&mut self, mut visitor: impl FnMut(&mut super::Span)) {
//         match self {
//             Self::StringLiteral(text) => visitor(&mut text.span),
//             Self::NumberLiteral(number) => visitor(&mut number.span),
//         }
//     }
// }
