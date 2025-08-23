use crate::grammar::LuaStatement;

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

pub trait SpanIterator {
    fn list_spans(&mut self, visitor: impl FnMut(&mut Span));
}

impl SpanIterator for Vec<Spanned<LuaStatement>> {
    fn list_spans(&mut self, mut visitor: impl FnMut(&mut Span)) {
        for statement in self {
            visitor(&mut statement.span);
            statement.token.list_spans(&mut visitor);
        }
    }
}

pub struct LineNumbers {
    /** Stores where the first character on each line is, 0-indexed. */
    pub newline_positions: Vec<usize>,
}

impl LineNumbers {
    pub fn new(content: &str) -> Self {
        let mut positions = vec![0];

        for (i, ch) in content.char_indices() {
            if ch == '\n' {
                positions.push(i + 1);
            }
        }

        Self {
            newline_positions: positions,
        }
    }

    /**
     * Offset is 0-indexed, line and column are both 1-indexed
     */
    pub fn get_line_and_column(&self, offset: usize) -> (usize, usize) {
        // Everything inside is zero indexed
        let line = self
            .newline_positions
            .iter()
            .filter(|pos| **pos <= offset)
            .count()
            - 1;

        let column = offset - self.newline_positions[line];

        (line + 1, column + 1)
    }
}

#[test]
fn test_line_numbers() {
    let line_numbers = LineNumbers::new("Hello\nworld\n\nHowdy!");
    assert_eq!(line_numbers.get_line_and_column(0), (1, 1));
    assert_eq!(line_numbers.get_line_and_column(4), (1, 5));
    assert_eq!(line_numbers.get_line_and_column(6), (2, 1));
    assert_eq!(line_numbers.get_line_and_column(10), (2, 5));
    assert_eq!(line_numbers.get_line_and_column(13), (4, 1));
    assert_eq!(line_numbers.get_line_and_column(18), (4, 6));
    assert_eq!(line_numbers.get_line_and_column(20), (4, 8)); // predictable behavior after eof
}
