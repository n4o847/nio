#[macro_use]
extern crate lalrpop_util;

pub mod ast;
lalrpop_mod!(pub grammar);
pub mod lexer;
pub mod token;

use lalrpop_util::ParseError;

pub type Location = usize;
pub type Error = &'static str;

pub fn parse(input: &str) -> Result<ast::Program, ParseError<Location, token::Token, Error>> {
    let lexer = lexer::Lexer::new(input);
    grammar::ProgramParser::new().parse(lexer)
}
