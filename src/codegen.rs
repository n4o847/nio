#![allow(dead_code)]

use std::cell::RefCell;
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

pub struct CodeGenerator {}

impl CodeGenerator {
    fn new() -> Self {
        Self {}
    }

    pub fn generate(program: &ir::Program) -> Result<wasm::Module, CodegenError> {
        let g = Self::new();
        let ctx = Context::new();
        let mut module = Module0::new();
        g.pregenerate_program(program, &ctx, &mut module)?;
        let mut module = module.upgrade();
        g.generate_program(program, &ctx, &mut module)?;
        let module = module.upgrade();
        Ok(module)
    }

    fn pregenerate_program<'a>(
        &self,
        program: &'a ir::Program,
        ctx: &'a Context<'a>,
        module: &mut Module0,
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
        module: &mut Module1,
    ) -> Result<(), CodegenError> {
        let start_func_type_idx = module.add_type(wasm::Type(wasm::RecType(vec![wasm::SubType(
            Some(wasm::Final),
            vec![],
            wasm::CompType::Func(wasm::ResultType(vec![]), wasm::ResultType(vec![])),
        )])));
        let mut start_func = wasm::Func(start_func_type_idx, vec![], wasm::Expr(vec![]));
        for stmt in program.statements.iter() {
            self.generate_stmt(stmt, &ctx, module, &mut start_func)?;
        }
        let start_func_idx = module.add_func(start_func);
        module.add_export(wasm::Export(
            wasm::Name("_start".to_string()),
            wasm::ExternIdx::Func(start_func_idx),
        ));
        Ok(())
    }

    fn pregenerate_stmt<'a>(
        &self,
        stmt: &'a ir::Stmt,
        ctx: &'a Context<'a>,
        module: &mut Module0,
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
                                        if body.is_some() {
                                            return Err(
                                                CodegenError::RedundantFunctionDefinition {
                                                    func_name: annot_name.to_string(),
                                                },
                                            );
                                        };
                                        let func_type_idx =
                                            module.add_type(to_wasm_func_type(params, return_type));
                                        let func_idx = module.add_func_import(
                                            wasm::Name(import_module.to_string()),
                                            wasm::Name(import_name.to_string()),
                                            wasm::TypeUse(func_type_idx),
                                        );
                                        ctx.0.as_ref().borrow_mut().funcs.push(Func {
                                            name,
                                            index: func_idx,
                                        });
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
        module: &mut Module1,
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
                match annotations.len() {
                    1 => {
                        let annot = &annotations[0];
                        match annot {
                            ir::Expr::Call { callee, .. } => match callee.as_ref() {
                                ir::Expr::Ident(name) => match name.as_ref() {
                                    "import" => {
                                        // Already pre-generated.
                                        return Ok(());
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
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
                let func_type_idx = module.add_type(to_wasm_func_type(params, return_type));
                let func_idx =
                    module.add_func(wasm::Func(func_type_idx, locals, wasm::Expr(instructions)));
                ctx.0.as_ref().borrow_mut().funcs.push(Func {
                    name,
                    index: func_idx.clone(),
                });
                match annotations.len() {
                    0 => {}
                    1 => {
                        let annot = &annotations[0];
                        match annot {
                            ir::Expr::Call { callee, args } => match callee.as_ref() {
                                ir::Expr::Ident(name) => match name.as_ref() {
                                    "export" => match args.as_slice() {
                                        [ir::Expr::StringLit(export_name)] => {
                                            module.add_export(wasm::Export(
                                                wasm::Name(export_name.to_string()),
                                                wasm::ExternIdx::Func(func_idx),
                                            ));
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
            }
            ir::Stmt::Let { name, type_, value } => {
                self.generate_expr(value, ctx, &mut func.2.0)?;
                func.1
                    .push(wasm::Local(wasm::ValType::NumType(wasm::NumType::I32)));
                let local_idx = wasm::LocalIdx(ctx.0.as_ref().borrow().locals.len() as u32);
                ctx.0
                    .as_ref()
                    .borrow_mut()
                    .locals
                    .push(Local { name, type_ });
                func.2.0.push(wasm::Instr::LocalSet(local_idx));
            }
            ir::Stmt::Expr(expr) => {
                let mut ctx = ctx.inherit();
                self.generate_expr(expr, &mut ctx, &mut func.2.0)?;
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

fn to_wasm_func_type(params: &Vec<(String, ir::Type)>, return_type: &ir::Type) -> wasm::Type {
    let mut func_type = (wasm::ResultType(vec![]), wasm::ResultType(vec![]));
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
    wasm::Type(wasm::RecType(vec![wasm::SubType(
        Some(wasm::Final),
        vec![],
        wasm::CompType::Func(func_type.0, func_type.1),
    )]))
}

// https://webassembly.github.io/spec/core/syntax/modules.html#syntax-index
// > The index space for tags, globals, memories, tables, and functions includes respective imports
// > declared in the same module. The indices of these imports precede the indices of other
// > definitions in the same index space.

struct Module0 {
    types: Vec<wasm::Type>,
    imports: Vec<wasm::Import>,
}

struct Module1 {
    types: Vec<wasm::Type>,
    imports: Vec<wasm::Import>,
    funcs: Vec<wasm::Func>,
    exports: Vec<wasm::Export>,
}

impl Module0 {
    fn new() -> Self {
        Module0 {
            types: Vec::new(),
            imports: Vec::new(),
        }
    }

    fn upgrade(self) -> Module1 {
        let Module0 { types, imports } = self;
        Module1 {
            types,
            imports,
            funcs: Vec::new(),
            exports: Vec::new(),
        }
    }

    fn add_type(&mut self, type_: wasm::Type) -> wasm::TypeIdx {
        let idx = wasm::TypeIdx(self.types.len() as u32);
        self.types.push(type_);
        idx
    }

    fn add_func_import(
        &mut self,
        module: wasm::Name,
        name: wasm::Name,
        type_: wasm::TypeUse,
    ) -> wasm::FuncIdx {
        let idx = wasm::FuncIdx(self.imports.len() as u32);
        self.imports
            .push(wasm::Import(module, name, wasm::ExternType::Func(type_)));
        idx
    }
}

impl Module1 {
    fn upgrade(self) -> wasm::Module {
        let Module1 {
            types,
            imports,
            funcs,
            exports,
        } = self;
        wasm::Module(
            types,
            imports,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            funcs,
            Vec::new(),
            Vec::new(),
            None,
            exports,
        )
    }

    fn add_type(&mut self, type_: wasm::Type) -> wasm::TypeIdx {
        let idx = wasm::TypeIdx(self.types.len() as u32);
        self.types.push(type_);
        idx
    }

    fn add_func(&mut self, func: wasm::Func) -> wasm::FuncIdx {
        let idx = wasm::FuncIdx(
            (self
                .imports
                .iter()
                .filter(|wasm::Import(_, _, xt)| matches!(xt, wasm::ExternType::Func(_)))
                .count()
                + self.funcs.len()) as u32,
        );
        self.funcs.push(func);
        idx
    }

    fn add_export(&mut self, export: wasm::Export) {
        self.exports.push(export);
    }
}
