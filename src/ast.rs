#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Assign {
        left: String,
        right: Box<Expr>,
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
