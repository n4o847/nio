use super::instructions::*;
use super::types::*;
use super::values::*;

// https://webassembly.github.io/spec/core/syntax/modules.html

// 2.5 Modules

pub struct Module {
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elems: Vec<Elem>,
    pub datas: Vec<Data>,
    pub start: Option<Start>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

// 2.5.1 Indices

#[derive(Clone)]
pub struct TypeIdx(pub u32);

#[derive(Clone)]
pub struct FuncIdx(pub u32);

#[derive(Clone)]
pub struct TableIdx(pub u32);

#[derive(Clone)]
pub struct MemIdx(pub u32);

#[derive(Clone)]
pub struct GlobalIdx(pub u32);

#[derive(Clone)]
pub struct ElemIdx(pub u32); // Wasm 2.0 or later

#[derive(Clone)]
pub struct DataIdx(pub u32); // Wasm 2.0 or later

#[derive(Clone)]
pub struct LocalIdx(pub u32);

#[derive(Clone)]
pub struct LabelIdx(pub u32);

// 2.5.3 Functions

pub struct Func {
    pub type_: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

// 2.5.4 Tables

pub struct Table {
    pub type_: TableType,
}

// 2.5.5 Memories

pub struct Mem {
    pub type_: MemType,
}

// 2.5.6 Globals

pub struct Global {
    pub type_: GlobalType,
    pub init: Expr,
}

// 2.5.7 Element Segments

pub struct Elem {
    pub type_: RefType, // Wasm 2.0 or later
    pub init: Vec<Expr>,
    pub mode: ElemMode,
}

pub enum ElemMode {
    Passive, // Wasm 2.0 or later
    Active { table: TableIdx, offset: Expr },
    Declarative, // Wasm 2.0 or later
}

// 2.5.8 Data Segments

pub struct Data {
    pub init: Vec<u8>,
    pub mode: DataMode,
}

pub enum DataMode {
    Passive, // Wasm 2.0 or later
    Active { memory: MemIdx, offset: Expr },
}

// 2.5.9 Start Function

pub struct Start {
    pub func: FuncIdx,
}

// 2.5.10 Exports

pub struct Export {
    pub name: Name,
    pub desc: ExportDesc,
}

pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

// 2.5.11 Imports

pub struct Import {
    pub module: Name,
    pub name: Name,
    pub desc: ImportDesc,
}

pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

// Implementations

impl Module {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            funcs: Vec::new(),
            tables: Vec::new(),
            mems: Vec::new(),
            globals: Vec::new(),
            elems: Vec::new(),
            datas: Vec::new(),
            start: None,
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }
}
