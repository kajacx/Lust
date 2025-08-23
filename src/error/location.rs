use std::fmt::Display;

#[derive(Debug)]
pub struct ErrorLocation {
    filename: String,
    line: usize,
    column: usize,
}

impl Display for ErrorLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}
