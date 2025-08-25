use crate::{
    error::{ErrorLocation, LustAssignmentError, LustError, LustErrorVariant},
    grammar::{LuaExpression, LuaStatement},
    typecheck::{ErrorCollector, LustType},
};

pub fn analyze_statements(statements: &[LuaStatement], mut collector: impl ErrorCollector) {
    for slice in statements.windows(2) {
        if let (Some(declared_type), Some(var_assignment)) = (
            slice[0].try_get_type_annotation(),
            slice[1].try_get_var_assignment(),
        ) {
            let var_name = &var_assignment.name;
            let var_type = declared_type.to_lust_type();

            let val = &var_assignment.value;
            let val_type = get_type(val);
            if !can_assign(&val_type, &var_type) {
                collector.on_error(LustError {
                    location: ErrorLocation {
                        filename: "tests/variable_fail.lua".to_string(),
                        line: var_assignment.span.line,
                        column: var_assignment.span.column,
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

fn get_type(expr: &LuaExpression) -> LustType {
    match expr {
        LuaExpression::Nil => LustType::Nil,
        LuaExpression::BooleanLiteral(_) => LustType::Boolean,
        LuaExpression::NumberLiteral(_) => LustType::Number,
        LuaExpression::StringLiteral(_) => LustType::String,
    }
}

fn can_assign(what: &LustType, into_what: &LustType) -> bool {
    what == into_what
}
