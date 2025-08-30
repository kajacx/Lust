mod union;

pub use union::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LustType {
    Never,
    Any,
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

    pub fn new_union(variants: impl Iterator<Item = LustType>) -> Self {
        let result = UnionType::new(variants);
        if result.variants.len() == 1 {
            result.variants.into_iter().next().unwrap()
        } else {
            Self::Union(result)
        }
    }

    pub fn intersect_type(&self, other: &Self) -> Self {
        if self == other {
            return self.clone();
        }

        match (self, other) {
            (Self::Any, _) => other.clone(),
            (_, Self::Any) => self.clone(),
            (Self::Union(u1), Self::Union(u2)) => {
                let variants = u1
                    .variants
                    .iter()
                    .filter(|t1| u2.variants.iter().any(|t2| *t1 == t2))
                    .cloned();
                Self::new_union(variants)
            }
            (Self::Union(u), t) | (t, Self::Union(u)) => {
                if u.variants.iter().any(|variant| variant == t) {
                    t.clone()
                } else {
                    Self::Never
                }
            }
            _ => Self::Never,
        }
    }

    pub fn exclude_type(&self, other: &Self) -> Self {
        if self == other {
            return Self::Never;
        }

        match (self, other) {
            (Self::Any, _) => Self::Any,
            (_, Self::Any) => self.clone(),
            (Self::Union(u1), Self::Union(u2)) => {
                let variants = u1
                    .variants
                    .iter()
                    .filter(|t1| !u2.variants.iter().any(|t2| *t1 == t2))
                    .cloned();
                Self::new_union(variants)
            }
            (Self::Union(u), t) => {
                let variants = u.variants.iter().filter(|variant| *variant != t).cloned();
                Self::new_union(variants)
            }
            _ => self.clone(),
        }
    }
}
