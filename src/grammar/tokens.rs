use crate::grammar::SpanIterator;

use super::Spanned;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaStatement {
    Comment(Spanned<LuaComment>),
    VarDeclaration {
        name: Spanned<String>,
        value: Spanned<LuaExpression>,
    },
}

impl LuaStatement {
    pub fn try_get_var_declaration(&self) -> Option<(&Spanned<String>, &Spanned<LuaExpression>)> {
        match self {
            Self::VarDeclaration { name, value } => Some((name, value)),
            _ => None,
        }
    }

    pub fn try_get_type_annotation(&self) -> Option<&Spanned<LuaType>> {
        match self {
            Self::Comment(comment) => match &comment.token {
                LuaComment::TypeAnnotation(annotation) => Some(annotation),
                _ => None,
            },
            _ => None,
        }
    }
}

impl SpanIterator for LuaStatement {
    fn list_spans(&mut self, mut visitor: impl FnMut(&mut super::Span)) {
        match self {
            Self::Comment(comment) => {
                visitor(&mut comment.span);
                comment.token.list_spans(visitor);
            }
            Self::VarDeclaration { name, value } => {
                visitor(&mut name.span);
                visitor(&mut value.span);
                value.token.list_spans(visitor);
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaComment {
    TypeAnnotation(Spanned<LuaType>),
    Text(Spanned<String>),
}

impl SpanIterator for LuaComment {
    fn list_spans(&mut self, mut visitor: impl FnMut(&mut super::Span)) {
        match self {
            Self::TypeAnnotation(annotation) => {
                visitor(&mut annotation.span);
                // annotation.token.list_spans(visitor); // TODO:
            }
            Self::Text(text) => visitor(&mut text.span),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaExpression {
    StringLiteral(Spanned<String>),
    NumberLiteral(Spanned<f64>),
}

impl SpanIterator for LuaExpression {
    fn list_spans(&mut self, mut visitor: impl FnMut(&mut super::Span)) {
        match self {
            Self::StringLiteral(text) => visitor(&mut text.span),
            Self::NumberLiteral(number) => visitor(&mut number.span),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LuaType {
    Number,
    String,
}
