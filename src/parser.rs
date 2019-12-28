use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum AST {
    IntegerLiteral { token: Token },
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            curr_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.curr_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Result<AST, &'static str> {
        let literal = self.parse_integer_literal()?;
        if self.curr_token == Token::EOF {
            Ok(literal)
        } else {
            Err("Expected EOF")
        }
    }

    fn parse_integer_literal(&mut self) -> Result<AST, &'static str> {
        match self.curr_token {
            Token::Int(_) => {
                let literal = AST::IntegerLiteral {
                    token: self.curr_token.clone(),
                };
                self.next_token();
                Ok(literal)
            }
            _ => Err("Expected IntegerLiteral"),
        }
    }
}
