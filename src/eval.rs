use crate::parser::{Infix, AST};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Integer { value: i64 },
}

#[derive(Debug)]
struct EvalError(&'static str);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Error for EvalError {}

pub fn eval(node: AST) -> Result<Object, Box<dyn Error>> {
    match node {
        AST::InfixExpression { left, infix, right } => eval_infix_expression(*left, infix, *right),
        AST::IntegerLiteral { raw } => eval_integer_literal(raw),
        _ => unimplemented!(),
    }
}

fn eval_infix_expression(left: AST, infix: Infix, right: AST) -> Result<Object, Box<dyn Error>> {
    let left = eval(left)?;
    let right = eval(right)?;
    if let Object::Integer { value: left_value } = left {
        if let Object::Integer { value: right_value } = right {
            Ok(match infix {
                Infix::Add => Object::Integer {
                    value: left_value + right_value,
                },
                Infix::Mul => Object::Integer {
                    value: left_value * right_value,
                },
            })
        } else {
            Err(Box::new(EvalError("type mismatch")))
        }
    } else {
        Err(Box::new(EvalError("type mismatch")))
    }
}

fn eval_integer_literal(raw: String) -> Result<Object, Box<dyn Error>> {
    Ok(Object::Integer {
        value: raw.parse()?,
    })
}
