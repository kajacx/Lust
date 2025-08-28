use crate::grammar::LuaExpression;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct OrOperation {
    pub left: Box<LuaExpression>,
    pub right: Box<LuaExpression>,
}
