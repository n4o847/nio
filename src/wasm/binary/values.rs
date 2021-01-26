use super::*;

// https://webassembly.github.io/spec/core/binary/values.html

impl Emitter<'_> {
  // Unsigned Integers

  pub fn write_u32(&mut self, mut value: u32) -> io::Result<()> {
    loop {
      if value < (1 << 7) {
        self.write(&[value as u8])?;
        break;
      } else {
        self.write(&[value as u8 | (1 << 7)])?;
        value >>= 7;
      }
    }
    Ok(())
  }

  // Signed Integers

  pub fn write_s32(&mut self, mut value: i32) -> io::Result<()> {
    loop {
      if 0 <= value && value < (1 << 6) {
        self.write(&[value as u8])?;
        break;
      } else if (-1 << 6) <= value && value < 0 {
        self.write(&[value as u8 & !(1 << 7)])?;
        break;
      } else {
        self.write(&[value as u8 | (1 << 7)])?;
        value >>= 7;
      }
    }
    Ok(())
  }

  // Uninterpreted Integers

  pub fn write_i32(&mut self, value: u32) -> io::Result<()> {
    self.write_s32(value as i32)
  }

  // Floating-Point

  pub fn write_f32(&mut self, value: f32) -> io::Result<()> {
    self.write(&value.to_le_bytes())
  }

  pub fn write_f64(&mut self, value: f64) -> io::Result<()> {
    self.write(&value.to_le_bytes())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_write_u32() {
    // https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
    let mut buffer = Vec::new();
    let mut emitter = Emitter::new(&mut buffer);
    let result = emitter.write_u32(624485);
    assert!(result.is_ok());
    assert_eq!(buffer, &[0xe5, 0x8e, 0x26]);
  }

  #[test]
  fn test_write_s32() {
    // https://en.wikipedia.org/wiki/LEB128#Signed_LEB128
    let mut buffer = Vec::new();
    let mut emitter = Emitter::new(&mut buffer);
    let result = emitter.write_s32(-123456);
    assert!(result.is_ok());
    assert_eq!(buffer, &[0xc0, 0xbb, 0x78]);
  }
}
