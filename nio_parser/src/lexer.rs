use std::str::Chars;

use thiserror::Error;

use crate::Location;
use crate::token::Token;

#[derive(Error, Debug, PartialEq)]
pub enum LexicalError {
    #[error("Unterminated string literal")]
    UnterminatedStringLiteral,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
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

    pub fn offset(&self) -> usize {
        self.input.len() - self.chars.as_str().len()
    }

    pub fn next_token(&mut self) -> Option<Result<(Location, Token<'a>, Location), LexicalError>> {
        self.skip_whitespace();
        let start = self.offset();
        let token = match self.peek_char() {
            Some('a'..='z' | 'A'..='Z' | '_') => {
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
                    ident => Token::Ident(ident),
                }
            }
            Some('1'..='9') => {
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
                Token::Int(&self.input[start..end])
            }
            Some('"') => {
                self.next_char();
                let mut value = String::new();
                loop {
                    match self.peek_char() {
                        Some('\\') => {
                            todo!("Character escapes are not yet supported.");
                        }
                        Some('"') => {
                            self.next_char();
                            let end = self.offset();
                            break Token::String {
                                raw: &self.input[start..end],
                                value,
                            };
                        }
                        Some(ch) => {
                            value.push(ch);
                            self.next_char();
                        }
                        None => {
                            return Some(Err(LexicalError::UnterminatedStringLiteral));
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
            Some('@') => {
                self.next_char();
                Token::At
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
            None => return None,
        };
        let end = self.offset();
        Some(Ok((start, token, end)))
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

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(Location, Token<'a>, Location), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let code = "12 + 34 * 56";
        let mut l = Lexer::new(code);
        assert_eq!(l.next_token(), Some(Ok((0, Token::Int("12"), 2))));
        assert_eq!(l.next_token(), Some(Ok((3, Token::Plus, 4))));
        assert_eq!(l.next_token(), Some(Ok((5, Token::Int("34"), 7))));
        assert_eq!(l.next_token(), Some(Ok((8, Token::Star, 9))));
        assert_eq!(l.next_token(), Some(Ok((10, Token::Int("56"), 12))));
        assert_eq!(l.next_token(), None);
    }
}
