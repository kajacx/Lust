use crate::grammar::{LuaExpression, Span, SpanIterator};

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LuaVarAssignment {
    pub span: Span,
    pub name: String,
    pub value: LuaExpression,
}

impl SpanIterator for LuaVarAssignment {
    fn list_spans(&mut self, mut visitor: impl FnMut(&mut Span)) {
        visitor(&mut self.span);
        // self.value.list_spans(visitor);
    }
}
