use std::collections::HashMap;

use crate::{
    error::{ErrorLocation, LustAssignmentError, LustError, LustErrorVariant},
    grammar::{LuaExpression, LuaStatement},
    typecheck::{ErrorCollector, LustType, TypeGate},
};

pub struct Analyzer {
    pub variables: HashMap<String, LustType>,
    pub gates: Vec<TypeGate>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            gates: vec![],
        }
    }

    pub fn analyze_statements(
        &mut self,
        statements: &[LuaStatement],
        collector: &mut impl ErrorCollector,
    ) {
        for slice in statements.windows(2) {
            if let (Some(declared_type), Some(var_assignment)) = (
                slice[0].try_get_type_annotation(),
                slice[1].try_get_var_assignment(),
            ) {
                let var_name = &var_assignment.name;
                let var_type = declared_type.to_lust_type();

                let val = &var_assignment.value;
                let val_type = self.get_type(val);

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
                            expression: val.clone(),
                            expression_type: val_type,
                        }),
                    })
                }

                self.variables.insert(var_name.clone(), var_type);
            }
        }

        for statement in statements {
            match statement {
                LuaStatement::IfStatement {
                    condition,
                    then_branch,
                } => {
                    if let Some(gate) = condition.get_top_level_gate() {
                        self.gates.push(gate);
                        self.analyze_statements(then_branch, collector);
                        self.gates.pop();
                    } else {
                        self.analyze_statements(then_branch, collector);
                    }
                }
                _ => {}
            }
        }
    }

    fn get_type(&self, expr: &LuaExpression) -> LustType {
        match expr {
            LuaExpression::Nil => LustType::Nil,
            LuaExpression::BooleanLiteral(true) => LustType::True,
            LuaExpression::BooleanLiteral(false) => LustType::False,
            LuaExpression::NumberLiteral(_) => LustType::Number,
            LuaExpression::StringLiteral(_) => LustType::String,
            LuaExpression::VarName(name) => self.get_variable_type(name),
            LuaExpression::OrOperation(or_operation) => {
                let left_type = self.get_type(&or_operation.left);
                let right_type = self.get_type(&or_operation.right);

                // TODO: Abusing variable name, separate type gate from variables somehow
                let gate = TypeGate::new_truthy("".to_string(), true);
                let left_type = gate.restrict_type("", &left_type);

                LustType::new_union([left_type, right_type])
            }
        }
    }

    fn get_variable_type(&self, varname: &str) -> LustType {
        let mut var_type = self
            .variables
            .get(varname)
            .cloned()
            .unwrap_or(LustType::Any);

        for gate in &self.gates {
            var_type = gate.restrict_type(varname, &var_type);
        }

        var_type
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

    // Neither are union types, so they must be exactly equal
    what == into_what
}
