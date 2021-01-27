mod instructions;
mod modules;
mod types;
mod values;

use super::syntax::Module;
use std::io;
use std::io::Write;

pub fn emit(writer: &mut dyn Write, module: &Module) -> io::Result<()> {
  let mut emitter = Emitter::new(writer);
  emitter.emit_module(module)?;
  Ok(())
}

struct Emitter<'a> {
  writer: &'a mut dyn Write,
}

impl Emitter<'_> {
  fn new(writer: &mut dyn Write) -> Emitter {
    Emitter { writer }
  }

  fn write(&mut self, buf: &[u8]) -> io::Result<()> {
    self.writer.write_all(buf)
  }
}
