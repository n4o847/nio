use super::*;

// https://webassembly.github.io/spec/core/binary/conventions.html

// 5.1 Conventions

impl<W: io::Write> Emitter<W> {
    // 5.1.3 Lists

    pub fn write_list<T, F>(&mut self, vec: &[T], mut f: F) -> io::Result<()>
    where
        F: FnMut(&mut Emitter<W>, &T) -> io::Result<()>,
    {
        self.write_u32(vec.len() as u32)?;
        for x in vec.iter() {
            f(self, x)?;
        }
        Ok(())
    }
}
