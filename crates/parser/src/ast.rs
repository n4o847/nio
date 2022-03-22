#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Def {
        annotations: Vec<Expr>,
        name: String,
        params: Vec<(String, String)>,
        return_type: String,
        body: Box<Expr>,
    },
    Let {
        name: String,
        type_: Option<String>,
        value: Box<Expr>,
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
