use crate::ir::*;
use std::{error, fmt};

#[derive(Debug)]
pub struct TypeError;

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-")
    }
}

impl error::Error for TypeError {}

pub fn typecheck(program: &mut Program) -> Result<(), TypeError> {
    TypeChecker::new().typecheck_program(program)
}

struct TypeChecker;

impl TypeChecker {
    fn new() -> Self {
        Self
    }

    fn resolve_type(&self, type_: &mut Type) -> Result<(), TypeError> {
        match type_ {
            Type::Unresolved(name) => match name.as_str() {
                "Int" => {
                    *type_ = Type::Int;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn typecheck_program(&self, program: &mut Program) -> Result<(), TypeError> {
        for stmt in program.statements.iter_mut() {
            self.typecheck_stmt(stmt)?;
        }
        Ok(())
    }

    fn typecheck_stmt(&self, stmt: &mut Stmt) -> Result<(), TypeError> {
        match stmt {
            Stmt::Def {
                annotations: _,
                name: _,
                params,
                return_type,
                body,
            } => {
                for (_, param_type) in params.iter_mut() {
                    self.resolve_type(param_type)?;
                }
                self.resolve_type(return_type)?;
                self.typecheck_expr(body)?;
            }
            Stmt::Expr(expr) => {
                self.typecheck_expr(expr)?;
            }
        }
        Ok(())
    }

    fn typecheck_expr(&self, expr: &mut Expr) -> Result<(), TypeError> {
        match expr {
            Expr::BinOp { op: _, lhs, rhs } => {
                self.typecheck_expr(lhs)?;
                self.typecheck_expr(rhs)?;
            }
            Expr::IntLit(_) => {}
            _ => todo!(),
        }
        Ok(())
    }
}
