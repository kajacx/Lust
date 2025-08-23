#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Spanned<T> {
    pub token: T,
    pub span: Span,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Span {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl<T> Spanned<T> {
    pub fn new(token: T, offset: usize) -> Self {
        Self {
            token,
            span: Span {
                offset,
                line: 0,
                column: 0,
            },
        }
    }
}
