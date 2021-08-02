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
            Token::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn curr_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.curr_token)
    }

    fn peek_precedence(&mut self) -> Precedence {
        Parser::token_to_precedence(&self.peek_token)
    }

    pub fn parse(input: &str) -> ParseResult<AST> {
        let mut p = Self::new(input);
        p.parse_program()
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

    fn parse_expr(&mut self, precedence: Precedence) -> ParseResult<Expr> {
        let mut left = match self.curr_token {
            Token::Ident(_) => match self.peek_token {
                Token::Assign => return self.parse_expr_assign(),
                _ => self.parse_expr_ident(),
            },
            Token::Int(_) => self.parse_expr_int_lit(),
            Token::String(_) => self.parse_expr_string_lit(),
            Token::Lparen => self.parse_expr_grouped(),
            Token::Vbar => self.parse_expr_lambda(),
            _ => Err("Expected Expr"),
        }?;
        while precedence < self.peek_precedence() {
            left = match self.peek_token {
                Token::Add | Token::Sub | Token::Mul => {
                    self.next_token();
                    self.parse_expr_bin_op(left)?
                }
                Token::Lparen => {
                    self.next_token();
                    self.parse_expr_call(left)?
                }
                _ => return Ok(left),
            }
        }
        Ok(left)
    }

    fn parse_expr_bin_op(&mut self, left: Expr) -> ParseResult<Expr> {
        let infix = match self.curr_token {
            Token::Add => Infix::Add,
            Token::Sub => Infix::Sub,
            Token::Mul => Infix::Mul,
            _ => return Err("Expected BinOp"),
        };
        let precedence = self.curr_precedence();
        self.next_token();
        let right = self.parse_expr(precedence)?;
        Ok(Expr::BinOp {
            left: Box::new(left),
            infix,
            right: Box::new(right),
        })
    }

    fn parse_expr_assign(&mut self) -> ParseResult<Expr> {
        let name = match self.curr_token {
            Token::Ident(ref name) => Ok(name.clone()),
            _ => Err("Expected Assign"),
        }?;
        self.next_token();
        self.next_token();
        let right = self.parse_expr(Precedence::Lowest)?;
        Ok(Expr::Assign {
            left: name.clone(),
            right: Box::new(right),
        })
    }

    fn parse_expr_grouped(&mut self) -> ParseResult<Expr> {
        self.next_token();
        let expr = self.parse_expr(Precedence::Lowest);
        if self.peek_token != Token::Rparen {
            return Err("Expected )");
        }
        self.next_token();
        expr
    }

    fn parse_expr_lambda(&mut self) -> ParseResult<Expr> {
        self.next_token();
        let mut params = Vec::new();
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
        Ok(Expr::Lambda {
            params,
            body: Box::new(body),
        })
    }

    fn parse_expr_call(&mut self, callee: Expr) -> ParseResult<Expr> {
        if self.peek_token == Token::Rparen {
            self.next_token();
            return Ok(Expr::Call {
                callee: Box::new(callee),
                args: Vec::new(),
            });
        }
        self.next_token();
        let mut args = Vec::new();
        args.push(self.parse_expr(Precedence::Lowest)?);
        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            args.push(self.parse_expr(Precedence::Lowest)?);
        }
        if self.peek_token == Token::Rparen {
            self.next_token();
            Ok(Expr::Call {
                callee: Box::new(callee),
                args,
            })
        } else {
            Err("Expected , or )")
        }
    }

    fn parse_expr_ident(&mut self) -> ParseResult<Expr> {
        match self.curr_token {
            Token::Ident(ref name) => Ok(Expr::Ident(name.clone())),
            _ => Err("Expected Ident"),
        }
    }

    fn parse_expr_int_lit(&mut self) -> ParseResult<Expr> {
        match self.curr_token {
            Token::Int(ref raw) => Ok(Expr::IntLit(raw.clone())),
            _ => Err("Expected IntLit"),
        }
    }

    fn parse_expr_string_lit(&mut self) -> ParseResult<Expr> {
        match self.curr_token {
            Token::String(ref raw) => Ok(Expr::StringLit(raw.clone())),
            _ => Err("Expected StringLit"),
        }
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
            Ok(AST::Program {
                expressions: vec![Lambda {
                    params: vec!["x".to_string()],
                    body: Box::new(BinOp {
                        left: Box::new(Ident("x".to_string())),
                        infix: Infix::Add,
                        right: Box::new(IntLit("1".to_string()))
                    })
                }]
            })
        );

        assert_eq!(
            Parser::parse("a + b(x, y)"),
            Ok(AST::Program {
                expressions: vec![BinOp {
                    left: Box::new(Ident("a".to_string())),
                    infix: Infix::Add,
                    right: Box::new(Call {
                        callee: Box::new(Ident("b".to_string())),
                        args: vec![Ident("x".to_string()), Ident("y".to_string()),]
                    })
                }]
            })
        );
    }
}
