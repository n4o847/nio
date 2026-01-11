#![allow(unused_variables)]

use super::super::syntax::*;
use super::*;
use crate::binary::values::{F32, F64, U32, U64};

// https://webassembly.github.io/spec/core/binary/instructions.html

// 5.4 Instructions

impl Binary for Instr {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        use Instr::*;

        match self {
            // 5.4.1 Parametric Instructions
            Unreachable => emit!(b, 0x00),
            Nop => emit!(b, 0x01),
            Drop => emit!(b, 0x1A),
            Select(None) => emit!(b, 0x1B),
            Select(Some(t)) => emit!(b, 0x1C, t),

            // 5.4.2 Control Instructions
            Block(bt, in_) => todo!(),
            Loop(bt, in_) => todo!(),
            IfElse(bt, in1, in2) => todo!(),
            Throw(x) => todo!(),
            ThrowRef => todo!(),
            Br(l) => todo!(),
            BrIf(l) => todo!(),
            BrTable(l, ln) => todo!(),
            Return => emit!(b, 0x0F),
            Call(x) => emit!(b, 0x10, x),
            CallIndirect(x, TypeUse(y)) => emit!(b, 0x11, y, x),
            ReturnCall(x) => todo!(),
            ReturnCallIndirect(x, y) => todo!(),
            CallRef(x) => todo!(),
            ReturnCallRef(y) => todo!(),
            TryTable(bt, c, in_) => todo!(),
            BrOnNull(l) => todo!(),
            BrOnNonNull(l) => todo!(),
            BrOnCast(l, RefType(null1, ht1), RefType(null2, ht2)) => todo!(),
            BrOnCastFail(l, RefType(null1, ht1), RefType(null2, ht2)) => todo!(),

            // 5.4.3 Variable Instructions
            LocalGet(x) => emit!(b, 0x20, x),
            LocalSet(x) => emit!(b, 0x21, x),
            LocalTee(x) => emit!(b, 0x22, x),
            GlobalGet(x) => emit!(b, 0x23, x),
            GlobalSet(x) => emit!(b, 0x24, x),

            // 5.4.4 Table Instructions
            // TODO

            // 5.4.5 Memory Instructions
            I32Load(x, a0) => emit!(b, 0x28, (x, a0)),
            I64Load(x, a0) => emit!(b, 0x29, (x, a0)),
            F32Load(x, a0) => emit!(b, 0x2A, (x, a0)),
            F64Load(x, a0) => emit!(b, 0x2B, (x, a0)),
            I32Load8S(x, a0) => emit!(b, 0x2C, (x, a0)),
            I32Load8U(x, a0) => emit!(b, 0x2D, (x, a0)),
            I32Load16S(x, a0) => emit!(b, 0x2E, (x, a0)),
            I32Load16U(x, a0) => emit!(b, 0x2F, (x, a0)),
            I64Load8S(x, a0) => emit!(b, 0x30, (x, a0)),
            I64Load8U(x, a0) => emit!(b, 0x31, (x, a0)),
            I64Load16S(x, a0) => emit!(b, 0x32, (x, a0)),
            I64Load16U(x, a0) => emit!(b, 0x33, (x, a0)),
            I64Load32S(x, a0) => emit!(b, 0x34, (x, a0)),
            I64Load32U(x, a0) => emit!(b, 0x35, (x, a0)),
            I32Store(x, a0) => emit!(b, 0x36, (x, a0)),
            I64Store(x, a0) => emit!(b, 0x37, (x, a0)),
            F32Store(x, a0) => emit!(b, 0x38, (x, a0)),
            F64Store(x, a0) => emit!(b, 0x39, (x, a0)),
            I32Store8(x, a0) => emit!(b, 0x3A, (x, a0)),
            I32Store16(x, a0) => emit!(b, 0x3B, (x, a0)),
            I64Store8(x, a0) => emit!(b, 0x3C, (x, a0)),
            I64Store16(x, a0) => emit!(b, 0x3D, (x, a0)),
            I64Store32(x, a0) => emit!(b, 0x3E, (x, a0)),
            MemorySize(x) => emit!(b, 0x3F, x),
            MemoryGrow(x) => emit!(b, 0x40, x),
            MemoryInit(x, y) => emit!(b, 0xFC, U32(8), y, x),
            DataDrop(x) => emit!(b, 0xFC, U32(9), x),
            MemoryCopy(x1, x2) => emit!(b, 0xFC, U32(10), x1, x2),
            MemoryFill(x) => emit!(b, 0xFC, U32(11), x),

            // 5.4.2 Reference Instructions
            RefNull(ht) => emit!(b, 0xD0, ht),
            RefIsNull => emit!(b, 0xD1),
            RefFunc(x) => emit!(b, 0xD2, x),
            RefEq => emit!(b, 0xD3),
            RefAsNonNull => emit!(b, 0xD4),
            RefTest(RefType(None, ht)) => emit!(b, 0xFB, U32(20), ht),
            RefTest(RefType(Some(Null), ht)) => emit!(b, 0xFB, U32(21), ht),
            RefCast(RefType(None, ht)) => emit!(b, 0xFB, U32(22), ht),
            RefCast(RefType(Some(Null), ht)) => emit!(b, 0xFB, U32(23), ht),

            // 5.4.7 Aggregate Instructions
            StructNew(x) => emit!(b, 0xFB, U32(0), x),
            StructNewDefault(x) => emit!(b, 0xFB, U32(1), x),
            StructGet(x, i) => emit!(b, 0xFB, U32(2), x, U32(*i)),
            StructGetS(x, i) => emit!(b, 0xFB, U32(3), x, U32(*i)),
            StructGetU(x, i) => emit!(b, 0xFB, U32(4), x, U32(*i)),
            StructSet(x, i) => emit!(b, 0xFB, U32(5), x, U32(*i)),
            ArrayNew(x) => emit!(b, 0xFB, U32(6), x),
            ArrayNewDefault(x) => emit!(b, 0xFB, U32(7), x),
            ArrayNewFixed(x, n) => emit!(b, 0xFB, U32(8), x, U32(*n)),
            ArrayNewData(x, y) => emit!(b, 0xFB, U32(9), x, y),
            ArrayNewElem(x, y) => emit!(b, 0xFB, U32(10), x, y),
            ArrayGet(x) => emit!(b, 0xFB, U32(11), x),
            ArrayGetS(x) => emit!(b, 0xFB, U32(12), x),
            ArrayGetU(x) => emit!(b, 0xFB, U32(13), x),
            ArraySet(x) => emit!(b, 0xFB, U32(14), x),
            ArrayLen => emit!(b, 0xFB, U32(15)),
            ArrayFill(x) => emit!(b, 0xFB, U32(16), x),
            ArrayCopy(x1, x2) => emit!(b, 0xFB, U32(17), x1, x2),
            ArrayInitData(x, y) => emit!(b, 0xFB, U32(18), x, y),
            ArrayInitElem(x, y) => emit!(b, 0xFB, U32(19), x, y),
            AnyConvertExtern => emit!(b, 0xFB, U32(26)),
            ExternConvertAny => emit!(b, 0xFB, U32(27)),
            RefI31 => emit!(b, 0xFB, U32(28)),
            I31GetS => emit!(b, 0xFB, U32(29)),
            I31GetU => emit!(b, 0xFB, U32(30)),

            // 5.4.8 Numeric Instructions
            I32Const(n) => emit!(b, 0x41, U32(*n)),
            I64Const(n) => emit!(b, 0x42, U64(*n)),
            F32Const(z) => emit!(b, 0x43, F32(*z)),
            F64Const(z) => emit!(b, 0x44, F64(*z)),

            I32Eqz => emit!(b, 0x45),
            I32Eq => emit!(b, 0x46),
            I32Ne => emit!(b, 0x47),
            I32LtS => emit!(b, 0x48),
            I32LtU => emit!(b, 0x49),
            I32GtS => emit!(b, 0x4A),
            I32GtU => emit!(b, 0x4B),
            I32LeS => emit!(b, 0x4C),
            I32LeU => emit!(b, 0x4D),
            I32GeS => emit!(b, 0x4E),
            I32GeU => emit!(b, 0x4F),

            I64Eqz => emit!(b, 0x50),
            I64Eq => emit!(b, 0x51),
            I64Ne => emit!(b, 0x52),
            I64LtS => emit!(b, 0x53),
            I64LtU => emit!(b, 0x54),
            I64GtS => emit!(b, 0x55),
            I64GtU => emit!(b, 0x56),
            I64LeS => emit!(b, 0x57),
            I64LeU => emit!(b, 0x58),
            I64GeS => emit!(b, 0x59),
            I64GeU => emit!(b, 0x5A),

            F32Eq => emit!(b, 0x5B),
            F32Ne => emit!(b, 0x5C),
            F32Lt => emit!(b, 0x5D),
            F32Gt => emit!(b, 0x5E),
            F32Le => emit!(b, 0x5F),
            F32Ge => emit!(b, 0x60),

            F64Eq => emit!(b, 0x61),
            F64Ne => emit!(b, 0x62),
            F64Lt => emit!(b, 0x63),
            F64Gt => emit!(b, 0x64),
            F64Le => emit!(b, 0x65),
            F64Ge => emit!(b, 0x66),

            I32Clz => emit!(b, 0x67),
            I32Ctz => emit!(b, 0x68),
            I32Popcnt => emit!(b, 0x69),
            I32Add => emit!(b, 0x6A),
            I32Sub => emit!(b, 0x6B),
            I32Mul => emit!(b, 0x6C),
            I32DivS => emit!(b, 0x6D),
            I32DivU => emit!(b, 0x6E),
            I32RemS => emit!(b, 0x6F),
            I32RemU => emit!(b, 0x70),
            I32And => emit!(b, 0x71),
            I32Or => emit!(b, 0x72),
            I32Xor => emit!(b, 0x73),
            I32Shl => emit!(b, 0x74),
            I32ShrS => emit!(b, 0x75),
            I32ShrU => emit!(b, 0x76),
            I32Rotl => emit!(b, 0x77),
            I32Rotr => emit!(b, 0x78),

            I64Clz => emit!(b, 0x79),
            I64Ctz => emit!(b, 0x7A),
            I64Popcnt => emit!(b, 0x7B),
            I64Add => emit!(b, 0x7C),
            I64Sub => emit!(b, 0x7D),
            I64Mul => emit!(b, 0x7E),
            I64DivS => emit!(b, 0x7F),
            I64DivU => emit!(b, 0x80),
            I64RemS => emit!(b, 0x81),
            I64RemU => emit!(b, 0x82),
            I64And => emit!(b, 0x83),
            I64Or => emit!(b, 0x84),
            I64Xor => emit!(b, 0x85),
            I64Shl => emit!(b, 0x86),
            I64ShrS => emit!(b, 0x87),
            I64ShrU => emit!(b, 0x88),
            I64Rotl => emit!(b, 0x89),
            I64Rotr => emit!(b, 0x8A),

            F32Abs => emit!(b, 0x8B),
            F32Neg => emit!(b, 0x8C),
            F32Ceil => emit!(b, 0x8D),
            F32Floor => emit!(b, 0x8E),
            F32Trunc => emit!(b, 0x8F),
            F32Nearest => emit!(b, 0x90),
            F32Sqrt => emit!(b, 0x91),
            F32Add => emit!(b, 0x92),
            F32Sub => emit!(b, 0x93),
            F32Mul => emit!(b, 0x94),
            F32Div => emit!(b, 0x95),
            F32Min => emit!(b, 0x96),
            F32Max => emit!(b, 0x97),
            F32Copysign => emit!(b, 0x98),

            F64Abs => emit!(b, 0x99),
            F64Neg => emit!(b, 0x9A),
            F64Ceil => emit!(b, 0x9B),
            F64Floor => emit!(b, 0x9C),
            F64Trunc => emit!(b, 0x9D),
            F64Nearest => emit!(b, 0x9E),
            F64Sqrt => emit!(b, 0x9F),
            F64Add => emit!(b, 0xA0),
            F64Sub => emit!(b, 0xA1),
            F64Mul => emit!(b, 0xA2),
            F64Div => emit!(b, 0xA3),
            F64Min => emit!(b, 0xA4),
            F64Max => emit!(b, 0xA5),
            F64Copysign => emit!(b, 0xA6),

            I32WrapI64 => emit!(b, 0xA7),
            I32TruncF32S => emit!(b, 0xA8),
            I32TruncF32U => emit!(b, 0xA9),
            I32TruncF64S => emit!(b, 0xAA),
            I32TruncF64U => emit!(b, 0xAB),
            I64ExtendI32S => emit!(b, 0xAC),
            I64ExtendI32U => emit!(b, 0xAD),
            I64TruncF32S => emit!(b, 0xAE),
            I64TruncF32U => emit!(b, 0xAF),
            I64TruncF64S => emit!(b, 0xB0),
            I64TruncF64U => emit!(b, 0xB1),
            F32ConvertI32S => emit!(b, 0xB2),
            F32ConvertI32U => emit!(b, 0xB3),
            F32ConvertI64S => emit!(b, 0xB4),
            F32ConvertI64U => emit!(b, 0xB5),
            F32DemoteF64 => emit!(b, 0xB6),
            F64ConvertI32S => emit!(b, 0xB7),
            F64ConvertI32U => emit!(b, 0xB8),
            F64ConvertI64S => emit!(b, 0xB9),
            F64ConvertI64U => emit!(b, 0xBA),
            F64PromoteF32 => emit!(b, 0xBB),
            I32ReinterpretF32 => emit!(b, 0xBC),
            I64ReinterpretF64 => emit!(b, 0xBD),
            F32ReinterpretI32 => emit!(b, 0xBE),
            F64ReinterpretI64 => emit!(b, 0xBF),

            I32Extend8S => emit!(b, 0xC0),
            I32Extend16S => emit!(b, 0xC1),
            I64Extend8S => emit!(b, 0xC2),
            I64Extend16S => emit!(b, 0xC3),
            I64Extend32S => emit!(b, 0xC4),

            I32TruncSatF32S => emit!(b, 0xFC, U32(0)),
            I32TruncSatF32U => emit!(b, 0xFC, U32(1)),
            I32TruncSatF64S => emit!(b, 0xFC, U32(2)),
            I32TruncSatF64U => emit!(b, 0xFC, U32(3)),
            I64TruncSatF32S => emit!(b, 0xFC, U32(4)),
            I64TruncSatF32U => emit!(b, 0xFC, U32(5)),
            I64TruncSatF64S => emit!(b, 0xFC, U32(6)),
            I64TruncSatF64U => emit!(b, 0xFC, U32(7)),
            // 5.4.9 Vector Instructions
            // TODO
        }
    }
}

impl Binary for (&MemIdx, &MemArg) {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match *self {
            (
                MemIdx(0),
                &MemArg {
                    align: n,
                    offset: m,
                },
            ) => {
                b.emit(U32(n))?;
                b.emit(U64(m))?;
            }
            (
                MemIdx(x),
                &MemArg {
                    align: n,
                    offset: m,
                },
            ) => {
                b.emit(U32(n + (1 << 6)))?;
                b.emit(U64(m))?;
            }
        }
        Ok(())
    }
}

// 5.4.9 Expressions

impl Binary for Expr {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        for instr in self.0.iter() {
            b.emit(instr)?;
        }
        b.emit(0x0B)?;
        Ok(())
    }
}
