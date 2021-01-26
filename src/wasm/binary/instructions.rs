// https://webassembly.github.io/spec/core/binary/instructions.html

use super::super::syntax::instructions::*;
use super::*;

macro_rules! bin {
  ($e:ident;) => {};
  ($e:ident; u32($x:expr) $(, $($t:tt)*)?) => {
    $e.write_u32($x)?;
    $(
      bin![$e; $($t)*]
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
      F64Const(z) => todo!(),

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

      // I64Clz => {}

      // F32Abs => {}

      // F64Abs => {}

      // I32WrapI64 => {}

      // I32Extend8S => {}

      // I32TruncSatF32S => {}
      _ => unimplemented!(),
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
