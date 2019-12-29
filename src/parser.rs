use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]
pub enum AST {
    InfixExpression {
        left: Box<AST>,
        infix: Infix,
        right: Box<AST>,
    },
    IntegerLiteral {
        token: Token,
    },
}

#[derive(Debug, PartialEq)]
pub enum Infix {
    Add,
    Mul,
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Sum,
    Product,
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

    fn token_to_precedence(token: &Token) -> Precedence {
        match token {
            Token::Add => Precedence::Sum,
            Token::Mul => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn curr_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.curr_token)
    }

    fn peek_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.peek_token)
    }

    pub fn parse_program(&mut self) -> Result<AST, &'static str> {
        let literal = self.parse_expression(Precedence::Lowest)?;
        self.next_token();
        if self.curr_token == Token::EOF {
            Ok(literal)
        } else {
            Err("Expected EOF")
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<AST, &'static str> {
        let mut left = match self.curr_token {
            Token::Int(_) => self.parse_integer_literal(),
            _ => Err("Expected Expression"),
        }?;
        while self.peek_token != Token::EOF && precedence < self.peek_precedence() {
            left = match self.peek_token {
                Token::Add | Token::Mul => {
                    self.next_token();
                    self.parse_infix_expression(left)?
                }
                _ => return Ok(left),
            }
        }
        Ok(left)
    }

    fn parse_infix_expression(&mut self, left: AST) -> Result<AST, &'static str> {
        let infix = match self.curr_token {
            Token::Add => Infix::Add,
            Token::Mul => Infix::Mul,
            _ => return Err("Expected InfixExpression"),
        };
        let precedence = self.curr_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(AST::InfixExpression {
            left: Box::new(left),
            infix,
            right: Box::new(right),
        })
    }

    fn parse_integer_literal(&mut self) -> Result<AST, &'static str> {
        match self.curr_token {
            Token::Int(_) => {
                let literal = AST::IntegerLiteral {
                    token: self.curr_token.clone(),
                };
                Ok(literal)
            }
            _ => Err("Expected IntegerLiteral"),
        }
    }
}

#[test]
fn test_integer_literal() {
    assert_eq!(
        Parser::new("123").parse_program(),
        Ok(AST::IntegerLiteral {
            token: Token::Int("123".to_string())
        })
    );
}

#[test]
fn test_infix_expression() {
    assert_eq!(
        Parser::new("1 + 2 * 3 * 4").parse_program(),
        Ok(AST::InfixExpression {
            left: Box::new(AST::IntegerLiteral {
                token: Token::Int("1".to_string())
            }),
            infix: Infix::Add,
            right: Box::new(AST::InfixExpression {
                left: Box::new(AST::InfixExpression {
                    left: Box::new(AST::IntegerLiteral {
                        token: Token::Int("2".to_string())
                    }),
                    infix: Infix::Mul,
                    right: Box::new(AST::IntegerLiteral {
                        token: Token::Int("3".to_string())
                    })
                }),
                infix: Infix::Mul,
                right: Box::new(AST::IntegerLiteral {
                    token: Token::Int("4".to_string())
                })
            })
        })
    );
}
