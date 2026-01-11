use super::*;
use crate::binary::values::U32;

// https://webassembly.github.io/spec/core/binary/conventions.html

// 5.1 Conventions

// 5.1.3 Lists

impl<T: Binary> Binary for Vec<T> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        b.emit(U32(self.len() as u32))?;
        for el in self.iter() {
            b.emit(el)?;
        }
        Ok(())
    }
}
