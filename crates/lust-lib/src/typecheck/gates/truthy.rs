use crate::typecheck::LustType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TruthyGate {
    pub varname: String,
    pub is_truthy: bool,
}

impl TruthyGate {
    pub fn new(varname: String, is_truthy: bool) -> Self {
        Self { varname, is_truthy }
    }

    pub fn restrict_type(&self, varname: &str, original: &LustType) -> LustType {
        if varname != self.varname {
            return original.clone();
        }

        if self.is_truthy {
            original.exclude_type(&LustType::Nil)
        } else {
            original.intersect_type(&LustType::Nil)
        }
    }
}
