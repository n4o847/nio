// https://webassembly.github.io/spec/core/binary/types.html

use super::super::syntax::types::*;
use super::*;

impl Emitter<'_> {
  pub fn emit_val_type(&mut self, val_type: &ValType) -> io::Result<()> {
    self.write(&[match val_type {
      ValType::I32 => 0x7f,
      ValType::I64 => 0x7e,
      ValType::F32 => 0x7d,
      ValType::F64 => 0x7c,
    }])?;
    Ok(())
  }
}
