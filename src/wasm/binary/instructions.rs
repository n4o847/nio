// https://webassembly.github.io/spec/core/binary/instructions.html

use super::super::syntax::instructions::*;
use super::*;

impl Emitter<'_> {
  // Instructions
  fn emit_instr(&mut self, instr: &Instr) -> io::Result<()> {
    use Instr::*;
    match instr {
      // Control Instructions
      Unreachable => self.write(&[0x00]),
      Nop => self.write(&[0x01]),

      // Parametric Instructions
      Drop => self.write(&[0x1a]),
      Select => self.write(&[0x1b]),

      // Variable Instructions
      LocalGet(x) => {
        self.write(&[0x20])?;
        self.write_u32(x.0)
      }
      LocalSet(x) => {
        self.write(&[0x21])?;
        self.write_u32(x.0)
      }
      LocalTee(x) => {
        self.write(&[0x22])?;
        self.write_u32(x.0)
      }
      GlobalGet(x) => {
        self.write(&[0x23])?;
        self.write_u32(x.0)
      }
      GlobalSet(x) => {
        self.write(&[0x24])?;
        self.write_u32(x.0)
      }

      // Memory Instructions
      I32Load(m) => {
        self.write(&[0x28])?;
        self.write_u32(m.align)?;
        self.write_u32(m.offset)
      }
      I64Load(m) => {
        self.write(&[0x29])?;
        self.write_u32(m.align)?;
        self.write_u32(m.offset)
      }
      F32Load(m) => {
        self.write(&[0x2a])?;
        self.write_u32(m.align)?;
        self.write_u32(m.offset)
      }
      F64Load(m) => {
        self.write(&[0x2b])?;
        self.write_u32(m.align)?;
        self.write_u32(m.offset)
      }
      I32Load8S(m) | I32Load8U(m) => {
        self.write(&[match instr {
          I32Load8S(_) => 0x2c,
          I64Load8U(_) => 0x2d,
          _ => unreachable!(),
        }])?;
        self.write_u32(m.align)?;
        self.write_u32(m.offset)
      }
      MemorySize => self.write(&[0x3f, 0x00]),
      MemoryGrow => self.write(&[0x40, 0x00]),

      // Numeric Instructions
      I32Const(n) => {
        self.write(&[0x41])?;
        self.write_i32(*n)
      }
      // I64Const(n) => {}
      F32Const(z) => {
        self.write(&[0x43])?;
        self.write_f32(*z)
      }
      // F64Const(z) => {}

      //   I32Eqz => {}
      //   I32Eq => {}

      //   I64Eqz => {}

      //   F32Eq => {}

      //   F64Eq => {}

      //   I32Clz => {}

      //   I64Clz => {}

      //   F32Abs => {}

      //   F64Abs => {}

      //   I32WrapI64 => {}

      //   I32Extend8S => {}

      //   I32TruncSatF32S => {}
      _ => unimplemented!(),
    }?;

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
