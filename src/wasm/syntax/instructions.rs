use super::modules::*;
use super::types::*;

// https://webassembly.github.io/spec/core/syntax/instructions.html

// Instructions

pub enum Instr {
  // Numeric Instructions
  // Constants
  I32Const(u32),
  I64Const(u64),
  F32Const(f32),
  F64Const(f64),
  // Unary Operations
  I32Clz,
  I32Ctz,
  I32Popcnt,
  I64Clz,
  I64Ctz,
  I64Popcnt,
  // Binary Operations
  // Tests
  // Comparisons
  // Conversions
  // Parametric Instructions
  Drop,
  Select,
  // Variable Instructions
  LocalGet(LocalIdx),
  LocalSet(LocalIdx),
  LocalTee(LocalIdx),
  GlobalGet(GlobalIdx),
  GlobalSet(GlobalIdx),
  // Memory Instructions
  I32Load(MemArg),
  I64Load(MemArg),
  F32Load(MemArg),
  F64Load(MemArg),
  I32Store(MemArg),
  I64Store(MemArg),
  F32Store(MemArg),
  F64Store(MemArg),
  I32Load8U(MemArg),
  I32Load8S(MemArg),
  // ...
  MemorySize,
  MemoryGrow,
  // Control Instructions
  Nop,
  Unreachable,
  Block(BlockType, Vec<Instr>),
  Loop(BlockType, Vec<Instr>),
  IfElse(BlockType, Vec<Instr>, Vec<Instr>),
  Br(LabelIdx),
  BrIf(LabelIdx),
  BrTable(Vec<LabelIdx>, LabelIdx),
  Return,
  Call(FuncIdx),
  CallIndirect(TypeIdx),
}

pub struct MemArg {
  pub offset: u32,
  pub align: u32,
}

pub enum BlockType {
  TypeIdx(TypeIdx),
  ValType(Option<ValType>),
}

// Expressions

pub struct Expr(Vec<Instr>);
