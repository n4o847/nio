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

enum Scope {
    Func,
    Block,
}

struct ContextImpl<'a> {
    scope: Scope,
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
    index: wasm::LocalIdx,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(ContextImpl {
            scope: Scope::Block,
            parent: None,
            funcs: Vec::new(),
            locals: Vec::new(),
        })))
    }

    fn new_func(&self) -> Context<'a> {
        Self(Rc::new(RefCell::new(ContextImpl {
            scope: Scope::Func,
            parent: Some(Self(Rc::clone(&self.0))),
            funcs: Vec::new(),
            locals: Vec::new(),
        })))
    }

    fn new_block(&self) -> Context<'a> {
        Self(Rc::new(RefCell::new(ContextImpl {
            scope: Scope::Block,
            parent: Some(Self(Rc::clone(&self.0))),
            funcs: Vec::new(),
            locals: Vec::new(),
        })))
    }

    fn add_func(&self, name: &'a String, index: wasm::FuncIdx) {
        self.0.borrow_mut().funcs.push(Func { name, index });
    }

    fn find_func_index<'b>(&'b self, name: &str) -> Option<wasm::FuncIdx> {
        let ctx = self.0.borrow();
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

    fn add_local(&self, name: &'a String, index: wasm::LocalIdx) {
        let mut ctx = self.0.borrow_mut();
        ctx.locals.push(Local { name, index });
    }

    fn find_local_index<'b>(&'b self, name: &str) -> Option<wasm::LocalIdx> {
        let ctx = self.0.borrow();
        for local in ctx.locals.iter() {
            if local.name == name {
                return Some(local.index.clone());
            }
        }
        match ctx.scope {
            Scope::Func => return None,
            Scope::Block => {
                if let Some(parent) = ctx.parent.as_ref() {
                    return parent.find_local_index(name);
                }
            }
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
        ctx: &Context<'a>,
        module: &mut Module0,
    ) -> Result<(), CodegenError> {
        for stmt in program.statements.iter() {
            self.pregenerate_stmt(stmt, ctx, module)?;
        }
        Ok(())
    }

    fn generate_program<'a>(
        &self,
        program: &'a ir::Program,
        ctx: &Context<'a>,
        module: &mut Module1,
    ) -> Result<(), CodegenError> {
        let start_func_type_idx = module.add_type(wasm::Type(wasm::RecType(vec![wasm::SubType(
            Some(wasm::Final),
            vec![],
            wasm::CompType::Func(wasm::ResultType(vec![]), wasm::ResultType(vec![])),
        )])));
        let mut start_func = wasm::Func(start_func_type_idx, vec![], wasm::Expr(vec![]));
        for stmt in program.statements.iter() {
            self.generate_stmt(stmt, ctx, module, &mut start_func)?;
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
        ctx: &Context<'a>,
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
                                        ctx.add_func(name, func_idx);
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
        ctx: &Context<'a>,
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
                let ctx = ctx.new_func();
                for (param_index, (param_name, _)) in params.iter().enumerate() {
                    ctx.add_local(param_name, wasm::LocalIdx(param_index as u32));
                }
                let func_type_idx = module.add_type(to_wasm_func_type(params, return_type));
                let mut func = wasm::Func(func_type_idx, vec![], wasm::Expr(vec![]));
                self.generate_expr(body, &ctx, module, &mut func)?;
                let func_idx = module.add_func(func);
                ctx.add_func(name, func_idx.clone());
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
            ir::Stmt::Let { name, value, .. } => {
                self.generate_expr(value, ctx, module, func)?;
                let local_idx = module.add_local(
                    func,
                    wasm::Local(wasm::ValType::NumType(wasm::NumType::I32)),
                );
                ctx.add_local(name, local_idx.clone());
                func.2.0.push(wasm::Instr::LocalSet(local_idx));
            }
            ir::Stmt::Expr(expr) => {
                self.generate_expr(expr, ctx, module, func)?;
            }
        }
        Ok(())
    }

    fn generate_expr<'a>(
        &self,
        expr: &'a ir::Expr,
        ctx: &Context<'a>,
        module: &mut Module1,
        func: &mut wasm::Func,
    ) -> Result<(), CodegenError> {
        match expr {
            ir::Expr::Block { statements } => {
                let ctx = ctx.new_block();
                for stmt in statements.iter() {
                    self.generate_stmt(stmt, &ctx, module, func)?;
                }
            }
            ir::Expr::BinOp { op, lhs, rhs } => {
                self.generate_expr(lhs, ctx, module, func)?;
                self.generate_expr(rhs, ctx, module, func)?;
                match op {
                    ir::BinOp::Add => {
                        func.2.0.push(wasm::Instr::I32Add);
                    }
                    ir::BinOp::Sub => {
                        func.2.0.push(wasm::Instr::I32Sub);
                    }
                    ir::BinOp::Mul => {
                        func.2.0.push(wasm::Instr::I32Mul);
                    }
                }
            }
            ir::Expr::Ident(name) => {
                let Some(local_idx) = ctx.find_local_index(name) else {
                    return Err(CodegenError::UndefinedVariable { name: name.clone() });
                };
                func.2.0.push(wasm::Instr::LocalGet(local_idx));
            }
            ir::Expr::IntLit(raw) => {
                let value =
                    raw.parse::<i32>()
                        .map_err(|err| CodegenError::InvalidIntegerLiteral {
                            raw: raw.clone(),
                            reason: err,
                        })?;
                func.2.0.push(wasm::Instr::I32Const(value as u32));
            }
            ir::Expr::Call { callee, args } => {
                for arg in args.iter() {
                    self.generate_expr(arg, ctx, module, func)?;
                }
                match callee.as_ref() {
                    ir::Expr::Ident(name) => {
                        let Some(func_idx) = ctx.find_func_index(name) else {
                            return Err(CodegenError::UndefinedVariable { name: name.clone() });
                        };
                        func.2.0.push(wasm::Instr::Call(func_idx));
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
        Ok(())
    }
}

fn to_wasm_func_type(params: &Vec<(String, ir::Type)>, return_type: &ir::Type) -> wasm::Type {
    let mut wasm_param_types = vec![];
    for (_, param_type) in params.iter() {
        match param_type {
            ir::Type::Int => wasm_param_types.push(wasm::ValType::NumType(wasm::NumType::I32)),
            _ => todo!(),
        }
    }
    let mut wasm_return_types = vec![];
    match return_type {
        ir::Type::Int => wasm_return_types.push(wasm::ValType::NumType(wasm::NumType::I32)),
        _ => todo!(),
    }
    wasm::Type(wasm::RecType(vec![wasm::SubType(
        Some(wasm::Final),
        vec![],
        wasm::CompType::Func(
            wasm::ResultType(wasm_param_types),
            wasm::ResultType(wasm_return_types),
        ),
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

    fn add_local(&self, func: &mut wasm::Func, local: wasm::Local) -> wasm::LocalIdx {
        let wasm::Func(wasm::TypeIdx(type_idx), locals, _) = func;
        let func_type = &self.types[*type_idx as usize];
        let wasm::Type(wasm::RecType(subtypes)) = func_type;
        let param_count = match &subtypes[0] {
            wasm::SubType(_, _, wasm::CompType::Func(wasm::ResultType(params), _)) => params.len(),
            _ => unreachable!(),
        };
        let local_idx = wasm::LocalIdx(param_count as u32 + locals.len() as u32);
        locals.push(local);
        local_idx
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
