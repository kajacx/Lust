use crate::grammar::{LuaComment, LuaExpression, LuaType, LuaVarAssignment, Span, SpanIterator};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaStatement {
    Comment(LuaComment),
    VarAssignment(LuaVarAssignment),
    IfStatement {
        condition: LuaExpression,
        then_branch: Vec<LuaStatement>,
    },
}

impl LuaStatement {
    pub fn new_var_assignment(span: Span, name: String, value: LuaExpression) -> Self {
        Self::VarAssignment(LuaVarAssignment { span, name, value })
    }

    pub fn new_if_statement(condition: LuaExpression, then_branch: Vec<LuaStatement>) -> Self {
        Self::IfStatement {
            condition: condition,
            then_branch: then_branch,
        }
    }

    pub fn try_get_var_assignment(&self) -> Option<&LuaVarAssignment> {
        match self {
            Self::VarAssignment(assignment) => Some(assignment),
            _ => None,
        }
    }

    pub fn try_get_type_annotation(&self) -> Option<&LuaType> {
        match self {
            Self::Comment(comment) => match comment {
                LuaComment::TypeAnnotation(annotation) => Some(annotation),
                _ => None,
            },
            _ => None,
        }
    }
}

impl SpanIterator for LuaStatement {
    fn list_spans(&mut self, mut visitor: impl FnMut(&mut Span)) {
        match self {
            Self::Comment(_comment) => {
                // visitor(&mut comment.span);
                // comment.token.list_spans(visitor);
            }
            Self::VarAssignment(assignment) => {
                visitor(&mut assignment.span);
            }
            Self::IfStatement {
                condition: _condition,
                then_branch: _then_branch,
            } => {
                // condition.list_spans(&mut visitor);
                // then_branch.list_spans(&mut visitor);
            }
        }
    }
}
