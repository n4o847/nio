#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Int(&'a str),
    EOF,
}

use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'a> {
    input: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input,
            char_indices: input.char_indices().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<(usize, char)> {
        self.char_indices.next()
    }

    fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.char_indices.peek()
    }

    pub fn next_token(&mut self) -> Result<Token, &str> {
        if let Some((pos, ch)) = self.read_char() {
            match ch {
                '1'..='9' => {
                    while let Some(&(pos_end, ch)) = self.peek_char() {
                        if !('0'..='9').contains(&ch) {
                            return Ok(Token::Int(&self.input[pos..pos_end]));
                        } else {
                            self.read_char();
                        }
                    }
                    return Ok(Token::Int(&self.input[pos..]));
                }
                _ => Err("Unexpected token"),
            }
        } else {
            Ok(Token::EOF)
        }
    }
}

#[test]
fn test_next_token() {
    let code = "123";
    let mut l = Lexer::new(code);
    assert_eq!(l.next_token(), Ok(Token::Int("123")));
    assert_eq!(l.next_token(), Ok(Token::EOF));
}
