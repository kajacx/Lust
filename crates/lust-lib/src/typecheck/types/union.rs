use super::LustType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UnionType {
    pub variants: Vec<LustType>,
}

impl UnionType {
    pub fn new(variants: impl Iterator<Item = LustType>) -> Self {
        // TODO: do not insert duplicate variants
        let mut flattened_variants = vec![];
        for variant in variants {
            match variant.try_into_union_variants() {
                Ok(mut variants) => flattened_variants.append(&mut variants),
                Err(other) => flattened_variants.push(other),
            }
        }
        Self {
            variants: flattened_variants,
        }
    }

    pub fn contains(&self, t: &LustType) -> bool {
        self.variants.iter().any(|variant| variant == t)
    }
}
