use super::super::syntax::modules::*;
use super::*;

// https://webassembly.github.io/spec/core/binary/modules.html

impl Emitter<'_> {
  fn write_sized<F>(&mut self, f: F) -> io::Result<()>
  where
    F: FnOnce(&mut Emitter) -> io::Result<()>,
  {
    let mut buffer = Vec::new();
    let mut emitter = Emitter::new(&mut buffer);
    f(&mut emitter)?;
    self.write_u32(buffer.len() as u32)?;
    self.write(&buffer)?;
    Ok(())
  }

  // Sections
  fn emit_section<F>(&mut self, id: u8, f: F) -> io::Result<()>
  where
    F: FnOnce(&mut Emitter) -> io::Result<()>,
  {
    self.write(&[id])?;
    self.write_sized(f)?;
    Ok(())
  }

  // Type Section
  fn emit_type_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Import Section
  fn emit_import_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Function Section
  fn emit_func_sec(&mut self, funcs: &Vec<Func>) -> io::Result<()> {
    self.emit_section(3, |e| {
      e.write_u32(funcs.len() as u32)?;
      for func in funcs.iter() {
        e.write_u32(func.r#type.0)?;
      }
      Ok(())
    })
  }

  // Table Section
  fn emit_table_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Memory Section
  fn emit_mem_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Global Section
  fn emit_global_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Export Section
  fn emit_export_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Start Section
  fn emit_start_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Element Section
  fn emit_elem_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Code Section
  fn emit_code_sec(&mut self, funcs: &Vec<Func>) -> io::Result<()> {
    self.emit_section(10, |e| {
      e.write_u32(funcs.len() as u32)?;
      for func in funcs.iter() {
        e.write_sized(|e| {
          let mut chunks = Vec::new();
          for i in 0..func.locals.len() {
            if i == 0 || func.locals[i - 1] != func.locals[i] {
              chunks.push((0, i));
            } else {
              chunks.last_mut().unwrap().0 += 1;
            }
          }

          e.write_u32(chunks.len() as u32)?;
          for chunk in chunks.iter() {
            e.write_u32(chunk.0)?;
            e.emit_val_type(&func.locals[chunk.1])?;
          }

          e.emit_expr(&func.body)?;

          Ok(())
        })?;
      }
      Ok(())
    })
  }

  // Data Section
  fn emit_data_sec(&mut self) -> io::Result<()> {
    Ok(())
  }

  // Modules
  pub fn emit_module(&mut self, module: &Module) -> io::Result<()> {
    let magic = [0x00, 0x61, 0x73, 0x6d];
    self.write(&magic)?;

    let version = [0x01, 0x00, 0x00, 0x00];
    self.write(&version)?;

    // Function Section
    if !module.funcs.is_empty() {
      self.emit_func_sec(&module.funcs)?;
    }

    // Code Section
    if !module.funcs.is_empty() {
      self.emit_code_sec(&module.funcs)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_emit_module() {
    let module = Module::new();
    let mut buffer = Vec::new();
    let mut emitter = Emitter::new(&mut buffer);
    let result = emitter.emit_module(&module);
    assert!(result.is_ok());
    assert_eq!(buffer, &[0, 97, 115, 109, 1, 0, 0, 0]);
  }
}
