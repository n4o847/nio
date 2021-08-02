use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Program { expressions: Vec<Expr> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    BinOp {
        left: Box<Expr>,
        infix: Infix,
        right: Box<Expr>,
    },
    Assign {
        left: String,
        right: Box<Expr>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Ident(String),
    IntLit(String),
    StringLit(String),
}

#[derive(Debug, PartialEq, Clone)]
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
    Call,
}

type ParseResult<T> = Result<T, &'static str>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    token: Token,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token();
        Parser { lexer, token }
    }

    fn next_token(&mut self) {
        self.token = self.lexer.next_token();
    }

    fn peek_token(&mut self) -> &Token {
        &self.token
    }

    fn token_to_precedence(token: &Token) -> Precedence {
        match token {
            Token::Add => Precedence::Sum,
            Token::Sub => Precedence::Sum,
            Token::Mul => Precedence::Product,
            Token::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn peek_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.token)
    }

    pub fn parse(input: &str) -> ParseResult<AST> {
        let mut p = Self::new(input);
        p.parse_program()
    }

    pub fn parse_program(&mut self) -> ParseResult<AST> {
        let mut expressions = Vec::new();
        while *self.peek_token() != Token::EOF {
            let expr = self.parse_expr(Precedence::Lowest)?;
            self.next_token();
            match self.peek_token() {
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

    fn parse_expr(&mut self, precedence: Precedence) -> ParseResult<Expr> {
        let mut expr = match self.peek_token() {
            Token::Ident(name) => {
                let name = name.clone();
                self.next_token();
                match self.peek_token() {
                    Token::Assign => {
                        self.next_token();
                        let right = self.parse_expr(Precedence::Lowest)?;
                        Ok(Expr::Assign {
                            left: name,
                            right: Box::new(right),
                        })
                    }
                    _ => Ok(Expr::Ident(name)),
                }
            }
            Token::Int(raw) => {
                let raw = raw.clone();
                self.next_token();
                Ok(Expr::IntLit(raw))
            }
            Token::String(raw) => {
                let raw = raw.clone();
                self.next_token();
                Ok(Expr::StringLit(raw))
            }
            Token::Lparen => {
                self.next_token();
                let expr = self.parse_expr(Precedence::Lowest)?;
                match self.peek_token() {
                    Token::Rparen => self.next_token(),
                    _ => return Err("Expected )"),
                }
                Ok(expr)
            }
            Token::Vbar => {
                self.next_token();
                let mut params = Vec::new();
                if let Token::Ident(name) = self.peek_token() {
                    let name = name.clone();
                    self.next_token();
                    params.push(name);
                    while let Token::Comma = self.peek_token() {
                        self.next_token();
                        if let Token::Ident(name) = self.peek_token() {
                            let name = name.clone();
                            self.next_token();
                            params.push(name);
                        } else {
                            return Err("Expected Ident");
                        }
                    }
                }
                match self.peek_token() {
                    Token::Vbar => self.next_token(),
                    _ => return Err("Expected |"),
                }
                let body = self.parse_expr(Precedence::Lowest)?;
                Ok(Expr::Lambda {
                    params,
                    body: Box::new(body),
                })
            }
            _ => Err("Expected Expr"),
        }?;
        while precedence < self.peek_precedence() {
            expr = match self.peek_token() {
                token @ (Token::Add | Token::Sub | Token::Mul) => {
                    let infix = match token {
                        Token::Add => Infix::Add,
                        Token::Sub => Infix::Sub,
                        Token::Mul => Infix::Mul,
                        _ => unreachable!(),
                    };
                    let precedence = self.peek_precedence();
                    self.next_token();
                    let right = self.parse_expr(precedence)?;
                    Ok(Expr::BinOp {
                        left: Box::new(expr),
                        infix,
                        right: Box::new(right),
                    })
                }
                Token::Lparen => {
                    self.next_token();
                    match self.peek_token() {
                        Token::Rparen => {
                            self.next_token();
                            Ok(Expr::Call {
                                callee: Box::new(expr),
                                args: Vec::new(),
                            })
                        }
                        _ => {
                            let mut args = Vec::new();
                            args.push(self.parse_expr(Precedence::Lowest)?);
                            while *self.peek_token() == Token::Comma {
                                self.next_token();
                                args.push(self.parse_expr(Precedence::Lowest)?);
                            }
                            if *self.peek_token() == Token::Rparen {
                                self.next_token();
                                Ok(Expr::Call {
                                    callee: Box::new(expr),
                                    args,
                                })
                            } else {
                                Err("Expected , or )")
                            }
                        }
                    }
                }
                _ => Ok(expr),
            }?;
        }
        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use Expr::*;
        use Infix::*;
        use AST::*;

        assert_eq!(
            Parser::parse("123"),
            Ok(Program {
                expressions: vec![IntLit("123".to_string())]
            })
        );

        assert_eq!(
            Parser::parse("1 + 2 * 3 * 4"),
            Ok(Program {
                expressions: vec![BinOp {
                    left: Box::new(IntLit("1".to_string())),
                    infix: Add,
                    right: Box::new(BinOp {
                        left: Box::new(BinOp {
                            left: Box::new(IntLit("2".to_string())),
                            infix: Mul,
                            right: Box::new(IntLit("3".to_string()))
                        }),
                        infix: Mul,
                        right: Box::new(IntLit("4".to_string()))
                    })
                }]
            })
        );

        assert_eq!(
            Parser::parse("|x| x + 1"),
            Ok(Program {
                expressions: vec![Lambda {
                    params: vec!["x".to_string()],
                    body: Box::new(BinOp {
                        left: Box::new(Ident("x".to_string())),
                        infix: Add,
                        right: Box::new(IntLit("1".to_string()))
                    })
                }]
            })
        );

        assert_eq!(
            Parser::parse("a + b(x, y)"),
            Ok(Program {
                expressions: vec![BinOp {
                    left: Box::new(Ident("a".to_string())),
                    infix: Add,
                    right: Box::new(Call {
                        callee: Box::new(Ident("b".to_string())),
                        args: vec![Ident("x".to_string()), Ident("y".to_string()),]
                    })
                }]
            })
        );
    }
}
