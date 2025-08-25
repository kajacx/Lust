#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum PrimitiveType {
    Nil,
    Boolean,
    Number,
    String,
}
