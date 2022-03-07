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
