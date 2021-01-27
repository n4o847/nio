use nio::wasm::binary::*;
use nio::wasm::syntax::*;
use std::fs::File;
use std::io;
use std::io::BufWriter;

fn main() -> io::Result<()> {
  let mut module = Module::new();
  module.types.push(FuncType(
    ResultType(vec![ValType::I32, ValType::I32]),
    ResultType(vec![ValType::I32]),
  ));
  module.funcs.push(Func {
    r#type: TypeIdx(0),
    locals: vec![],
    body: Expr(vec![
      Instr::LocalGet(LocalIdx(0)),
      Instr::LocalGet(LocalIdx(1)),
      Instr::I32Add,
    ]),
  });
  module.exports.push(Export {
    name: Name("add".to_string()),
    desc: ExportDesc::Func(FuncIdx(0)),
  });
  let mut buf = BufWriter::new(File::create("examples/add.wasm")?);
  emit(&mut buf, &module)?;
  eprintln!("done");
  Ok(())
}
