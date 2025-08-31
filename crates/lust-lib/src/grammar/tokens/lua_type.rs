use crate::typecheck::{LustType, UnionType};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LuaType {
    Any,
    Nil,
    Boolean,
    Number,
    String,
    Union(Vec<LuaType>),
}

impl LuaType {
    pub fn to_lust_type(&self) -> LustType {
        match self {
            Self::Any => LustType::Any,
            Self::Nil => LustType::Nil,
            Self::Boolean => LustType::new_union([LustType::True, LustType::False]),
            Self::Number => LustType::Number,
            Self::String => LustType::String,
            Self::Union(variants) => {
                let mapped = variants.iter().map(|variant| variant.to_lust_type());
                LustType::Union(UnionType::new(mapped))
            }
        }
    }

    pub fn new_union(t1: LuaType, t2: LuaType) -> Self {
        // No need to flatten nested unions, LustType::Union will do it
        Self::Union(vec![t1, t2])
    }
}

impl std::fmt::Display for LuaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Nil => write!(f, "nil"),
            Self::Boolean => write!(f, "boolean"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Union(variants) => {
                let variant_strs = variants
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>();
                write!(f, "{}", variant_strs.join(" | "))
            }
        }
    }
}
