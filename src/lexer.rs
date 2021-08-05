use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),
    String(String),
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

pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            chars: input.chars(),
        }
    }

    fn next_char(&mut self) {
        self.chars.next();
    }

    fn peek_char(&self) -> Option<char> {
        // Instead of `Peekable<CharIndices>`, I'm using `Char` directly and cloning it when I need to "peek".
        // See https://users.rust-lang.org/t/takewhile-iterator-over-chars-to-string-slice/11014
        self.chars.clone().next()
    }

    fn offset(&self) -> usize {
        self.input.len() - self.chars.as_str().len()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.peek_char() {
            Some('a'..='z' | 'A'..='Z' | '_') => {
                let start = self.offset();
                self.next_char();
                loop {
                    match self.peek_char() {
                        Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') => {
                            self.next_char();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                let end = self.offset();
                match &self.input[start..end] {
                    "def" => Token::KwDef,
                    "let" => Token::KwLet,
                    ident => Token::Ident(ident.to_string()),
                }
            }
            Some('1'..='9') => {
                let start = self.offset();
                self.next_char();
                loop {
                    match self.peek_char() {
                        Some('0'..='9') => {
                            self.next_char();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                let end = self.offset();
                return Token::Int((&self.input[start..end]).to_string());
            }
            Some('"') => {
                let start = self.offset();
                self.next_char();
                loop {
                    match self.peek_char() {
                        Some('\\') => {
                            break Token::Unexpected('\\');
                        }
                        Some('"') => {
                            self.next_char();
                            let end = self.offset();
                            break Token::String(self.input[start..end].to_string());
                        }
                        Some(_) => {
                            self.next_char();
                        }
                        None => {
                            break Token::Unexpected('"');
                        }
                    }
                }
            }
            Some('+') => {
                self.next_char();
                Token::Plus
            }
            Some('-') => {
                self.next_char();
                match self.peek_char() {
                    Some('>') => {
                        self.next_char();
                        Token::RArrow
                    }
                    _ => Token::Minus,
                }
            }
            Some('*') => {
                self.next_char();
                Token::Star
            }
            Some('/') => {
                self.next_char();
                Token::Slash
            }
            Some('%') => {
                self.next_char();
                Token::Percent
            }
            Some('^') => {
                self.next_char();
                Token::Caret
            }
            Some('!') => {
                self.next_char();
                Token::Not
            }
            Some('&') => {
                self.next_char();
                Token::And
            }
            Some('|') => {
                self.next_char();
                Token::Or
            }
            Some('=') => {
                self.next_char();
                match self.peek_char() {
                    Some('=') => {
                        self.next_char();
                        Token::EqEq
                    }
                    Some('>') => {
                        self.next_char();
                        Token::FatArrow
                    }
                    _ => Token::Eq,
                }
            }
            Some('>') => {
                self.next_char();
                match self.peek_char() {
                    Some('=') => {
                        self.next_char();
                        Token::Ge
                    }
                    _ => Token::Gt,
                }
            }
            Some('<') => {
                self.next_char();
                match self.peek_char() {
                    Some('=') => {
                        self.next_char();
                        Token::Le
                    }
                    _ => Token::Lt,
                }
            }
            Some('(') => {
                self.next_char();
                Token::LParen
            }
            Some(')') => {
                self.next_char();
                Token::RParen
            }
            Some('{') => {
                self.next_char();
                Token::LBrace
            }
            Some('}') => {
                self.next_char();
                Token::RBrace
            }
            Some('[') => {
                self.next_char();
                Token::LBracket
            }
            Some(']') => {
                self.next_char();
                Token::RBracket
            }
            Some('.') => {
                self.next_char();
                Token::Dot
            }
            Some(',') => {
                self.next_char();
                Token::Comma
            }
            Some(';') => {
                self.next_char();
                Token::Semi
            }
            Some(':') => {
                self.next_char();
                Token::Colon
            }
            Some('\n') => {
                self.next_char();
                Token::Nl
            }
            Some(ch) => {
                self.next_char();
                Token::Unexpected(ch)
            }
            None => Token::Eof,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some('\t' | ' ') => {
                    self.next_char();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let code = "12 + 34 * 56";
        let mut l = Lexer::new(code);
        assert_eq!(l.next_token(), Token::Int("12".to_string()));
        assert_eq!(l.next_token(), Token::Plus);
        assert_eq!(l.next_token(), Token::Int("34".to_string()));
        assert_eq!(l.next_token(), Token::Star);
        assert_eq!(l.next_token(), Token::Int("56".to_string()));
        assert_eq!(l.next_token(), Token::Eof);
    }
}
