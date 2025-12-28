#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::rc::Rc;

use nio_wasm as wasm;
use thiserror::Error;

use crate::ir;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("No function definition for {func_name:?}")]
    NoFunctionDefinition { func_name: String },
    #[error("Redundant function definition for {func_name:?}")]
    RedundantFunctionDefinition { func_name: String },
    #[error("Undefined variable {name:?}")]
    UndefinedVariable { name: String },
    #[error("Invalid integer literal {raw:?}: {reason:?}")]
    InvalidIntegerLiteral { raw: String, reason: ParseIntError },
}

struct Context<'a>(Rc<RefCell<ContextImpl<'a>>>);

struct ContextImpl<'a> {
    parent: Option<Context<'a>>,
    funcs: Vec<Func<'a>>,
    locals: Vec<Local<'a>>,
}

#[derive(Clone)]
struct Func<'a> {
    name: &'a String,
    index: wasm::FuncIdx,
}

#[derive(Clone)]
struct Local<'a> {
    name: &'a String,
    type_: &'a ir::Type,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(ContextImpl {
            parent: None,
            funcs: Vec::new(),
            locals: Vec::new(),
        })))
    }

    fn inherit(&'a self) -> Context<'a> {
        Self(Rc::new(RefCell::new(ContextImpl {
            parent: Some(Self(Rc::clone(&self.0))),
            funcs: Vec::new(),
            locals: Vec::new(),
        })))
    }

    fn find_func_index<'b>(&'b self, name: &str) -> Option<wasm::FuncIdx> {
        let ctx = self.0.as_ref().borrow();
        for func in ctx.funcs.iter() {
            if func.name == name {
                return Some(func.index.clone());
            }
        }
        if let Some(parent) = ctx.parent.as_ref() {
            return parent.find_func_index(name);
        }
        None
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

    pub fn generate(program: &ir::Program) -> Result<wasm::Module, CodegenError> {
        let g = Self::new();
        let mut module = wasm::Module::new();
        let ctx = Context::new();
        g.pregenerate_program(program, &ctx, &mut module)?;
        g.generate_program(program, &ctx, &mut module)?;
        Ok(module)
    }

    fn pregenerate_program<'a>(
        &self,
        program: &'a ir::Program,
        ctx: &'a Context<'a>,
        module: &mut wasm::Module,
    ) -> Result<(), CodegenError> {
        for stmt in program.statements.iter() {
            self.pregenerate_stmt(stmt, &ctx, module)?;
        }
        Ok(())
    }

    fn generate_program<'a>(
        &self,
        program: &'a ir::Program,
        ctx: &'a Context<'a>,
        module: &mut wasm::Module,
    ) -> Result<(), CodegenError> {
        let r#type = wasm::FuncType(wasm::ResultType(vec![]), wasm::ResultType(vec![]));
        let type_idx = wasm::TypeIdx(module.types.len() as u32);
        module.types.push(r#type);
        let mut start_func = wasm::Func {
            type_: type_idx,
            locals: vec![],
            body: wasm::Expr(vec![]),
        };
        for stmt in program.statements.iter() {
            self.generate_stmt(stmt, &ctx, module, &mut start_func)?;
        }
        let func_idx = new_func_idx(module);
        module.funcs.push(start_func);
        module.exports.push(wasm::Export {
            name: wasm::Name("_start".to_string()),
            desc: wasm::ExportDesc::Func(func_idx),
        });
        Ok(())
    }

    fn pregenerate_stmt<'a>(
        &self,
        stmt: &'a ir::Stmt,
        ctx: &'a Context<'a>,
        module: &mut wasm::Module,
    ) -> Result<(), CodegenError> {
        match stmt {
            ir::Stmt::Def {
                annotations,
                name,
                params,
                return_type,
                body,
            } => match annotations.len() {
                0 => {}
                1 => {
                    let annot = &annotations[0];
                    match annot {
                        ir::Expr::Call { callee, args } => match callee.as_ref() {
                            ir::Expr::Ident(annot_name) => match annot_name.as_ref() {
                                "import" => match args.as_slice() {
                                    [
                                        ir::Expr::StringLit(import_module),
                                        ir::Expr::StringLit(import_name),
                                    ] => {
                                        let r#type = to_wasm_func_type(params, return_type);
                                        let type_idx = wasm::TypeIdx(module.types.len() as u32);
                                        module.types.push(r#type);
                                        let func_idx = new_func_idx(module);
                                        ctx.0.as_ref().borrow_mut().funcs.push(Func {
                                            name,
                                            index: func_idx,
                                        });
                                        module.imports.push(wasm::Import {
                                            module: wasm::Name(import_module.to_string()),
                                            name: wasm::Name(import_name.to_string()),
                                            desc: wasm::ImportDesc::Func(type_idx),
                                        });
                                        if body.is_some() {
                                            return Err(
                                                CodegenError::RedundantFunctionDefinition {
                                                    func_name: annot_name.to_string(),
                                                },
                                            );
                                        };
                                    }
                                    _ => todo!("Invalid form of import annotation: {:?}", args),
                                },
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => {}
            },
            ir::Stmt::Let { .. } => {}
            ir::Stmt::Expr(_) => {}
        }
        Ok(())
    }

    fn generate_stmt<'a>(
        &self,
        stmt: &'a ir::Stmt,
        ctx: &'a Context<'a>,
        module: &mut wasm::Module,
        func: &mut wasm::Func,
    ) -> Result<(), CodegenError> {
        match stmt {
            ir::Stmt::Def {
                annotations,
                name,
                params,
                return_type,
                body,
            } => {
                let func_idx = new_func_idx(module);
                match annotations.len() {
                    0 => {}
                    1 => {
                        let annot = &annotations[0];
                        match annot {
                            ir::Expr::Call { callee, args } => match callee.as_ref() {
                                ir::Expr::Ident(name) => match name.as_ref() {
                                    "import" => {
                                        // Already pre-generated.
                                        return Ok(());
                                    }
                                    "export" => match args.as_slice() {
                                        [ir::Expr::StringLit(export_name)] => {
                                            module.exports.push(wasm::Export {
                                                name: wasm::Name(export_name.to_string()),
                                                desc: wasm::ExportDesc::Func(func_idx.clone()),
                                            });
                                        }
                                        _ => todo!("Invalid form of export annotation: {:?}", args),
                                    },
                                    _ => todo!("Unsupported annotation: {:?}", name),
                                },
                                _ => todo!("Unsupported annotation: {:?}", annot),
                            },
                            _ => todo!("Unsupported annotation: {:?}", annot),
                        }
                    }
                    _ => todo!("Multiple annotations not supported"),
                }
                let r#type = to_wasm_func_type(params, return_type);
                let type_idx = wasm::TypeIdx(module.types.len() as u32);
                module.types.push(r#type);
                ctx.0.as_ref().borrow_mut().funcs.push(Func {
                    name,
                    index: func_idx,
                });
                let Some(body) = body else {
                    return Err(CodegenError::NoFunctionDefinition {
                        func_name: name.to_string(),
                    });
                };
                let locals = vec![];
                let mut ctx = ctx.inherit();
                for (param_name, param_type) in params.iter() {
                    ctx.0.as_ref().borrow_mut().locals.push(Local {
                        name: param_name,
                        type_: param_type,
                    });
                }
                let mut instructions = vec![];
                self.generate_expr(body, &mut ctx, &mut instructions)?;
                module.funcs.push(wasm::Func {
                    type_: type_idx,
                    locals,
                    body: wasm::Expr(instructions),
                });
            }
            ir::Stmt::Let { name, type_, value } => {
                self.generate_expr(value, ctx, &mut func.body.0)?;
                func.locals.push(wasm::ValType::NumType(wasm::NumType::I32));
                let local_idx = wasm::LocalIdx(ctx.0.as_ref().borrow().locals.len() as u32);
                ctx.0
                    .as_ref()
                    .borrow_mut()
                    .locals
                    .push(Local { name, type_ });
                func.body.0.push(wasm::Instr::LocalSet(local_idx));
            }
            ir::Stmt::Expr(expr) => {
                let mut ctx = ctx.inherit();
                self.generate_expr(expr, &mut ctx, &mut func.body.0)?;
                todo!();
            }
        }
        Ok(())
    }

    fn generate_expr(
        &self,
        expr: &ir::Expr,
        ctx: &Context,
        instructions: &mut Vec<wasm::Instr>,
    ) -> Result<(), CodegenError> {
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
                for (idx, local) in ctx.0.as_ref().borrow().locals.iter().enumerate() {
                    if name == local.name {
                        instructions.push(wasm::Instr::LocalGet(wasm::LocalIdx(idx as u32)));
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(CodegenError::UndefinedVariable { name: name.clone() });
                }
            }
            ir::Expr::IntLit(raw) => {
                let value =
                    raw.parse::<i32>()
                        .map_err(|err| CodegenError::InvalidIntegerLiteral {
                            raw: raw.clone(),
                            reason: err,
                        })?;
                instructions.push(wasm::Instr::I32Const(value as u32));
            }
            ir::Expr::Call { callee, args } => {
                for arg in args.iter() {
                    self.generate_expr(arg, ctx, instructions)?;
                }
                match callee.as_ref() {
                    ir::Expr::Ident(name) => {
                        let idx = ctx.find_func_index(name).ok_or_else(|| {
                            CodegenError::UndefinedVariable {
                                name: name.to_string(),
                            }
                        })?;
                        instructions.push(wasm::Instr::Call(idx));
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
        dbg!(expr, instructions.len());
        Ok(())
    }
}

fn to_wasm_func_type(params: &Vec<(String, ir::Type)>, return_type: &ir::Type) -> wasm::FuncType {
    let mut func_type = wasm::FuncType(wasm::ResultType(vec![]), wasm::ResultType(vec![]));
    for (_, param_type) in params.iter() {
        match param_type {
            ir::Type::Int => func_type
                .0
                .0
                .push(wasm::ValType::NumType(wasm::NumType::I32)),
            _ => todo!(),
        }
    }
    match return_type {
        ir::Type::Int => func_type
            .1
            .0
            .push(wasm::ValType::NumType(wasm::NumType::I32)),
        _ => todo!(),
    }
    func_type
}

fn new_func_idx(module: &wasm::Module) -> wasm::FuncIdx {
    // https://webassembly.github.io/spec/core/syntax/modules.html#syntax-index
    // > The index space for functions, tables, memories and globals includes respective imports
    // > declared in the same module. The indices of these imports precede the indices of other
    // > definitions in the same index space.
    wasm::FuncIdx(
        (module
            .imports
            .iter()
            .filter(|i| matches!(i.desc, wasm::ImportDesc::Func(_)))
            .count()
            + module.funcs.len()) as u32,
    )
}
