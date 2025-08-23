#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaStatement {
    Comment(LuaComment),
    VarDeclaration { name: String, value: LuaExpression },
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaComment {
    TypeAnnotation(LuaType),
    Text(String),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LuaExpression {
    StringLiteral(String),
    NumberLiteral(f64),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LuaType {
    Number,
    String,
}
