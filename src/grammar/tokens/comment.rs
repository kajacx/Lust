use crate::grammar::{LuaType, Span, SpanIterator};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaComment {
    TypeAnnotation(LuaType),
    Text(String),
}

impl SpanIterator for LuaComment {
    fn list_spans(&mut self, _visitor: impl FnMut(&mut Span)) {
        match self {
            Self::TypeAnnotation(_annotation) => {
                // visitor(&mut annotation.span);
            }
            Self::Text(_) => {}
        }
    }
}
