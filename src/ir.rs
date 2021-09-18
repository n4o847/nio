use crate::parser::ast;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Def {
        name: String,
        params: Vec<(String, String)>,
        return_type: String,
        body: Box<Expr>,
    },
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
                name,
                params,
                return_type,
                body,
            } => Stmt::Def {
                name,
                params,
                return_type,
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
