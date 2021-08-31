#![allow(dead_code)]

use crate::ast;
use crate::wasm::syntax as wasm;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct CodeGenerator {
    func_type_map: HashMap<wasm::FuncType, usize>,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            func_type_map: HashMap::new(),
        }
    }

    pub fn generate(program: &ast::Program) -> Result<wasm::Module> {
        let g = Self::new();
        let mut module = wasm::Module::new();
        g.generate_program(program, &mut module)?;
        Ok(module)
    }

    fn generate_program(&self, program: &ast::Program, module: &mut wasm::Module) -> Result<()> {
        let r#type = wasm::FuncType(wasm::ResultType(vec![]), wasm::ResultType(vec![]));
        let type_idx = wasm::TypeIdx(module.types.len() as u32);
        module.types.push(r#type);
        let locals = vec![];
        let mut instructions = vec![];
        for stmt in program.statements.iter() {
            self.generate_stmt(stmt, module, &mut instructions)?;
        }
        let func_idx = wasm::FuncIdx(module.funcs.len() as u32);
        module.funcs.push(wasm::Func {
            r#type: type_idx,
            locals,
            body: wasm::Expr(instructions),
        });
        module.exports.push(wasm::Export {
            name: wasm::Name("_start".to_string()),
            desc: wasm::ExportDesc::Func(func_idx),
        });
        Ok(())
    }

    fn generate_stmt(
        &self,
        stmt: &ast::Stmt,
        module: &mut wasm::Module,
        instructions: &mut Vec<wasm::Instr>,
    ) -> Result<()> {
        match stmt {
            ast::Stmt::Def {
                name: _name,
                params,
                return_type,
                body,
            } => {
                let mut r#type = wasm::FuncType(wasm::ResultType(vec![]), wasm::ResultType(vec![]));
                for (_param_name, param_type) in params.iter() {
                    match param_type.as_str() {
                        "Int" => r#type.0 .0.push(wasm::ValType::I32),
                        _ => todo!(),
                    }
                }
                match return_type.as_str() {
                    "Int" => r#type.1 .0.push(wasm::ValType::I32),
                    _ => todo!(),
                }
                let type_idx = wasm::TypeIdx(module.types.len() as u32);
                module.types.push(r#type);
                let locals = vec![];
                let mut instructions = vec![];
                self.generate_expr(body, &mut instructions)?;
                module.funcs.push(wasm::Func {
                    r#type: type_idx,
                    locals,
                    body: wasm::Expr(instructions),
                });
                Ok(())
            }
            ast::Stmt::Expr(expr) => {
                self.generate_expr(expr, instructions)?;
                Ok(())
            }
        }
    }

    fn generate_expr(&self, expr: &ast::Expr, instructions: &mut Vec<wasm::Instr>) -> Result<()> {
        match expr {
            ast::Expr::BinOp { op, left, right } => {
                self.generate_expr(left, instructions)?;
                self.generate_expr(right, instructions)?;
                match op {
                    ast::BinOp::Add => {
                        instructions.push(wasm::Instr::I32Add);
                    }
                    ast::BinOp::Sub => {
                        instructions.push(wasm::Instr::I32Sub);
                    }
                    ast::BinOp::Mul => {
                        instructions.push(wasm::Instr::I32Mul);
                    }
                }
                Ok(())
            }
            ast::Expr::IntLit(raw) => {
                let value = raw.parse::<i32>()?;
                instructions.push(wasm::Instr::I32Const(value as u32));
                Ok(())
            }
            _ => todo!(),
        }
    }
}
