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
        // TODO: flatten nested unions, not needed right now since LustType::Union will do it
        Self::Union(vec![t1, t2])
    }
}
