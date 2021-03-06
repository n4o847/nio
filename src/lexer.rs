use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),
    String(String),
    Add,
    Sub,
    Mul,
    Assign,
    Vbar,
    Rarrow,
    Lparen,
    Rparen,
    Comma,
    Semicolon,
    EOF,
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

    fn read_char(&mut self) -> Option<(usize, char)> {
        self.char_indices.next()
    }

    fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.char_indices.peek()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if let Some((pos, ch)) = self.read_char() {
            match ch {
                'a'..='z' => {
                    while let Some(&(pos_end, ch)) = self.peek_char() {
                        match ch {
                            'a'..='z' | '0'..='9' => {
                                self.read_char();
                            }
                            _ => return Token::Ident(self.input[pos..pos_end].to_string()),
                        }
                    }
                    return Token::Ident((&self.input[pos..]).to_string());
                }
                '1'..='9' => {
                    while let Some(&(pos_end, ch)) = self.peek_char() {
                        match ch {
                            '0'..='9' => {
                                self.read_char();
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
                                self.read_char();
                                return Token::String(
                                    self.input[pos..pos_end + ch.len_utf8()].to_string(),
                                );
                            }
                            _ => {
                                self.read_char();
                            }
                        }
                    }
                    return Token::Unexpected('"');
                }
                '+' => Token::Add,
                '-' => {
                    if let Some(&(_, ch)) = self.peek_char() {
                        match ch {
                            '>' => {
                                self.read_char();
                                Token::Rarrow
                            }
                            _ => Token::Sub,
                        }
                    } else {
                        Token::Sub
                    }
                }
                '*' => Token::Mul,
                '=' => Token::Assign,
                '|' => Token::Vbar,
                '(' => Token::Lparen,
                ')' => Token::Rparen,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                _ => Token::Unexpected(ch),
            }
        } else {
            Token::EOF
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_pos, ch)) = self.peek_char() {
            match ch {
                '\t' | ' ' => {
                    self.read_char();
                }
                _ => return,
            }
        }
    }
}

#[test]
fn test_next_token() {
    let code = "12 + 34 * 56";
    let mut l = Lexer::new(code);
    assert_eq!(l.next_token(), Token::Int("12".to_string()));
    assert_eq!(l.next_token(), Token::Add);
    assert_eq!(l.next_token(), Token::Int("34".to_string()));
    assert_eq!(l.next_token(), Token::Mul);
    assert_eq!(l.next_token(), Token::Int("56".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}
