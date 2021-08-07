use crate::ast::*;
use crate::lexer::{Lexer, Token};

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
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Star => Precedence::Product,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn peek_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.token)
    }

    pub fn parse(input: &str) -> ParseResult<Program> {
        let mut p = Self::new(input);
        p.parse_program()
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut statements = Vec::new();
        loop {
            match self.peek_token() {
                Token::Nl => {
                    self.next_token();
                }
                Token::Eof => {
                    break;
                }
                _ => {
                    let stmt = self.parse_stmt()?;
                    statements.push(stmt);
                }
            }
        }
        Ok(Program { statements })
    }

    fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        let stmt = match self.peek_token() {
            Token::KwDef => {
                self.next_token();
                let name = self.parse_ident()?;
                match self.peek_token() {
                    Token::LParen => {
                        self.next_token();
                        Ok(())
                    }
                    _ => Err("Expected ("),
                }?;
                let mut params = vec![];
                loop {
                    let param_name = match self.peek_token() {
                        Token::Ident(param_name) => {
                            let param_name = param_name.clone();
                            self.next_token();
                            param_name
                        }
                        Token::RParen => {
                            self.next_token();
                            break;
                        }
                        _ => return Err("Expected Ident or )"),
                    };
                    match self.peek_token() {
                        Token::Colon => {
                            self.next_token();
                        }
                        _ => return Err("Expected :"),
                    }
                    let param_type = match self.peek_token() {
                        Token::Ident(param_type) => {
                            let param_type = param_type.clone();
                            self.next_token();
                            param_type
                        }
                        _ => return Err("Expected Ident"),
                    };
                    params.push((param_name, param_type));
                    match self.peek_token() {
                        Token::Comma => {
                            self.next_token();
                        }
                        Token::RParen => {
                            self.next_token();
                            break;
                        }
                        _ => return Err("Expected , or )"),
                    }
                }
                match self.peek_token() {
                    Token::Colon => {
                        self.next_token();
                    }
                    _ => return Err("Expected :"),
                }
                let return_type = match self.peek_token() {
                    Token::Ident(return_type) => {
                        let return_type = return_type.clone();
                        self.next_token();
                        return_type
                    }
                    _ => return Err("Expected Ident"),
                };
                match self.peek_token() {
                    Token::Eq => {
                        self.next_token();
                    }
                    _ => return Err("Expected ="),
                }
                let body = self.parse_expr(Precedence::Lowest)?;
                Stmt::Def {
                    name,
                    params,
                    return_type,
                    body: Box::new(body),
                }
            }
            Token::KwLet => {
                self.next_token();
                todo!();
            }
            _ => {
                let expr = self.parse_expr(Precedence::Lowest)?;
                Stmt::Expr(expr)
            }
        };
        match self.peek_token() {
            Token::Semi | Token::Nl => {
                self.next_token();
            }
            Token::Eof => {}
            _ => {
                return Err("Expected ; or newline");
            }
        }
        Ok(stmt)
    }

    fn parse_expr(&mut self, precedence: Precedence) -> ParseResult<Expr> {
        let mut expr = match self.peek_token() {
            Token::Ident(name) => {
                let name = name.clone();
                self.next_token();
                match self.peek_token() {
                    Token::Eq => {
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
            Token::LParen => {
                self.next_token();
                let expr = self.parse_expr(Precedence::Lowest)?;
                match self.peek_token() {
                    Token::RParen => self.next_token(),
                    _ => return Err("Expected )"),
                }
                Ok(expr)
            }
            Token::Or => {
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
                    Token::Or => self.next_token(),
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
                token @ (Token::Plus | Token::Minus | Token::Star) => {
                    let op = match token {
                        Token::Plus => BinOp::Add,
                        Token::Minus => BinOp::Sub,
                        Token::Star => BinOp::Mul,
                        _ => unreachable!(),
                    };
                    let precedence = self.peek_precedence();
                    self.next_token();
                    let right = self.parse_expr(precedence)?;
                    Ok(Expr::BinOp {
                        op,
                        left: Box::new(expr),
                        right: Box::new(right),
                    })
                }
                Token::LParen => {
                    self.next_token();
                    match self.peek_token() {
                        Token::RParen => {
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
                            if *self.peek_token() == Token::RParen {
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

    fn parse_ident(&mut self) -> ParseResult<String> {
        match self.peek_token() {
            Token::Ident(name) => {
                let name = name.clone();
                self.next_token();
                Ok(name)
            }
            _ => Err("Expected Ident"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use self::BinOp::*;
        use self::Expr::*;
        use self::Program;
        use self::Stmt::*;

        assert_eq!(
            Parser::parse("123"),
            Ok(Program {
                statements: vec![Expr(IntLit("123".to_string()))]
            })
        );

        assert_eq!(
            Parser::parse("1 + 2 * 3 * 4"),
            Ok(Program {
                statements: vec![Expr(BinOp {
                    op: Add,
                    left: Box::new(IntLit("1".to_string())),
                    right: Box::new(BinOp {
                        op: Mul,
                        left: Box::new(BinOp {
                            op: Mul,
                            left: Box::new(IntLit("2".to_string())),
                            right: Box::new(IntLit("3".to_string()))
                        }),
                        right: Box::new(IntLit("4".to_string()))
                    })
                })]
            })
        );

        assert_eq!(
            Parser::parse("|x| x + 1"),
            Ok(Program {
                statements: vec![Expr(Lambda {
                    params: vec!["x".to_string()],
                    body: Box::new(BinOp {
                        op: Add,
                        left: Box::new(Ident("x".to_string())),
                        right: Box::new(IntLit("1".to_string()))
                    })
                })]
            })
        );

        assert_eq!(
            Parser::parse("a + b(x, y)"),
            Ok(Program {
                statements: vec![Expr(BinOp {
                    op: Add,
                    left: Box::new(Ident("a".to_string())),
                    right: Box::new(Call {
                        callee: Box::new(Ident("b".to_string())),
                        args: vec![Ident("x".to_string()), Ident("y".to_string()),]
                    })
                })]
            })
        );
    }
}
