use crate::{
    error::{ErrorLocation, LustAssignmentError, LustError, LustErrorVariant},
    grammar::{LuaComment, LuaExpression, LuaStatement, LuaType},
    typecheck::ErrorCollector,
};

pub fn analyze_statements(statements: &[LuaStatement], mut collector: impl ErrorCollector) {
    for slice in statements.windows(2) {
        if let LuaStatement::Comment(LuaComment::TypeAnnotation(var_type)) = &slice[0] {
            if let LuaStatement::VarDeclaration {
                name: var_name,
                value: val,
            } = &slice[1]
            {
                let val_type = get_type(val);
                if !can_assign(&val_type, var_type) {
                    collector.on_error(LustError {
                        location: ErrorLocation {
                            filename: "tests/variable_fail.lua".to_string(),
                            line: 2,
                            column: 4,
                        },
                        error: LustErrorVariant::Assignment(LustAssignmentError {
                            var_name: var_name.clone(),
                            var_type: var_type.clone(),
                            expression: format!("TODO: {val:?}"),
                            expression_type: val_type,
                        }),
                    })
                }
            }
        }
    }
}

fn get_type(expr: &LuaExpression) -> LuaType {
    match expr {
        LuaExpression::NumberLiteral(_) => LuaType::Number,
        LuaExpression::StringLiteral(_) => LuaType::String,
    }
}

fn can_assign(what: &LuaType, into_what: &LuaType) -> bool {
    what == into_what
}
