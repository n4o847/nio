use super::modules::*;
use super::types::*;

// https://webassembly.github.io/spec/core/syntax/instructions.html

// 2.4 Instructions

pub enum Instr {
    // 2.4.1 Numeric Instructions
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

    F32Abs,
    F32Neg,
    F32Sqrt,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F64Abs,
    F64Neg,
    F64Sqrt,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,

    // Binary Operations
    I32Add,
    I32Sub,
    I32Mul,
    I32DivU,
    I32DivS,
    I32RemU,
    I32RemS,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrU,
    I32ShrS,
    I32Rotl,
    I32Rotr,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivU,
    I64DivS,
    I64RemU,
    I64RemS,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrU,
    I64ShrS,
    I64Rotl,
    I64Rotr,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    // Tests
    I32Eqz,
    I64Eqz,

    // Comparisons
    I32Eq,
    I32Ne,
    I32LtU,
    I32LtS,
    I32GtU,
    I32GtS,
    I32LeU,
    I32LeS,
    I32GeU,
    I32GeS,
    I64Eq,
    I64Ne,
    I64LtU,
    I64LtS,
    I64GtU,
    I64GtS,
    I64LeU,
    I64LeS,
    I64GeU,
    I64GeS,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    // Conversions
    I32Extend8S,  // Wasm 2.0 or later
    I32Extend16S, // Wasm 2.0 or later
    I64Extend8S,  // Wasm 2.0 or later
    I64Extend16S, // Wasm 2.0 or later
    I64Extend32S, // Wasm 2.0 or later

    I32WrapI64,
    I64ExtendI32U,
    I64ExtendI32S,
    I32TruncF32U,
    I32TruncF32S,
    I32TruncF64U,
    I32TruncF64S,
    I64TruncF32U,
    I64TruncF32S,
    I64TruncF64U,
    I64TruncF64S,
    I32TruncSatF32U, // Wasm 2.0 or later
    I32TruncSatF32S, // Wasm 2.0 or later
    I32TruncSatF64U, // Wasm 2.0 or later
    I32TruncSatF64S, // Wasm 2.0 or later
    I64TruncSatF32U, // Wasm 2.0 or later
    I64TruncSatF32S, // Wasm 2.0 or later
    I64TruncSatF64U, // Wasm 2.0 or later
    I64TruncSatF64S, // Wasm 2.0 or later

    F32DemoteF64,
    F64PromoteF32,
    F32ConvertI32U,
    F32ConvertI32S,
    F32ConvertI64U,
    F32ConvertI64S,
    F64ConvertI32U,
    F64ConvertI32S,
    F64ConvertI64U,
    F64ConvertI64S,

    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,

    // 2.4.2 Vector Instructions
    // TODO

    // 2.4.3 Reference Instructions
    RefNull(RefType),
    RefIsNull,
    RefFunc(FuncIdx),

    // 2.4.4 Parametric Instructions
    Drop,
    Select(Option<Vec<ValType>>),

    // 2.4.5 Variable Instructions
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

    // 2.4.6 Table Instructions
    // TODO

    // 2.4.7 Memory Instructions
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
    I32Load16U(MemArg),
    I32Load16S(MemArg),
    I64Load8U(MemArg),
    I64Load8S(MemArg),
    I64Load16U(MemArg),
    I64Load16S(MemArg),
    I64Load32U(MemArg),
    I64Load32S(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),

    MemorySize,
    MemoryGrow,
    MemoryFill,          // Wasm 2.0 or later
    MemoryCopy,          // Wasm 2.0 or later
    MemoryInit(DataIdx), // Wasm 2.0 or later
    DataDrop(DataIdx),   // Wasm 2.0 or later

    // 2.4.8 Control Instructions
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
    CallIndirect(TableIdx, TypeIdx),
}

pub struct MemArg {
    pub offset: u32,
    pub align: u32,
}

pub enum BlockType {
    TypeIdx(TypeIdx),
    ValType(Option<ValType>),
}

// 2.4.9 Expressions

pub struct Expr(pub Vec<Instr>);
