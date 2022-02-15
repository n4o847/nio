#![allow(dead_code)]

use crate::ir;
use crate::wasm;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Context<'a> {
    locals: Vec<(&'a String, &'a ir::Type)>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Self { locals: Vec::new() }
    }
}

pub struct CodeGenerator {
    func_type_map: HashMap<wasm::FuncType, usize>,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            func_type_map: HashMap::new(),
        }
    }

    pub fn generate(program: &ir::Program) -> Result<wasm::Module> {
        let g = Self::new();
        let mut module = wasm::Module::new();
        g.generate_program(program, &mut module)?;
        Ok(module)
    }

    fn generate_program(&self, program: &ir::Program, module: &mut wasm::Module) -> Result<()> {
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
        stmt: &ir::Stmt,
        module: &mut wasm::Module,
        instructions: &mut Vec<wasm::Instr>,
    ) -> Result<()> {
        match stmt {
            ir::Stmt::Def {
                annotations,
                name: _name,
                params,
                return_type,
                body,
            } => {
                match annotations.len() {
                    0 => {}
                    1 => {
                        let annot = &annotations[0];
                        match annot {
                            ir::Expr::Call { callee, args } => {
                                match (callee.as_ref(), args.as_slice()) {
                                    (ir::Expr::Ident(name), [ir::Expr::StringLit(export_name)])
                                        if name == "export" =>
                                    {
                                        let func_idx = wasm::FuncIdx(module.funcs.len() as u32);
                                        module.exports.push(wasm::Export {
                                            name: wasm::Name(export_name.to_string()),
                                            desc: wasm::ExportDesc::Func(func_idx),
                                        });
                                    }
                                    _ => todo!(),
                                }
                            }
                            _ => todo!(),
                        }
                    }
                    _ => todo!(),
                }
                let mut r#type = wasm::FuncType(wasm::ResultType(vec![]), wasm::ResultType(vec![]));
                for (_, param_type) in params.iter() {
                    match param_type {
                        ir::Type::Int => r#type.0 .0.push(wasm::ValType::I32),
                        _ => todo!(),
                    }
                }
                match return_type {
                    ir::Type::Int => r#type.1 .0.push(wasm::ValType::I32),
                    _ => todo!(),
                }
                let type_idx = wasm::TypeIdx(module.types.len() as u32);
                module.types.push(r#type);
                let locals = vec![];
                let mut ctx = Context::new();
                for (param_name, param_type) in params.iter() {
                    ctx.locals.push((param_name, param_type));
                }
                let mut instructions = vec![];
                self.generate_expr(body, &mut ctx, &mut instructions)?;
                module.funcs.push(wasm::Func {
                    r#type: type_idx,
                    locals,
                    body: wasm::Expr(instructions),
                });
            }
            ir::Stmt::Expr(expr) => {
                let mut ctx = Context::new();
                self.generate_expr(expr, &mut ctx, instructions)?;
                todo!();
            }
        }
        Ok(())
    }

    fn generate_expr(
        &self,
        expr: &ir::Expr,
        ctx: &mut Context,
        instructions: &mut Vec<wasm::Instr>,
    ) -> Result<()> {
        match expr {
            ir::Expr::BinOp { op, lhs, rhs } => {
                self.generate_expr(lhs, ctx, instructions)?;
                self.generate_expr(rhs, ctx, instructions)?;
                match op {
                    ir::BinOp::Add => {
                        instructions.push(wasm::Instr::I32Add);
                    }
                    ir::BinOp::Sub => {
                        instructions.push(wasm::Instr::I32Sub);
                    }
                    ir::BinOp::Mul => {
                        instructions.push(wasm::Instr::I32Mul);
                    }
                }
            }
            ir::Expr::Ident(name) => {
                let mut found = false;
                for (idx, (local_name, _)) in ctx.locals.iter().enumerate() {
                    if name == *local_name {
                        instructions.push(wasm::Instr::LocalGet(wasm::LocalIdx(idx as u32)));
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err("".into());
                }
            }
            ir::Expr::IntLit(raw) => {
                let value = raw.parse::<i32>()?;
                instructions.push(wasm::Instr::I32Const(value as u32));
            }
            _ => todo!(),
        }
        Ok(())
    }
}
