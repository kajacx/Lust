use crate::{
    error::{ErrorLocation, LustAssignmentError, LustError, LustErrorVariant},
    grammar::{LuaExpression, LuaStatement, LuaType, Spanned},
    typecheck::ErrorCollector,
};

pub fn analyze_statements(
    statements: &[Spanned<LuaStatement>],
    mut collector: impl ErrorCollector,
) {
    for slice in statements.windows(2) {
        if let (Some(declared_type), Some(var_assignment)) = (
            slice[0].token.try_get_type_annotation(),
            slice[1].token.try_get_var_declaration(),
        ) {
            let var_type = &declared_type.token;
            let var_name = &var_assignment.0.token;

            let val = &var_assignment.1.token;
            let val_type = get_type(val);
            if !can_assign(&val_type, var_type) {
                collector.on_error(LustError {
                    location: ErrorLocation {
                        filename: "tests/variable_fail.lua".to_string(),
                        line: var_assignment.0.span.line,
                        column: var_assignment.0.span.column,
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

fn get_type(expr: &LuaExpression) -> LuaType {
    match expr {
        LuaExpression::NumberLiteral(_) => LuaType::Number,
        LuaExpression::StringLiteral(_) => LuaType::String,
    }
}

fn can_assign(what: &LuaType, into_what: &LuaType) -> bool {
    what == into_what
}
