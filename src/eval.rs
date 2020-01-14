use crate::parser::{Infix, AST};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Integer { value: i64 },
    Nil,
}

#[derive(Debug)]
struct EvalError(&'static str);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Error for EvalError {}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Environment {
    store: HashMap<String, Rc<RefCell<Object>>>,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    fn get(&self, name: String) -> Option<Rc<RefCell<Object>>> {
        self.store.get(&name).map(Rc::clone)
    }

    fn set(&mut self, name: String, value: Rc<RefCell<Object>>) -> Option<Rc<RefCell<Object>>> {
        self.store.insert(name, value)
    }
}

pub struct Evaluator {
    env: Environment,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: Environment::new(),
        }
    }

    pub fn eval(&mut self, node: AST) -> Result<Rc<RefCell<Object>>, Box<dyn Error>> {
        match node {
            AST::Program { expressions } => {
                let mut last = Rc::new(RefCell::new(Object::Nil));
                for expression in expressions {
                    last = self.eval(expression)?
                }
                Ok(last)
            }
            AST::InfixExpression { left, infix, right } => {
                self.eval_infix_expression(*left, infix, *right)
            }
            AST::AssignmentExpression { left, right } => {
                self.eval_assignment_expression(left, *right)
            }
            AST::IdentifierExpression { name } => self.eval_identifier(name),
            AST::IntegerLiteral { raw } => self.eval_integer_literal(raw),
            // _ => unimplemented!(),
        }
    }

    fn eval_infix_expression(
        &mut self,
        left: AST,
        infix: Infix,
        right: AST,
    ) -> Result<Rc<RefCell<Object>>, Box<dyn Error>> {
        let left = self.eval(left)?;
        let right = self.eval(right)?;
        let left_value = if let Object::Integer { value } = *left.borrow() {
            value
        } else {
            return Err(Box::new(EvalError("type mismatch")));
        };
        let right_value = if let Object::Integer { value } = *right.borrow() {
            value
        } else {
            return Err(Box::new(EvalError("type mismatch")));
        };
        Ok(Rc::new(RefCell::new(match infix {
            Infix::Add => Object::Integer {
                value: left_value + right_value,
            },
            Infix::Sub => Object::Integer {
                value: left_value - right_value,
            },
            Infix::Mul => Object::Integer {
                value: left_value * right_value,
            },
        })))
    }

    fn eval_assignment_expression(
        &mut self,
        left: String,
        right: AST,
    ) -> Result<Rc<RefCell<Object>>, Box<dyn Error>> {
        let right = self.eval(right)?;
        self.env.set(left, Rc::clone(&right));
        Ok(right)
    }

    fn eval_identifier(&self, name: String) -> Result<Rc<RefCell<Object>>, Box<dyn Error>> {
        if let Some(value) = self.env.get(name) {
            Ok(value)
        } else {
            Err(Box::new(EvalError("identifier not found")))
        }
    }

    fn eval_integer_literal(&self, raw: String) -> Result<Rc<RefCell<Object>>, Box<dyn Error>> {
        Ok(Rc::new(RefCell::new(Object::Integer {
            value: raw.parse()?,
        })))
    }
}
