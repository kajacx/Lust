use super::LustType;

#[derive(Debug, Clone)]
pub struct UnionType {
    pub variants: Vec<LustType>,
}

impl UnionType {
    pub fn new(variants: impl Iterator<Item = LustType>) -> Self {
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
}
