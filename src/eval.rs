use crate::parser::{Infix, AST};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Integer {
        value: i64,
    },
    Nil,
    Lambda {
        params: Vec<String>,
        body: AST,
        env: Rc<RefCell<Environment>>,
    },
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

pub struct Environment {
    store: HashMap<String, Rc<RefCell<Object>>>,
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Env")?;
        Ok(())
    }
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

type EvalResult = Result<Rc<RefCell<Object>>, Box<dyn Error>>;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn eval(&mut self, node: AST) -> EvalResult {
        match node {
            AST::Program { expressions } => {
                let mut last = Rc::new(RefCell::new(Object::Nil));
                for expr in expressions {
                    last = self.eval(expr)?
                }
                Ok(last)
            }
            AST::InfixExpr { left, infix, right } => self.eval_infix_expr(*left, infix, *right),
            AST::AssignmentExpr { left, right } => self.eval_assignment_expr(left, *right),
            AST::LambdaExpr { params, body } => self.eval_lambda_expr(params, *body),
            AST::IdentExpr { name } => self.eval_ident(name),
            AST::IntLiteral { raw } => self.eval_int_literal(raw),
            // _ => unimplemented!(),
        }
    }

    fn eval_infix_expr(&mut self, left: AST, infix: Infix, right: AST) -> EvalResult {
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

    fn eval_assignment_expr(&mut self, left: String, right: AST) -> EvalResult {
        let right = self.eval(right)?;
        self.env.borrow_mut().set(left, Rc::clone(&right));
        Ok(right)
    }

    fn eval_lambda_expr(&mut self, params: Vec<String>, body: AST) -> EvalResult {
        Ok(Rc::new(RefCell::new(Object::Lambda {
            params,
            body,
            env: Rc::clone(&self.env),
        })))
    }

    fn eval_ident(&self, name: String) -> EvalResult {
        if let Some(value) = self.env.borrow().get(name) {
            Ok(value)
        } else {
            Err(Box::new(EvalError("identifier not found")))
        }
    }

    fn eval_int_literal(&self, raw: String) -> EvalResult {
        Ok(Rc::new(RefCell::new(Object::Integer {
            value: raw.parse()?,
        })))
    }
}
