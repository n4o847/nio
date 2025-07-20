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
    ($e:ident; i32($x:expr) $(, $($t:tt)*)?) => {
        $e.write_i32($x)?;
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

impl Emitter<'_> {
    // Instructions
    fn emit_instr(&mut self, instr: &Instr) -> io::Result<()> {
        use Instr::*;

        let buffer = match instr {
            // Control Instructions
            Unreachable => bin![0x00],
            Nop => bin![0x01],
            Block(b, i) => todo!(),
            Loop(b, i) => todo!(),
            IfElse(b, i1, i2) => todo!(),
            Br(l) => todo!(),
            BrIf(l) => todo!(),
            BrTable(ls, l) => todo!(),
            Return => bin![0x0f],
            Call(x) => todo!(),
            CallIndirect(x) => todo!(),

            // Parametric Instructions
            Drop => bin![0x1a],
            Select => bin![0x1b],

            // Variable Instructions
            LocalGet(x) => bin![0x20, u32(x.0)],
            LocalSet(x) => bin![0x21, u32(x.0)],
            LocalTee(x) => bin![0x22, u32(x.0)],
            GlobalGet(x) => bin![0x23, u32(x.0)],
            GlobalSet(x) => bin![0x24, u32(x.0)],

            // Memory Instructions
            I32Load(m) => bin![0x28, u32(m.align), u32(m.offset)],
            I64Load(m) => bin![0x29, u32(m.align), u32(m.offset)],
            F32Load(m) => bin![0x2a, u32(m.align), u32(m.offset)],
            F64Load(m) => bin![0x2b, u32(m.align), u32(m.offset)],
            I32Load8S(m) => bin![0x2c, u32(m.align), u32(m.offset)],
            I32Load8U(m) => bin![0x2d, u32(m.align), u32(m.offset)],
            I32Load16S(m) => bin![0x2e, u32(m.align), u32(m.offset)],
            I32Load16U(m) => bin![0x2f, u32(m.align), u32(m.offset)],
            I64Load8S(m) => bin![0x30, u32(m.align), u32(m.offset)],
            I64Load8U(m) => bin![0x31, u32(m.align), u32(m.offset)],
            I64Load16S(m) => bin![0x32, u32(m.align), u32(m.offset)],
            I64Load16U(m) => bin![0x33, u32(m.align), u32(m.offset)],
            I64Load32S(m) => bin![0x34, u32(m.align), u32(m.offset)],
            I64Load32U(m) => bin![0x35, u32(m.align), u32(m.offset)],
            I32Store(m) => bin![0x36, u32(m.align), u32(m.offset)],
            I64Store(m) => bin![0x37, u32(m.align), u32(m.offset)],
            F32Store(m) => bin![0x38, u32(m.align), u32(m.offset)],
            F64Store(m) => bin![0x39, u32(m.align), u32(m.offset)],
            I32Store8(m) => bin![0x3a, u32(m.align), u32(m.offset)],
            I32Store16(m) => bin![0x3b, u32(m.align), u32(m.offset)],
            I64Store8(m) => bin![0x3c, u32(m.align), u32(m.offset)],
            I64Store16(m) => bin![0x3d, u32(m.align), u32(m.offset)],
            I64Store32(m) => bin![0x3e, u32(m.align), u32(m.offset)],
            MemorySize => bin![0x3f, 0x00],
            MemoryGrow => bin![0x40, 0x00],

            // Numeric Instructions
            I32Const(n) => bin![0x41, i32(*n)],
            I64Const(n) => todo!(),
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

            I32TruncSatF32S => bin![0xfc, 0],
            I32TruncSatF32U => bin![0xfc, 1],
            I32TruncSatF64S => bin![0xfc, 2],
            I32TruncSatF64U => bin![0xfc, 3],
            I64TruncSatF32S => bin![0xfc, 4],
            I64TruncSatF32U => bin![0xfc, 5],
            I64TruncSatF64S => bin![0xfc, 6],
            I64TruncSatF64U => bin![0xfc, 7],
        };

        self.write(&buffer)?;

        Ok(())
    }

    // Expressions
    pub fn emit_expr(&mut self, expr: &Expr) -> io::Result<()> {
        for instr in expr.0.iter() {
            self.emit_instr(&instr)?;
        }

        self.write(&[0x0b])?;

        Ok(())
    }
}
