use super::instructions::*;
use super::types::*;
use super::values::*;

// https://webassembly.github.io/spec/core/syntax/modules.html

pub struct Module {
  pub types: Vec<FuncType>,
  pub funcs: Vec<Func>,
  pub tables: Vec<Table>,
  pub mems: Vec<Mem>,
  pub globals: Vec<Global>,
  pub elem: Vec<Elem>,
  pub data: Vec<Data>,
  pub start: Option<Start>,
  pub imports: Vec<Import>,
  pub exports: Vec<Export>,
}

// Indices

pub struct TypeIdx(u32);

pub struct FuncIdx(u32);

pub struct TableIdx(u32);

pub struct MemIdx(u32);

pub struct GlobalIdx(u32);

pub struct LocalIdx(u32);

pub struct LabelIdx(u32);

// Functions

pub struct Func {
  pub r#type: TypeIdx,
  pub locals: ValType,
  pub body: Expr,
}

// Tables

pub struct Table {
  pub r#type: TableType,
}

// Memories

pub struct Mem {
  pub r#type: MemType,
}

// Globals

pub struct Global {
  pub r#type: GlobalType,
  pub init: Expr,
}

// Element Segments

pub struct Elem {
  pub table: TableIdx,
  pub offset: Expr,
  pub init: Vec<FuncIdx>,
}

// Data Segments

pub struct Data {
  pub data: MemIdx,
  pub offset: Expr,
  pub init: Vec<u8>,
}

// Start Function

pub struct Start {
  pub func: FuncIdx,
}

// Exports

pub struct Export {
  pub name: Name,
  pub desc: (),
}

pub enum ExportDesc {
  Func(FuncIdx),
  Table(TableIdx),
  Mem(MemIdx),
  Global(GlobalIdx),
}

// Imports

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
      elem: Vec::new(),
      data: Vec::new(),
      start: None,
      imports: Vec::new(),
      exports: Vec::new(),
    }
  }
}
