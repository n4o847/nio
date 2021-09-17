use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Ident(&'a str),
    Int(&'a str),
    String(&'a str),
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Percent,  // %
    Caret,    // ^
    Not,      // !
    And,      // &
    Or,       // |
    Eq,       // =
    EqEq,     // ==
    Gt,       // >
    Lt,       // <
    Ge,       // >=
    Le,       // <=
    RArrow,   // ->
    FatArrow, // =>
    LParen,   // (
    RParen,   // )
    LBrace,   // [
    RBrace,   // ]
    LBracket, // {
    RBracket, // }
    Dot,      // .
    Comma,    // ,
    Semi,     // ;
    Colon,    // :
    Nl,       // newline
    Eof,      // end-of-file
    KwDef,    // def
    KwLet,    // let
    Unexpected(char),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
