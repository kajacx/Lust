use crate::grammar::LuaExpression;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<LuaExpression>,
}

impl FunctionCall {
    pub fn new(function_name: String, argument: LuaExpression) -> Self {
        Self {
            function_name,
            arguments: vec![argument],
        }
    }
}

impl std::fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self
            .arguments
            .iter()
            .map(|arg| format!("{}", arg))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}({})", self.function_name, args)
    }
}

// impl SpanIterator for FunctionCall {
//     fn list_spans(&mut self, mut visitor: impl FnMut(&mut Span)) {
//         for arg in &mut self.arguments {
//             arg.list_spans(&mut visitor);
//         }
//     }
// }
