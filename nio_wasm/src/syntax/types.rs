// https://webassembly.github.io/spec/core/syntax/types.html

// Value Types

#[derive(PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

// Result Types

pub struct ResultType(pub Vec<ValType>);

// Function Types

pub struct FuncType(pub ResultType, pub ResultType);

// Limits

pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

// Memory Types

pub struct MemType(pub Limits);

// Table Types

pub struct TableType(pub Limits, pub ElemType);

pub struct ElemType;

// Global Types

pub struct GlobalType(pub Mut, pub ValType);

pub enum Mut {
    Const,
    Var,
}

// External Types

pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
