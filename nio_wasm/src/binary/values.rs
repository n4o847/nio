use super::super::syntax::*;
use super::*;

// https://webassembly.github.io/spec/core/binary/values.html

// 5.2 Values

// 5.2.1 Bytes

impl Binary for u8 {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        b.write(&[*self])
    }
}

// 5.2.2 Integers

macro_rules! impl_binary_for_u {
    ($type:ty) => {
        impl Binary for $type {
            fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
                let mut value = self.0;
                loop {
                    if value < (1 << 7) {
                        b.emit(value as u8)?;
                        break;
                    } else {
                        b.emit(value as u8 | (1 << 7))?;
                        value >>= 7;
                    }
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_binary_for_s {
    ($type:ty) => {
        impl Binary for $type {
            fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
                let mut value = self.0;
                loop {
                    if 0 <= value && value < (1 << 6) {
                        b.emit(value as u8)?;
                        break;
                    } else if (-1 << 6) <= value && value < 0 {
                        b.emit(value as u8 & !(1 << 7))?;
                        break;
                    } else {
                        b.emit(value as u8 | (1 << 7))?;
                        value >>= 7;
                    }
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_binary_for_f {
    ($type:ty) => {
        impl Binary for $type {
            fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
                b.write(&self.0.to_le_bytes())
            }
        }
    };
}

pub struct U32(pub u32);
pub struct U64(pub u64);

impl_binary_for_u!(U32);
impl_binary_for_u!(U64);

pub struct S33(pub i64);

impl_binary_for_s!(S33);

pub struct F32(pub f32);
pub struct F64(pub f64);

impl_binary_for_f!(F32);
impl_binary_for_f!(F64);

// 5.2.4 Names

impl Binary for Name {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        b.emit(U32(self.0.len() as u32))?;
        b.write(self.0.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u32() {
        // https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
        let mut buf = Vec::new();
        let mut emitter = Emitter::new(&mut buf);
        let result = emitter.emit(U32(624485));
        assert!(result.is_ok());
        assert_eq!(buf, &[0xe5, 0x8e, 0x26]);
    }

    #[test]
    fn test_write_s33() {
        // https://en.wikipedia.org/wiki/LEB128#Signed_LEB128
        let mut buf = Vec::new();
        let mut emitter = Emitter::new(&mut buf);
        let result = emitter.emit(S33(-123456));
        assert!(result.is_ok());
        assert_eq!(buf, &[0xc0, 0xbb, 0x78]);
    }
}
