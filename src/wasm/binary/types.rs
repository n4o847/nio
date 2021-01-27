// https://webassembly.github.io/spec/core/binary/types.html

use super::super::syntax::*;
use super::*;

impl Emitter<'_> {
  // Value Types
  pub fn emit_val_type(&mut self, val_type: &ValType) -> io::Result<()> {
    self.write(&[match val_type {
      ValType::I32 => 0x7f,
      ValType::I64 => 0x7e,
      ValType::F32 => 0x7d,
      ValType::F64 => 0x7c,
    }])?;
    Ok(())
  }

  // Result Types
  pub fn emit_result_type(&mut self, result_type: &ResultType) -> io::Result<()> {
    self.write_u32(result_type.0.len() as u32)?;
    for val_type in result_type.0.iter() {
      self.emit_val_type(val_type)?;
    }
    Ok(())
  }

  // Function Types
  pub fn emit_func_type(&mut self, func_type: &FuncType) -> io::Result<()> {
    self.write(&[0x60])?;
    self.emit_result_type(&func_type.0)?;
    self.emit_result_type(&func_type.1)?;
    Ok(())
  }
}
