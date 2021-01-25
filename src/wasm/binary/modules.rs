use super::*;

impl Emitter<'_> {
  // https://webassembly.github.io/spec/core/binary/modules.html#binary-module
  pub fn emit_module(&mut self, module: &Module) -> io::Result<()> {
    let magic = [0x00, 0x61, 0x73, 0x6d];
    self.write(&magic)?;
    let version = [0x01, 0x00, 0x00, 0x00];
    self.write(&version)?;
    Ok(())
  }
}

#[test]
fn test_emit_module() {
  let module = Module::new();
  let mut buffer = Vec::new();
  let mut emitter = Emitter::new(&mut buffer);
  let result = emitter.emit_module(&module);
  assert!(result.is_ok());
  assert_eq!(buffer, &[0, 97, 115, 109, 1, 0, 0, 0]);
}
