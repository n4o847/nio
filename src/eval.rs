use crate::parser::{Infix, AST};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Int {
        value: i64,
    },
    String {
        value: Vec<char>,
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
    outer: Option<Rc<RefCell<Environment>>>,
    store: HashMap<String, Rc<RefCell<Object>>>,
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Env")?;
        Ok(())
    }
}

impl Environment {
    fn new() -> Self {
        Environment {
            outer: None,
            store: HashMap::new(),
        }
    }

    fn new_from(outer: Rc<RefCell<Self>>) -> Self {
        Environment {
            outer: Some(outer),
            store: HashMap::new(),
        }
    }

    fn get(&self, name: &String) -> Option<Rc<RefCell<Object>>> {
        if let Some(value) = self.store.get(name) {
            Some(Rc::clone(value))
        } else if let Some(env) = self.outer.as_ref() {
            env.borrow().get(name)
        } else {
            None
        }
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

    pub fn eval(&mut self, node: &AST) -> EvalResult {
        match node {
            AST::Program { expressions } => {
                let mut last = Rc::new(RefCell::new(Object::Nil));
                for expr in expressions {
                    last = self.eval(expr)?
                }
                Ok(last)
            }
            AST::InfixExpr { left, infix, right } => self.eval_infix_expr(left, infix, right),
            AST::AssignmentExpr { left, right } => self.eval_assignment_expr(left, right),
            AST::LambdaExpr { params, body } => self.eval_lambda_expr(params, body),
            AST::CallExpr { callee, args } => self.eval_call_expr(callee, args),
            AST::IdentExpr { name } => self.eval_ident(name),
            AST::IntLiteral { raw } => self.eval_int_literal(raw),
            AST::StringLiteral { raw } => self.eval_string_literal(raw),
            // _ => unimplemented!(),
        }
    }

    fn eval_infix_expr(&mut self, left: &AST, infix: &Infix, right: &AST) -> EvalResult {
        let left = self.eval(left)?;
        let right = self.eval(right)?;
        let left_value = if let Object::Int { value } = *left.borrow() {
            value
        } else {
            return Err(Box::new(EvalError("type mismatch")));
        };
        let right_value = if let Object::Int { value } = *right.borrow() {
            value
        } else {
            return Err(Box::new(EvalError("type mismatch")));
        };
        Ok(Rc::new(RefCell::new(match infix {
            Infix::Add => Object::Int {
                value: left_value + right_value,
            },
            Infix::Sub => Object::Int {
                value: left_value - right_value,
            },
            Infix::Mul => Object::Int {
                value: left_value * right_value,
            },
        })))
    }

    fn eval_assignment_expr(&mut self, left: &String, right: &AST) -> EvalResult {
        let right = self.eval(right)?;
        self.env.borrow_mut().set(left.clone(), Rc::clone(&right));
        Ok(right)
    }

    fn eval_lambda_expr(&mut self, params: &Vec<String>, body: &AST) -> EvalResult {
        Ok(Rc::new(RefCell::new(Object::Lambda {
            params: params.clone(),
            body: body.clone(),
            env: Rc::clone(&self.env),
        })))
    }

    fn eval_call_expr(&mut self, callee: &AST, args: &Vec<AST>) -> EvalResult {
        let callee = self.eval(callee)?;
        match &*Rc::clone(&callee).borrow() {
            Object::Lambda { params, body, env } => {
                if params.len() != args.len() {
                    return Err(Box::new(EvalError("arguments error")));
                }
                let mut env = Environment::new_from(Rc::clone(&env));
                for i in 0..params.len() {
                    let arg = self.eval(&args[i])?;
                    env.set(params[i].to_string(), arg);
                }
                Self {
                    env: Rc::new(RefCell::new(env)),
                }
                .eval(&body)
            }
            _ => Err(Box::new(EvalError("callee is not a function"))),
        }
    }

    fn eval_ident(&self, name: &String) -> EvalResult {
        if let Some(value) = self.env.borrow().get(name) {
            Ok(value)
        } else {
            Err(Box::new(EvalError("identifier not found")))
        }
    }

    fn eval_int_literal(&self, raw: &String) -> EvalResult {
        Ok(Rc::new(RefCell::new(Object::Int {
            value: raw.parse()?,
        })))
    }

    fn eval_string_literal(&self, raw: &String) -> EvalResult {
        let mut value = vec![];
        let mut chars = raw.chars();
        match chars.next() {
            Some('"') => loop {
                match chars.next() {
                    Some('\\') => {
                        unimplemented!();
                    }
                    Some('"') => {
                        break;
                    }
                    Some(ch) => {
                        value.push(ch);
                    }
                    None => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
        Ok(Rc::new(RefCell::new(Object::String { value })))
    }
}
