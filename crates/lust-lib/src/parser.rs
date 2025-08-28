use crate::{
    grammar::{LineNumbers, LuaStatement, SpanIterator},
    luasyn,
};

pub fn parse_file_content(content: &str) -> Vec<LuaStatement> {
    let parser = luasyn::LuaStatementsParser::new();
    let mut statements = match parser.parse(content) {
        Ok(value) => value,
        Err(err) => {
            panic!("TODO: Error when parsing the input file: {err:?}");
        }
    };

    let line_numbers = LineNumbers::new(content);
    for statement in &mut statements {
        statement.list_spans(|span| {
            let (line, column) = line_numbers.get_line_and_column(span.offset);
            span.line = line;
            span.column = column;
        });
    }

    statements
}
