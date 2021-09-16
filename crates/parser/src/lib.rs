#[macro_use]
extern crate lalrpop_util;

pub mod ast;
lalrpop_mod!(pub grammar);
pub mod lexer;

use lalrpop_util::ParseError;

pub type Location = ();
pub type Error = &'static str;

pub fn parse(input: &str) -> Result<ast::Program, ParseError<Location, lexer::Token, Error>> {
    let lexer = lexer::Lexer::new(input);
    grammar::ProgramParser::new().parse(input, lexer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use ast::BinOp::*;
        use ast::Expr::*;
        use ast::Program;
        use ast::Stmt::*;

        assert_eq!(
            parse("123"),
            Ok(Program {
                statements: vec![Expr(IntLit("123".to_string()))]
            })
        );

        assert_eq!(
            parse("1 + 2 * 3 * 4"),
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

        // assert_eq!(
        //     parse("|x| x + 1"),
        //     Ok(Program {
        //         statements: vec![Expr(Lambda {
        //             params: vec!["x".to_string()],
        //             body: Box::new(BinOp {
        //                 op: Add,
        //                 left: Box::new(Ident("x".to_string())),
        //                 right: Box::new(IntLit("1".to_string()))
        //             })
        //         })]
        //     })
        // );

        // assert_eq!(
        //     parse("a + b(x, y)"),
        //     Ok(Program {
        //         statements: vec![Expr(BinOp {
        //             op: Add,
        //             left: Box::new(Ident("a".to_string())),
        //             right: Box::new(Call {
        //                 callee: Box::new(Ident("b".to_string())),
        //                 args: vec![Ident("x".to_string()), Ident("y".to_string()),]
        //             })
        //         })]
        //     })
        // );
    }
}
