use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct ErrorLocation {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}

impl Display for ErrorLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}
