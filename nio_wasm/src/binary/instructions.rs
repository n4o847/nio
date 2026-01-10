#![allow(unused_variables)]

use super::super::syntax::*;
use super::*;

// https://webassembly.github.io/spec/core/binary/instructions.html

macro_rules! bin {
    ($e:ident;) => {};
    ($e:ident; u32($x:expr) $(, $($t:tt)*)?) => {
        $e.write_u32($x)?;
        $(
            bin![$e; $($t)*];
        )?
    };
    ($e:ident; u64($x:expr) $(, $($t:tt)*)?) => {
        $e.write_u64($x)?;
        $(
            bin![$e; $($t)*];
        )?
    };
    ($e:ident; f32($x:expr) $(, $($t:tt)*)?) => {
        $e.write_f32($x)?;
        $(
            bin![$e; $($t)*];
        )?
    };
    ($e:ident; f64($x:expr) $(, $($t:tt)*)?) => {
        $e.write_f64($x)?;
        $(
            bin![$e; $($t)*];
        )?
    };
    ($e:ident; $emit:ident($($x:expr),*) $(, $($t:tt)*)?) => {
        $e.$emit($($x),*)?;
        $(
            bin![$e; $($t)*];
        )?
    };
    ($e:ident; $b:expr $(, $($t:tt)*)?) => {
        $e.write(&[$b])?;
        $(
            bin![$e; $($t)*];
        )?
    };
    ($($t:tt)*) => {
        {
            let mut buffer = Vec::new();
            let mut emitter = Emitter::new(&mut buffer);
            bin![emitter; $($t)*];
            buffer
        }
    };
}

// 5.4 Instructions

impl<W: io::Write> Emitter<W> {
    fn emit_instr(&mut self, instr: &Instr) -> io::Result<()> {
        use Instr::*;

        let buffer = match instr {
            // 5.4.1 Parametric Instructions
            Unreachable => bin![0x00],
            Nop => bin![0x01],
            Drop => bin![0x1a],
            Select(None) => bin![0x1b],
            Select(Some(t)) => bin![0x1c, write_list(t, |e, val_type| e.emit_val_type(val_type))],

            // 5.4.2 Control Instructions
            Block(bt, in_) => todo!(),
            Loop(bt, in_) => todo!(),
            IfElse(bt, in1, in2) => todo!(),
            Throw(x) => todo!(),
            ThrowRef => todo!(),
            Br(l) => todo!(),
            BrIf(l) => todo!(),
            BrTable(l, ln) => todo!(),
            Return => bin![0x0f],
            Call(x) => bin![0x10, u32(x.0)],
            CallIndirect(x, TypeUse(y)) => bin![0x11, u32(y.0), u32(x.0)],
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
            LocalGet(x) => bin![0x20, u32(x.0)],
            LocalSet(x) => bin![0x21, u32(x.0)],
            LocalTee(x) => bin![0x22, u32(x.0)],
            GlobalGet(x) => bin![0x23, u32(x.0)],
            GlobalSet(x) => bin![0x24, u32(x.0)],

            // 5.4.4 Table Instructions
            // TODO

            // 5.4.5 Memory Instructions
            I32Load(x, a0) => bin![0x28, emit_mem_arg(x, a0)],
            I64Load(x, a0) => bin![0x29, emit_mem_arg(x, a0)],
            F32Load(x, a0) => bin![0x2a, emit_mem_arg(x, a0)],
            F64Load(x, a0) => bin![0x2b, emit_mem_arg(x, a0)],
            I32Load8S(x, a0) => bin![0x2c, emit_mem_arg(x, a0)],
            I32Load8U(x, a0) => bin![0x2d, emit_mem_arg(x, a0)],
            I32Load16S(x, a0) => bin![0x2e, emit_mem_arg(x, a0)],
            I32Load16U(x, a0) => bin![0x2f, emit_mem_arg(x, a0)],
            I64Load8S(x, a0) => bin![0x30, emit_mem_arg(x, a0)],
            I64Load8U(x, a0) => bin![0x31, emit_mem_arg(x, a0)],
            I64Load16S(x, a0) => bin![0x32, emit_mem_arg(x, a0)],
            I64Load16U(x, a0) => bin![0x33, emit_mem_arg(x, a0)],
            I64Load32S(x, a0) => bin![0x34, emit_mem_arg(x, a0)],
            I64Load32U(x, a0) => bin![0x35, emit_mem_arg(x, a0)],
            I32Store(x, a0) => bin![0x36, emit_mem_arg(x, a0)],
            I64Store(x, a0) => bin![0x37, emit_mem_arg(x, a0)],
            F32Store(x, a0) => bin![0x38, emit_mem_arg(x, a0)],
            F64Store(x, a0) => bin![0x39, emit_mem_arg(x, a0)],
            I32Store8(x, a0) => bin![0x3a, emit_mem_arg(x, a0)],
            I32Store16(x, a0) => bin![0x3b, emit_mem_arg(x, a0)],
            I64Store8(x, a0) => bin![0x3c, emit_mem_arg(x, a0)],
            I64Store16(x, a0) => bin![0x3d, emit_mem_arg(x, a0)],
            I64Store32(x, a0) => bin![0x3e, emit_mem_arg(x, a0)],
            MemorySize(x) => bin![0x3f, u32(x.0)],
            MemoryGrow(x) => bin![0x40, u32(x.0)],
            MemoryInit(x, y) => bin![0xfc, u32(8), u32(y.0), u32(x.0)],
            DataDrop(x) => bin![0xfc, u32(9), u32(x.0)],
            MemoryCopy(x1, x2) => bin![0xfc, u32(10), u32(x1.0), u32(x2.0)],
            MemoryFill(x) => bin![0xfc, u32(11), u32(x.0)],

            // 5.4.2 Reference Instructions
            RefNull(ht) => bin![0xd0, emit_heap_type(ht)],
            RefIsNull => bin![0xd1],
            RefFunc(x) => bin![0xd2, u32(x.0)],
            RefEq => bin![0xd3],
            RefAsNonNull => bin![0xd4],
            RefTest(RefType(None, ht)) => bin![0xfb, u32(20), emit_heap_type(ht)],
            RefTest(RefType(Some(Null), ht)) => bin![0xfb, u32(21), emit_heap_type(ht)],
            RefCast(RefType(None, ht)) => bin![0xfb, u32(22), emit_heap_type(ht)],
            RefCast(RefType(Some(Null), ht)) => bin![0xfb, u32(23), emit_heap_type(ht)],

            // 5.4.7 Aggregate Instructions
            StructNew(x) => bin![0xfb, u32(0), u32(x.0)],
            StructNewDefault(x) => bin![0xfb, u32(1), u32(x.0)],
            StructGet(x, i) => bin![0xfb, u32(2), u32(x.0), u32(*i)],
            StructGetS(x, i) => bin![0xfb, u32(3), u32(x.0), u32(*i)],
            StructGetU(x, i) => bin![0xfb, u32(4), u32(x.0), u32(*i)],
            StructSet(x, i) => bin![0xfb, u32(5), u32(x.0), u32(*i)],
            ArrayNew(x) => bin![0xfb, u32(6), u32(x.0)],
            ArrayNewDefault(x) => bin![0xfb, u32(7), u32(x.0)],
            ArrayNewFixed(x, n) => bin![0xfb, u32(8), u32(x.0), u32(*n)],
            ArrayNewData(x, y) => bin![0xfb, u32(9), u32(x.0), u32(y.0)],
            ArrayNewElem(x, y) => bin![0xfb, u32(10), u32(x.0), u32(y.0)],
            ArrayGet(x) => bin![0xfb, u32(11), u32(x.0)],
            ArrayGetS(x) => bin![0xfb, u32(12), u32(x.0)],
            ArrayGetU(x) => bin![0xfb, u32(13), u32(x.0)],
            ArraySet(x) => bin![0xfb, u32(14), u32(x.0)],
            ArrayLen => bin![0xfb, u32(15)],
            ArrayFill(x) => bin![0xfb, u32(16), u32(x.0)],
            ArrayCopy(x1, x2) => bin![0xfb, u32(17), u32(x1.0), u32(x2.0)],
            ArrayInitData(x, y) => bin![0xfb, u32(18), u32(x.0), u32(y.0)],
            ArrayInitElem(x, y) => bin![0xfb, u32(19), u32(x.0), u32(y.0)],
            AnyConvertExtern => bin![0xfb, u32(26)],
            ExternConvertAny => bin![0xfb, u32(27)],
            RefI31 => bin![0xfb, u32(28)],
            I31GetS => bin![0xfb, u32(29)],
            I31GetU => bin![0xfb, u32(30)],

            // 5.4.8 Numeric Instructions
            I32Const(n) => bin![0x41, u32(*n)],
            I64Const(n) => bin![0x42, u64(*n)],
            F32Const(z) => bin![0x43, f32(*z)],
            F64Const(z) => bin![0x44, f64(*z)],

            I32Eqz => bin![0x45],
            I32Eq => bin![0x46],
            I32Ne => bin![0x47],
            I32LtS => bin![0x48],
            I32LtU => bin![0x49],
            I32GtS => bin![0x4a],
            I32GtU => bin![0x4b],
            I32LeS => bin![0x4c],
            I32LeU => bin![0x4d],
            I32GeS => bin![0x4e],
            I32GeU => bin![0x4f],

            I64Eqz => bin![0x50],
            I64Eq => bin![0x51],
            I64Ne => bin![0x52],
            I64LtS => bin![0x53],
            I64LtU => bin![0x54],
            I64GtS => bin![0x55],
            I64GtU => bin![0x56],
            I64LeS => bin![0x57],
            I64LeU => bin![0x58],
            I64GeS => bin![0x59],
            I64GeU => bin![0x5a],

            F32Eq => bin![0x5b],
            F32Ne => bin![0x5c],
            F32Lt => bin![0x5d],
            F32Gt => bin![0x5e],
            F32Le => bin![0x5f],
            F32Ge => bin![0x60],

            F64Eq => bin![0x61],
            F64Ne => bin![0x62],
            F64Lt => bin![0x63],
            F64Gt => bin![0x64],
            F64Le => bin![0x65],
            F64Ge => bin![0x66],

            I32Clz => bin![0x67],
            I32Ctz => bin![0x68],
            I32Popcnt => bin![0x69],
            I32Add => bin![0x6a],
            I32Sub => bin![0x6b],
            I32Mul => bin![0x6c],
            I32DivS => bin![0x6d],
            I32DivU => bin![0x6e],
            I32RemS => bin![0x6f],
            I32RemU => bin![0x70],
            I32And => bin![0x71],
            I32Or => bin![0x72],
            I32Xor => bin![0x73],
            I32Shl => bin![0x74],
            I32ShrS => bin![0x75],
            I32ShrU => bin![0x76],
            I32Rotl => bin![0x77],
            I32Rotr => bin![0x78],

            I64Clz => bin![0x79],
            I64Ctz => bin![0x7a],
            I64Popcnt => bin![0x7b],
            I64Add => bin![0x7c],
            I64Sub => bin![0x7d],
            I64Mul => bin![0x7e],
            I64DivS => bin![0x7f],
            I64DivU => bin![0x80],
            I64RemS => bin![0x81],
            I64RemU => bin![0x82],
            I64And => bin![0x83],
            I64Or => bin![0x84],
            I64Xor => bin![0x85],
            I64Shl => bin![0x86],
            I64ShrS => bin![0x87],
            I64ShrU => bin![0x88],
            I64Rotl => bin![0x89],
            I64Rotr => bin![0x8a],

            F32Abs => bin![0x8b],
            F32Neg => bin![0x8c],
            F32Ceil => bin![0x8d],
            F32Floor => bin![0x8e],
            F32Trunc => bin![0x8f],
            F32Nearest => bin![0x90],
            F32Sqrt => bin![0x91],
            F32Add => bin![0x92],
            F32Sub => bin![0x93],
            F32Mul => bin![0x94],
            F32Div => bin![0x95],
            F32Min => bin![0x96],
            F32Max => bin![0x97],
            F32Copysign => bin![0x98],

            F64Abs => bin![0x99],
            F64Neg => bin![0x9a],
            F64Ceil => bin![0x9b],
            F64Floor => bin![0x9c],
            F64Trunc => bin![0x9d],
            F64Nearest => bin![0x9e],
            F64Sqrt => bin![0x9f],
            F64Add => bin![0xa0],
            F64Sub => bin![0xa1],
            F64Mul => bin![0xa2],
            F64Div => bin![0xa3],
            F64Min => bin![0xa4],
            F64Max => bin![0xa5],
            F64Copysign => bin![0xa6],

            I32WrapI64 => bin![0xa7],
            I32TruncF32S => bin![0xa8],
            I32TruncF32U => bin![0xa9],
            I32TruncF64S => bin![0xaa],
            I32TruncF64U => bin![0xab],
            I64ExtendI32S => bin![0xac],
            I64ExtendI32U => bin![0xad],
            I64TruncF32S => bin![0xae],
            I64TruncF32U => bin![0xaf],
            I64TruncF64S => bin![0xb0],
            I64TruncF64U => bin![0xb1],
            F32ConvertI32S => bin![0xb2],
            F32ConvertI32U => bin![0xb3],
            F32ConvertI64S => bin![0xb4],
            F32ConvertI64U => bin![0xb5],
            F32DemoteF64 => bin![0xb6],
            F64ConvertI32S => bin![0xb7],
            F64ConvertI32U => bin![0xb8],
            F64ConvertI64S => bin![0xb9],
            F64ConvertI64U => bin![0xba],
            F64PromoteF32 => bin![0xbb],
            I32ReinterpretF32 => bin![0xbc],
            I64ReinterpretF64 => bin![0xbd],
            F32ReinterpretI32 => bin![0xbe],
            F64ReinterpretI64 => bin![0xbf],

            I32Extend8S => bin![0xc0],
            I32Extend16S => bin![0xc1],
            I64Extend8S => bin![0xc2],
            I64Extend16S => bin![0xc3],
            I64Extend32S => bin![0xc4],

            I32TruncSatF32S => bin![0xfc, u32(0)],
            I32TruncSatF32U => bin![0xfc, u32(1)],
            I32TruncSatF64S => bin![0xfc, u32(2)],
            I32TruncSatF64U => bin![0xfc, u32(3)],
            I64TruncSatF32S => bin![0xfc, u32(4)],
            I64TruncSatF32U => bin![0xfc, u32(5)],
            I64TruncSatF64S => bin![0xfc, u32(6)],
            I64TruncSatF64U => bin![0xfc, u32(7)],
            // 5.4.9 Vector Instructions
            // TODO
        };

        self.write(&buffer)?;

        Ok(())
    }

    fn emit_mem_arg(&mut self, mem_idx: &MemIdx, mem_arg: &MemArg) -> io::Result<()> {
        match (mem_idx, mem_arg) {
            (
                MemIdx(0),
                &MemArg {
                    align: n,
                    offset: m,
                },
            ) => {
                self.write_u32(n)?;
                self.write_u64(m)?;
            }
            (
                MemIdx(x),
                &MemArg {
                    align: n,
                    offset: m,
                },
            ) => {
                self.write_u32(n + (1 << 6))?;
                self.write_u64(m)?;
            }
        }

        Ok(())
    }

    // 5.4.9 Expressions
    pub fn emit_expr(&mut self, expr: &Expr) -> io::Result<()> {
        for instr in expr.0.iter() {
            self.emit_instr(&instr)?;
        }

        self.write(&[0x0b])?;

        Ok(())
    }
}
