pub mod ast;
pub mod lexer;
pub mod token;

use lalrpop_util::lalrpop_mod;
use thiserror::Error;

lalrpop_mod!(pub grammar);

pub type Location = usize;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    Lalrpop(lalrpop_util::ParseError<Location, String, lexer::LexicalError>),
}

pub fn parse(input: &str) -> Result<ast::Program, ParseError> {
    let lexer = lexer::Lexer::new(input);
    grammar::ProgramParser::new()
        .parse(lexer)
        .map_err(|err| ParseError::Lalrpop(parse_error_to_string(err)))
}

fn parse_error_to_string(
    err: lalrpop_util::ParseError<Location, token::Token, lexer::LexicalError>,
) -> lalrpop_util::ParseError<Location, String, lexer::LexicalError> {
    match err {
        lalrpop_util::ParseError::InvalidToken { location } => {
            lalrpop_util::ParseError::InvalidToken { location }
        }
        lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
            lalrpop_util::ParseError::UnrecognizedEof { location, expected }
        }
        lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
            lalrpop_util::ParseError::UnrecognizedToken {
                token: token_to_string(token),
                expected,
            }
        }
        lalrpop_util::ParseError::ExtraToken { token } => lalrpop_util::ParseError::ExtraToken {
            token: token_to_string(token),
        },
        lalrpop_util::ParseError::User { error } => lalrpop_util::ParseError::User { error },
    }
}

fn token_to_string(
    (start, token, end): (Location, token::Token, Location),
) -> (Location, String, Location) {
    (start, token.to_string(), end)
}
