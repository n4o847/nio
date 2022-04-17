use nio::{codegen::CodeGenerator, parser, typecheck};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(source: &str) -> Result<String, String> {
    let result = parser::parse(source).map_err(|err| format!("Error: {}", err))?;
    Ok(format!("{:?}", result))
}

#[wasm_bindgen]
pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let program = parser::parse(&source).map_err(|err| format!("Error: {}", err))?;

    let mut program = program.into();

    typecheck::typecheck(&mut program).map_err(|err| format!("Error: {}", err))?;

    let module = CodeGenerator::generate(&program).map_err(|err| format!("Error: {}", err))?;

    let mut target = Vec::new();
    nio::wasm::emit(&mut target, &module).map_err(|err| format!("Error: {}", err))?;

    Ok(target)
}
