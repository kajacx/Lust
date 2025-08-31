mod union;

pub use union::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LustType {
    Never,
    Any,
    Nil,
    True,
    False,
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

    pub fn new_union(variants: impl IntoIterator<Item = LustType>) -> Self {
        let result = UnionType::new(variants);

        // Replace the whole thing with `any` if there is at least one `any`
        if result.variants.iter().any(|v| *v == Self::Any) {
            return Self::Any;
        }

        match result.variants.len() {
            0 => Self::Never,
            1 => result.variants.into_iter().next().unwrap(),
            _ => Self::Union(result),
        }
    }

    pub fn intersect_type(&self, other: &Self) -> Self {
        if self == other {
            return self.clone();
        }

        match (self, other) {
            (Self::Any, _) => other.clone(),
            (_, Self::Any) => self.clone(),
            (Self::Never, _) | (_, Self::Never) => Self::Never,
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

#[test]
fn test_intersect_type() {
    use LustType::*;

    assert_eq!(Number.intersect_type(&Number), Number);
    assert_eq!(Number.intersect_type(&String), Never);
    assert_eq!(Any.intersect_type(&Number), Number);
    assert_eq!(Number.intersect_type(&Any), Number);
    assert_eq!(
        LustType::new_union([Number, String]).intersect_type(&String),
        String
    );
    assert_eq!(
        LustType::new_union([Number, String]).intersect_type(&LustType::new_union([String, False])),
        String
    );
    assert_eq!(
        LustType::new_union([Number, String]).intersect_type(&LustType::new_union([True, Nil])),
        Never
    );
}

#[test]
fn test_exclude_type() {
    use LustType::*;

    assert_eq!(Number.exclude_type(&Number), Never);
    assert_eq!(Number.exclude_type(&String), Number);
    assert_eq!(Any.exclude_type(&Number), Any);
    assert_eq!(
        LustType::new_union([Number, String]).exclude_type(&String),
        Number
    );
    assert_eq!(
        LustType::new_union([Number, String]).exclude_type(&LustType::new_union([String, True])),
        Number
    );
    assert_eq!(
        LustType::new_union([Number, String]).exclude_type(&LustType::new_union([Number, String])),
        Never
    );
}
