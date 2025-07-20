#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Def {
        annotations: Vec<Expr>,
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Box<Expr>,
    },
    Let {
        name: String,
        type_: Type,
        value: Box<Expr>,
    },
    Expr(Expr),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Type {
    Unresolved(String),
    Untyped,
    Unit,
    Int,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
}
