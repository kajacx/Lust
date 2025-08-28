use std::fmt::Display;

pub use super::*;

#[derive(PartialEq, Debug)]
pub enum LustErrorVariant {
    Assignment(LustAssignmentError),
}

impl Display for LustErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assignment(assignment) => write!(f, "{assignment}"),
        }
    }
}
