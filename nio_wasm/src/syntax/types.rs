// https://webassembly.github.io/spec/core/syntax/types.html

// 2.3 Types

// 2.3.1 Number Types

#[derive(PartialEq, Eq)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

// 2.3.2 Vector Types (Wasm 2.0 or later)

#[derive(PartialEq, Eq)]
pub enum VecType {
    V128,
}

// 2.3.3 Reference Types (Wasm 2.0 or later)

#[derive(PartialEq, Eq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

// 2.3.4 Value Types (Wasm 2.0 or later)

#[derive(PartialEq, Eq)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

// 2.3.5 Result Types

pub struct ResultType(pub Vec<ValType>);

// 2.3.6 Function Types

pub struct FuncType(pub ResultType, pub ResultType);

// 2.3.7 Limits

pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

// 2.3.8 Memory Types

pub struct MemType(pub Limits);

// 2.3.9 Table Types

pub struct TableType(pub Limits, pub RefType);

// 2.3.10 Global Types

pub struct GlobalType(pub Mut, pub ValType);

pub enum Mut {
    Const,
    Var,
}

// 2.3.11 External Types

pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
