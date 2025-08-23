mod assignment;
mod location;
mod variant;

use std::{error::Error, fmt::Display};

pub use assignment::*;
pub use location::*;
pub use variant::*;

#[derive(PartialEq, Debug)]
pub struct LustError {
    pub location: ErrorLocation,
    pub error: LustErrorVariant,
}

impl Display for LustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.location, self.error)
    }
}

impl Error for LustError {}
