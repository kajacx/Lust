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
                        filename: String::new(), // Will be filled in later
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
        LuaExpression::VarName(_) => todo!("Variable type"),
        LuaExpression::OrOperation(or_operation) => {
            let left_type = get_type(&or_operation.left);
            let right_type = get_type(&or_operation.right);
            LustType::new_union([left_type, right_type].into_iter())
        }
    }
}

fn can_assign(what: &LustType, into_what: &LustType) -> bool {
    // "Any" can be assigned to/from anything
    if (what == &LustType::Any) || (into_what == &LustType::Any) {
        return true;
    }

    // "What" is a union type, all variants must be assignable to "into_what"
    if let Some(what_variants) = what.try_get_union_variants() {
        return what_variants
            .iter()
            .all(|what_variant| can_assign(what_variant, into_what));
    }

    // "Into what" is a union type, at least one variant must be assignable to from "what"
    if let Some(into_what_variants) = into_what.try_get_union_variants() {
        return into_what_variants
            .iter()
            .any(|variant| can_assign(what, variant));
    }

    // Neither are union types, so they must be exactly equal TODO: arrays and tables
    what == into_what
}
