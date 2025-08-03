use std::error;

use wasmtime::{Engine, Instance, Module, Store};

#[test]
fn test_wasmtime_add() -> Result<(), Box<dyn error::Error>> {
    let nio_code = concat! {
        r#"@export("add") def add(x: Int, y: Int): Int = x + y"#,
    };

    let wasm_bytes = nio::compiler::compile(nio_code)?.to_bytes()?;

    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytes)?;
    let mut store = Store::new(&engine, ());

    let instance = Instance::new(&mut store, &module, &[])?;

    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;

    let result = add.call(&mut store, (3, 4))?;
    assert_eq!(result, 7);

    Ok(())
}
