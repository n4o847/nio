use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),
    String(String),
    Plus,   // +
    Minus,  // -
    Star,   // *
    Eq,     // =
    Or,     // |
    RArrow, // ->
    LParen, // (
    RParen, // )
    Comma,  // ,
    Semi,   // ;
    Eof,    // end-of-file
    Unexpected(char),
}

pub struct Lexer<'a> {
    input: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            char_indices: input.char_indices().peekable(),
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        self.char_indices.next()
    }

    fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.char_indices.peek()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if let Some((pos, ch)) = self.next_char() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = (|| {
                        while let Some(&(pos_end, ch)) = self.peek_char() {
                            match ch {
                                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                    self.next_char();
                                }
                                _ => {
                                    return &self.input[pos..pos_end];
                                }
                            }
                        }
                        return &self.input[pos..];
                    })();
                    return Token::Ident(ident.to_string());
                }
                '1'..='9' => {
                    while let Some(&(pos_end, ch)) = self.peek_char() {
                        match ch {
                            '0'..='9' => {
                                self.next_char();
                            }
                            _ => return Token::Int(self.input[pos..pos_end].to_string()),
                        }
                    }
                    return Token::Int((&self.input[pos..]).to_string());
                }
                '"' => {
                    while let Some(&(pos_end, ch)) = self.peek_char() {
                        match ch {
                            '\\' => {
                                return Token::Unexpected('\\');
                            }
                            '"' => {
                                self.next_char();
                                return Token::String(
                                    self.input[pos..pos_end + ch.len_utf8()].to_string(),
                                );
                            }
                            _ => {
                                self.next_char();
                            }
                        }
                    }
                    return Token::Unexpected('"');
                }
                '+' => Token::Plus,
                '-' => {
                    if let Some(&(_, ch)) = self.peek_char() {
                        match ch {
                            '>' => {
                                self.next_char();
                                Token::RArrow
                            }
                            _ => Token::Minus,
                        }
                    } else {
                        Token::Minus
                    }
                }
                '*' => Token::Star,
                '=' => Token::Eq,
                '|' => Token::Or,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                ';' => Token::Semi,
                _ => Token::Unexpected(ch),
            }
        } else {
            Token::Eof
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_pos, ch)) = self.peek_char() {
            match ch {
                '\t' | ' ' => {
                    self.next_char();
                }
                _ => return,
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
