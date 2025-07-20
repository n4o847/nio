pub mod ast;
pub mod lexer;
pub mod token;

use lalrpop_util::{ParseError, lalrpop_mod};

lalrpop_mod!(pub grammar);

pub type Location = usize;
pub type Error = &'static str;

pub fn parse(input: &str) -> Result<ast::Program, ParseError<Location, token::Token, Error>> {
    let lexer = lexer::Lexer::new(input);
    grammar::ProgramParser::new().parse(lexer)
}
