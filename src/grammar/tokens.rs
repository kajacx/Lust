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

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaComment {
    TypeAnnotation(Spanned<LuaType>),
    Text(Spanned<String>),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaExpression {
    StringLiteral(Spanned<String>),
    NumberLiteral(Spanned<f64>),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LuaType {
    Number,
    String,
}
