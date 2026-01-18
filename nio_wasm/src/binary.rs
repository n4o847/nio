macro_rules! emit {
    ($dst:expr $(,)?) => {
        Ok(())
    };
    ($dst:expr, $arg:expr $(,)?) => {
        $dst.emit($arg)
    };
    ($dst:expr, $arg:expr $(, $($rest:tt)*)?) => {
        $dst.emit($arg).and_then(|_| emit!($dst, $($($rest)*)?))
    };
}

mod conventions;
mod instructions;
mod modules;
mod types;
mod values;

use std::io;

use super::syntax::Module;
use crate::binary::values::U32;

pub fn emit(writer: impl io::Write, module: &Module) -> io::Result<()> {
    let mut emitter = Emitter::new(writer);
    emitter.emit(module)?;
    Ok(())
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

    fn write_sized<F>(&mut self, f: F) -> io::Result<()>
    where
        F: FnOnce(&mut Emitter<&mut Vec<u8>>) -> io::Result<()>,
    {
        let mut buf = Vec::new();
        let mut emitter = Emitter::new(&mut buf);
        f(&mut emitter)?;
        self.emit(U32(buf.len() as u32))?;
        self.write(&buf)?;
        Ok(())
    }

    fn emit<T: Binary>(&mut self, item: T) -> io::Result<()> {
        item.binary(self)
    }
}

trait Binary {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()>;
}

impl<T: Binary> Binary for &T {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        T::binary(*self, b)
    }
}
