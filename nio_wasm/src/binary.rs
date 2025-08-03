mod instructions;
mod modules;
mod types;
mod values;

use std::io;

use super::syntax::Module;

pub fn emit(writer: impl io::Write, module: &Module) -> io::Result<()> {
    let mut emitter = Emitter::new(writer);
    emitter.emit_module(module)
}

struct Emitter<W: io::Write> {
    writer: W,
}

impl<W: io::Write> Emitter<W> {
    fn new(writer: W) -> Emitter<W> {
        Self { writer }
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }
}
