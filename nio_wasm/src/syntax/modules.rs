use super::instructions::*;
use super::types::*;
use super::values::*;

// https://webassembly.github.io/spec/core/syntax/modules.html

// 2.5 Modules

pub struct Module(
    pub Vec<Type>,
    pub Vec<Import>,
    pub Vec<Tag>, // Wasm 3.0 or later
    pub Vec<Global>,
    pub Vec<Mem>,
    pub Vec<Table>,
    pub Vec<Func>,
    pub Vec<Data>,
    pub Vec<Elem>,
    pub Option<Start>,
    pub Vec<Export>,
);

// 2.5.1 Indices

pub type Idx = u32;

#[derive(Clone, PartialEq, Eq)]
pub struct TypeIdx(pub Idx);

#[derive(Clone)]
pub struct FuncIdx(pub Idx);

#[derive(Clone)]
pub struct GlobalIdx(pub Idx);

#[derive(Clone)]
pub struct TableIdx(pub Idx);

#[derive(Clone)]
pub struct MemIdx(pub Idx);

#[derive(Clone)]
pub struct TagIdx(pub Idx); // Wasm 3.0 or later

#[derive(Clone)]
pub struct ElemIdx(pub Idx); // Wasm 2.0 or later

#[derive(Clone)]
pub struct DataIdx(pub Idx); // Wasm 2.0 or later

#[derive(Clone)]
pub struct LabelIdx(pub Idx);

#[derive(Clone)]
pub struct LocalIdx(pub Idx);

#[derive(Clone)]
pub struct FieldIdx(pub Idx); // Wasm 3.0 or later

// 2.5.2 Types

pub struct Type(pub RecType); // Wasm 3.0 or later

// 2.5.3 Tags (Wasm 3.0 or later)

pub struct Tag(pub TagType);

// 2.5.4 Globals

pub struct Global(pub GlobalType, pub Expr);

// 2.5.5 Memories

pub struct Mem(pub MemType);

// 2.5.6 Tables

pub struct Table(
    pub TableType,
    pub Expr, // Wasm 3.0 or later
);

// 2.5.7 Functions

pub struct Func(pub TypeIdx, pub Vec<Local>, pub Expr);

pub struct Local(pub ValType);

// 2.5.8 Data Segments

pub struct Data(pub Vec<u8>, pub DataMode);

pub enum DataMode {
    Active(MemIdx, Expr),
    Passive, // Wasm 2.0 or later
}

// 2.5.9 Element Segments

pub struct Elem(
    pub RefType, // Wasm 2.0 or later
    pub Vec<Expr>,
    pub ElemMode,
);

pub enum ElemMode {
    Active(TableIdx, Expr),
    Passive,     // Wasm 2.0 or later
    Declarative, // Wasm 2.0 or later
}

// 2.5.10 Start Function

pub struct Start(pub FuncIdx);

// 2.5.11 Imports

pub struct Import(pub Name, pub Name, pub ExternType);

// 2.5.10 Exports

pub struct Export(pub Name, pub ExternIdx);

pub enum ExternIdx {
    Func(FuncIdx),
    Global(GlobalIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Tag(TagIdx), // Wasm 3.0 or later
}

// Implementations

impl Module {
    pub fn new() -> Self {
        Self(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
        )
    }
}
