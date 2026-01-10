use crate::TypeIdx;

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

// 2.3.3 Type Uses (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub struct TypeUse(pub TypeIdx);

// 2.3.4 Heap Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub enum AbsHeapType {
    Any,
    Eq,
    I31,
    Struct,
    Array,
    None,
    Func,
    NoFunc,
    Exn,
    NoExn,
    Extern,
    NoExtern,
}

#[derive(PartialEq, Eq)]
pub enum HeapType {
    AbsHeapType(AbsHeapType),
    TypeIdx(TypeIdx),
}

// 2.3.5 Reference Types (Wasm 2.0 or later)

#[derive(PartialEq, Eq)]
pub struct RefType(pub Option<Null>, pub HeapType);

#[derive(PartialEq, Eq)]
pub struct Null;

pub const ANYREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Any));
pub const EQREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Eq));
pub const I31REF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::I31));
pub const STRUCTREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Struct));
pub const ARRAYREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Array));
pub const FUNCREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Func));
pub const EXNREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Exn));
pub const EXTERNREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Extern));
pub const NULLREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::None));
pub const NULLFUNCREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::NoFunc));
pub const NULLEXNREF: RefType = RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::NoExn));
pub const NULLEXTERNREF: RefType =
    RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::NoExtern));

// 2.3.6 Value Types (Wasm 2.0 or later)

// Wasm 3.0 or later
#[derive(PartialEq, Eq)]
pub enum ConstType {
    NumType(NumType),
    VecType(VecType),
}

#[derive(PartialEq, Eq)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

// 2.3.7 Result Types

#[derive(PartialEq, Eq)]
pub struct ResultType(pub Vec<ValType>);

// 2.3.8 Block Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub enum BlockType {
    ValType(Option<ValType>),
    TypeIdx(TypeIdx),
}

// 2.3.9 Composite Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub enum CompType {
    Struct(Vec<FieldType>),
    Array(FieldType),
    Func(ResultType, ResultType),
}

#[derive(PartialEq, Eq)]
pub struct FieldType(pub Option<Mut>, pub StorageType);

#[derive(PartialEq, Eq)]
pub struct Mut;

#[derive(PartialEq, Eq)]
pub enum StorageType {
    ValType(ValType),
    PackType(PackType),
}

#[derive(PartialEq, Eq)]
pub enum PackType {
    I8,
    I16,
}

// 2.3.10 Recursive Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub struct RecType(pub Vec<SubType>);

#[derive(PartialEq, Eq)]
pub struct SubType(pub Option<Final>, pub Vec<TypeUse>, pub CompType);

#[derive(PartialEq, Eq)]
pub struct Final;

// 2.3.11 Address Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub enum AddrType {
    I32,
    I64,
}

// 2.3.12 Limits

#[derive(PartialEq, Eq)]
pub struct Limits(pub u64, pub Option<u64>);

// 2.3.13 Tag Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub struct TagType(pub TypeUse);

// 2.3.14 Global Types

#[derive(PartialEq, Eq)]
pub struct GlobalType(pub Option<Mut>, pub ValType);

// 2.3.15 Memory Types

#[derive(PartialEq, Eq)]
pub struct MemType(pub AddrType, pub Limits);

// 2.3.16 Table Types

#[derive(PartialEq, Eq)]
pub struct TableType(pub AddrType, pub Limits, pub RefType);

// 2.3.17 Data Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub enum DataType {
    Ok,
}

// 2.3.18 Element Types (Wasm 3.0 or later)

#[derive(PartialEq, Eq)]
pub enum ElemType {
    RefType(RefType),
}

// 2.3.19 External Types

#[derive(PartialEq, Eq)]
pub enum ExternType {
    Tag(TagType), // (Wasm 3.0 or later)
    Global(GlobalType),
    Mem(MemType),
    Table(TableType),
    Func(TypeUse),
}
