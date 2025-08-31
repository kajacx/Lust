use super::LustType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UnionType {
    pub variants: Vec<LustType>,
}

impl UnionType {
    /** This will:
     * 1) Flatten nested unions
     * 2) Remove duplicate variants
     * 3) Remove the `Never`
     *
     * This will NOT:
     * 1) Remove other variants if there is an `Any`
     * 2) Order the variants in any way
     */
    pub fn new(variants: impl IntoIterator<Item = LustType>) -> Self {
        let mut flattened_variants = vec![];

        for variant in variants {
            Self::add_variant(&mut flattened_variants, variant);
        }

        Self {
            variants: flattened_variants,
        }
    }

    pub fn add_variant(flattened: &mut Vec<LustType>, v: LustType) {
        if v == LustType::Never {
            return;
        }

        match v.try_into_union_variants() {
            Ok(variants) => {
                for nested in variants {
                    Self::add_variant(flattened, nested);
                }
            }
            Err(other) => {
                if !flattened.contains(&other) {
                    flattened.push(other);
                }
            }
        }
    }

    pub fn contains(&self, t: &LustType) -> bool {
        self.variants.iter().any(|variant| variant == t)
    }
}
