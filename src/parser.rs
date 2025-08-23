use crate::{
    grammar::{LuaStatement, Spanned},
    luasyn,
};

pub fn parse_file_content(content: &str) -> Vec<Spanned<LuaStatement>> {
    let parser = luasyn::LuaStatementsParser::new();
    let statements = match parser.parse(content) {
        Ok(value) => value,
        Err(err) => {
            panic!("Error when parsing the input file: {err:?}");
        }
    };

    statements
}
