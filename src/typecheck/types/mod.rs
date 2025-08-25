mod union;

pub use union::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LustType {
    Nil,
    Boolean,
    Number,
    String,
    Union(UnionType),
}

impl LustType {
    pub fn try_get_union_variants(&self) -> Option<&[LustType]> {
        match self {
            Self::Union(union_type) => Some(&union_type.variants),
            _ => None,
        }
    }

    pub fn try_into_union_variants(self) -> Result<Vec<LustType>, Self> {
        match self {
            Self::Union(union_type) => Ok(union_type.variants),
            _ => Err(self),
        }
    }
}
