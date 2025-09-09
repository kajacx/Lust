use crate::grammar::LuaExpression;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct EqualsOperation {
    pub left: Box<LuaExpression>,
    pub right: Box<LuaExpression>,
}

impl EqualsOperation {
    pub fn new(left: LuaExpression, right: LuaExpression) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl std::fmt::Display for EqualsOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} == {})", self.left, self.right)
    }
}
