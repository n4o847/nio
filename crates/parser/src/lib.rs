#[macro_use]
extern crate lalrpop_util;

pub mod ast;
lalrpop_mod!(pub grammar);
pub mod lexer;

use lalrpop_util::ParseError;

pub type Location = ();
pub type Error = &'static str;

pub fn parse(input: &str) -> Result<ast::Program, ParseError<Location, lexer::Token, Error>> {
    let lexer = lexer::Lexer::new(input);
    grammar::ProgramParser::new().parse(input, lexer)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse {
        ($test_name:ident, $name:expr) => {
            #[test]
            fn $test_name() {
                let input = include_str!(concat!("../tests/", $name, ".in"));
                let output = include_str!(concat!("../tests/", $name, ".out"));
                let result = &format!("{:#?}", parse(input));
                assert_eq!(result, output, "expected: {}", result);
            }
        };
    }

    test_parse!(test_parse_empty, "empty");
    test_parse!(test_parse_int_lit, "int_lit");
    test_parse!(test_parse_expr, "expr");
    test_parse!(test_parse_lambda, "lambda");
    test_parse!(test_parse_call, "call");
}
