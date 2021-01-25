use std::io;
use std::io::Write;

// https://webassembly.github.io/spec/core/syntax/modules.html
struct Module {
  types: (),
  funcs: (),
  tables: (),
  mems: (),
  globals: (),
  elem: (),
  data: (),
  start: (),
  imports: (),
  exports: (),
}

fn emit(writer: &mut dyn Write, module: &Module) -> io::Result<()> {
  // https://webassembly.github.io/spec/core/binary/modules.html#binary-module
  let magic = [0x00, 0x61, 0x73, 0x6d];
  writer.write_all(&magic)?;
  let version = [0x01, 0x00, 0x00, 0x00];
  writer.write_all(&version)?;
  Ok(())
}

#[test]
fn test_emit() {
  let module = Module {
    types: (),
    funcs: (),
    tables: (),
    mems: (),
    globals: (),
    elem: (),
    data: (),
    start: (),
    imports: (),
    exports: (),
  };
  let mut buffer = Vec::new();
  let result = emit(&mut buffer, &module);
  assert!(result.is_ok());
  assert_eq!(buffer, &[0, 97, 115, 109, 1, 0, 0, 0]);
}
