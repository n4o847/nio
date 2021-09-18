use crate::parser::ast;

pub struct Program {
    pub statements: Vec<Stmt>,
}

pub enum Stmt {
    Def {
        annotations: Vec<Expr>,
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Box<Expr>,
    },
    Expr(Expr),
}

pub enum Expr {
    BinOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Assign {
        lhs: String,
        rhs: Box<Expr>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Ident(String),
    IntLit(String),
    StringLit(String),
}

pub enum Type {
    Unresolved(String),
    Untyped,
    Unit,
    Int,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
}

impl From<ast::Program> for Program {
    fn from(p: ast::Program) -> Self {
        Program {
            statements: p.statements.into_iter().map(Stmt::from).collect(),
        }
    }
}

impl From<ast::Stmt> for Stmt {
    fn from(s: ast::Stmt) -> Self {
        match s {
            ast::Stmt::Def {
                annotations,
                name,
                params,
                return_type,
                body,
            } => Stmt::Def {
                annotations: annotations.into_iter().map(Expr::from).collect(),
                name,
                params: params
                    .into_iter()
                    .map(|(param_name, param_type)| (param_name, Type::Unresolved(param_type)))
                    .collect(),
                return_type: Type::Unresolved(return_type),
                body: Box::new(Expr::from(*body)),
            },
            ast::Stmt::Expr(e) => Stmt::Expr(Expr::from(e)),
        }
    }
}

impl From<ast::Expr> for Expr {
    fn from(e: ast::Expr) -> Self {
        match e {
            ast::Expr::BinOp { op, lhs, rhs } => Expr::BinOp {
                op: BinOp::from(op),
                lhs: Box::new(Expr::from(*lhs)),
                rhs: Box::new(Expr::from(*rhs)),
            },
            ast::Expr::Assign { lhs, rhs } => Expr::Assign {
                lhs,
                rhs: Box::new(Expr::from(*rhs)),
            },
            ast::Expr::Lambda { params, body } => Expr::Lambda {
                params,
                body: Box::new(Expr::from(*body)),
            },
            ast::Expr::Call { callee, args } => Expr::Call {
                callee: Box::new((*callee).into()),
                args: args.into_iter().map(Expr::from).collect(),
            },
            ast::Expr::Ident(i) => Expr::Ident(i),
            ast::Expr::IntLit(i) => Expr::IntLit(i),
            ast::Expr::StringLit(s) => Expr::StringLit(s),
        }
    }
}

impl From<ast::BinOp> for BinOp {
    fn from(o: ast::BinOp) -> Self {
        match o {
            ast::BinOp::Add => BinOp::Add,
            ast::BinOp::Sub => BinOp::Sub,
            ast::BinOp::Mul => BinOp::Mul,
        }
    }
}
