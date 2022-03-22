use crate::ir;
use crate::parser::ast;

impl From<ast::Program> for ir::Program {
    fn from(p: ast::Program) -> Self {
        ir::Program {
            statements: p.statements.into_iter().map(ir::Stmt::from).collect(),
        }
    }
}

impl From<ast::Stmt> for ir::Stmt {
    fn from(s: ast::Stmt) -> Self {
        match s {
            ast::Stmt::Def {
                annotations,
                name,
                params,
                return_type,
                body,
            } => ir::Stmt::Def {
                annotations: annotations.into_iter().map(ir::Expr::from).collect(),
                name,
                params: params
                    .into_iter()
                    .map(|(param_name, param_type)| (param_name, ir::Type::Unresolved(param_type)))
                    .collect(),
                return_type: ir::Type::Unresolved(return_type),
                body: Box::new(ir::Expr::from(*body)),
            },
            ast::Stmt::Let { name, type_, value } => ir::Stmt::Let {
                name,
                type_: match type_ {
                    Some(type_) => ir::Type::Unresolved(type_),
                    None => ir::Type::Untyped,
                },
                value: Box::new(ir::Expr::from(*value)),
            },
            ast::Stmt::Expr(e) => ir::Stmt::Expr(ir::Expr::from(e)),
        }
    }
}

impl From<ast::Expr> for ir::Expr {
    fn from(e: ast::Expr) -> Self {
        match e {
            ast::Expr::BinOp { op, lhs, rhs } => ir::Expr::BinOp {
                op: ir::BinOp::from(op),
                lhs: Box::new(ir::Expr::from(*lhs)),
                rhs: Box::new(ir::Expr::from(*rhs)),
            },
            ast::Expr::Assign { lhs, rhs } => ir::Expr::Assign {
                lhs,
                rhs: Box::new(ir::Expr::from(*rhs)),
            },
            ast::Expr::Lambda { params, body } => ir::Expr::Lambda {
                params,
                body: Box::new(ir::Expr::from(*body)),
            },
            ast::Expr::Call { callee, args } => ir::Expr::Call {
                callee: Box::new((*callee).into()),
                args: args.into_iter().map(ir::Expr::from).collect(),
            },
            ast::Expr::Ident(i) => ir::Expr::Ident(i),
            ast::Expr::IntLit(i) => ir::Expr::IntLit(i),
            ast::Expr::StringLit(s) => ir::Expr::StringLit(s),
        }
    }
}

impl From<ast::BinOp> for ir::BinOp {
    fn from(o: ast::BinOp) -> Self {
        match o {
            ast::BinOp::Add => ir::BinOp::Add,
            ast::BinOp::Sub => ir::BinOp::Sub,
            ast::BinOp::Mul => ir::BinOp::Mul,
        }
    }
}
