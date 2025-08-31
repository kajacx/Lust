use crate::grammar::LuaExpression;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct OrOperation {
    pub left: Box<LuaExpression>,
    pub right: Box<LuaExpression>,
}
