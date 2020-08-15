use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]
pub enum AST {
    Program {
        expressions: Vec<AST>,
    },
    InfixExpr {
        left: Box<AST>,
        infix: Infix,
        right: Box<AST>,
    },
    AssignmentExpr {
        left: String,
        right: Box<AST>,
    },
    LambdaExpr {
        params: Vec<String>,
        body: Box<AST>,
    },
    IdentExpr {
        name: String,
    },
    IntLiteral {
        raw: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum Infix {
    Add,
    Sub,
    Mul,
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Sum,
    Product,
}

type ParseResult<T> = Result<T, &'static str>;

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
            Token::Sub => Precedence::Sum,
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

    pub fn parse_program(&mut self) -> ParseResult<AST> {
        let mut expressions = Vec::new();
        while self.curr_token != Token::EOF {
            let expr = self.parse_expr(Precedence::Lowest)?;
            self.next_token();
            match self.curr_token {
                Token::Semicolon => {
                    self.next_token();
                }
                Token::EOF => (),
                _ => return Err("Expected ;"),
            }
            expressions.push(expr);
        }
        Ok(AST::Program { expressions })
    }

    fn parse_expr(&mut self, precedence: Precedence) -> ParseResult<AST> {
        let mut left = match self.curr_token {
            Token::Ident(_) => match self.peek_token {
                Token::Assign => return self.parse_assignment_expression(),
                _ => self.parse_identifier_expression(),
            },
            Token::Int(_) => self.parse_integer_literal(),
            Token::Lparen => self.parse_grouped_expression(),
            Token::Vbar => self.parse_lambda_expression(),
            _ => Err("Expected Expr"),
        }?;
        while precedence < self.peek_precedence() {
            left = match self.peek_token {
                Token::Add | Token::Sub | Token::Mul => {
                    self.next_token();
                    self.parse_infix_expression(left)?
                }
                _ => return Ok(left),
            }
        }
        Ok(left)
    }

    fn parse_infix_expression(&mut self, left: AST) -> ParseResult<AST> {
        let infix = match self.curr_token {
            Token::Add => Infix::Add,
            Token::Sub => Infix::Sub,
            Token::Mul => Infix::Mul,
            _ => return Err("Expected InfixExpr"),
        };
        let precedence = self.curr_precedence();
        self.next_token();
        let right = self.parse_expr(precedence)?;
        Ok(AST::InfixExpr {
            left: Box::new(left),
            infix,
            right: Box::new(right),
        })
    }

    fn parse_assignment_expression(&mut self) -> ParseResult<AST> {
        let name = match self.curr_token {
            Token::Ident(ref name) => Ok(name.clone()),
            _ => Err("Expected AssignmentExpr"),
        }?;
        self.next_token();
        self.next_token();
        let right = self.parse_expr(Precedence::Lowest)?;
        return Ok(AST::AssignmentExpr {
            left: name.clone(),
            right: Box::new(right),
        });
    }

    fn parse_grouped_expression(&mut self) -> ParseResult<AST> {
        self.next_token();
        let expr = self.parse_expr(Precedence::Lowest);
        if self.peek_token != Token::Rparen {
            return Err("Expected )");
        }
        self.next_token();
        return expr;
    }

    fn parse_lambda_expression(&mut self) -> ParseResult<AST> {
        self.next_token();
        let mut params = vec![];
        if let Token::Ident(ref name) = self.curr_token {
            params.push(name.clone());
            self.next_token();
            while let Token::Comma = self.curr_token {
                self.next_token();
                if let Token::Ident(ref name) = self.curr_token {
                    params.push(name.clone());
                    self.next_token();
                } else {
                    return Err("Expected Ident");
                }
            }
        }
        match self.curr_token {
            Token::Vbar => self.next_token(),
            _ => return Err("Expected |"),
        }
        let body = self.parse_expr(Precedence::Lowest)?;
        Ok(AST::LambdaExpr {
            params,
            body: Box::new(body),
        })
    }

    fn parse_identifier_expression(&mut self) -> ParseResult<AST> {
        match self.curr_token {
            Token::Ident(ref name) => Ok(AST::IdentExpr { name: name.clone() }),
            _ => Err("Expected IdentExpr"),
        }
    }

    fn parse_integer_literal(&mut self) -> ParseResult<AST> {
        match self.curr_token {
            Token::Int(ref raw) => Ok(AST::IntLiteral { raw: raw.clone() }),
            _ => Err("Expected IntLiteral"),
        }
    }
}

#[test]
fn test_integer_literal() {
    assert_eq!(
        Parser::new("123").parse_program(),
        Ok(AST::Program {
            expressions: vec![AST::IntLiteral {
                raw: "123".to_string()
            }]
        })
    );
}

#[test]
fn test_infix_expression() {
    assert_eq!(
        Parser::new("1 + 2 * 3 * 4").parse_program(),
        Ok(AST::Program {
            expressions: vec![AST::InfixExpr {
                left: Box::new(AST::IntLiteral {
                    raw: "1".to_string()
                }),
                infix: Infix::Add,
                right: Box::new(AST::InfixExpr {
                    left: Box::new(AST::InfixExpr {
                        left: Box::new(AST::IntLiteral {
                            raw: "2".to_string()
                        }),
                        infix: Infix::Mul,
                        right: Box::new(AST::IntLiteral {
                            raw: "3".to_string()
                        })
                    }),
                    infix: Infix::Mul,
                    right: Box::new(AST::IntLiteral {
                        raw: "4".to_string()
                    })
                })
            }]
        })
    );
}

#[test]
fn test_lambda_expression() {
    assert_eq!(
        Parser::new("|x| x + 1").parse_program(),
        Ok(AST::Program {
            expressions: vec![AST::LambdaExpr {
                params: vec!["x".to_string()],
                body: Box::new(AST::InfixExpr {
                    left: Box::new(AST::IdentExpr {
                        name: "x".to_string()
                    }),
                    infix: Infix::Add,
                    right: Box::new(AST::IntLiteral {
                        raw: "1".to_string()
                    })
                })
            }]
        })
    );
}
