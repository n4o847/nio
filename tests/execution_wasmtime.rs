use std::error;

use indoc::indoc;
use wasmtime::{Engine, Instance, Linker, Module, Store};

#[test]
fn test_wasmtime_add() -> Result<(), Box<dyn error::Error>> {
    let nio_code = indoc! {r#"
        @export("add") def add(x: Int, y: Int): Int = x + y
    "#};

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

#[test]
fn test_wasmtime_import() -> Result<(), Box<dyn error::Error>> {
    let nio_code = indoc! {r#"
        @import("ffi", "one") def one(): Int
        @export("inc") def inc(x: Int): Int = x + one()
    "#};

    let wasm_bytes = nio::compiler::compile(nio_code)?.to_bytes()?;

    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytes)?;
    let mut store = Store::new(&engine, ());

    let mut linker = Linker::new(&engine);
    linker.func_wrap("ffi", "one", || 1)?;

    let instance = linker.instantiate(&mut store, &module)?;

    let inc = instance.get_typed_func::<i32, i32>(&mut store, "inc")?;

    let result = inc.call(&mut store, 3)?;
    assert_eq!(result, 4);

    Ok(())
}

#[test]
fn test_wasmtime_let() -> Result<(), Box<dyn error::Error>> {
    let nio_code = indoc! {r#"
        @export("func") def func(x: Int, y: Int): Int = {
            let z: Int = 10
            x + y + z
        }
    "#};

    let wasm_bytes = nio::compiler::compile(nio_code)?.to_bytes()?;

    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytes)?;
    let mut store = Store::new(&engine, ());

    let instance = Instance::new(&mut store, &module, &[])?;

    let func = instance.get_typed_func::<(i32, i32), i32>(&mut store, "func")?;

    let result = func.call(&mut store, (3, 4))?;
    assert_eq!(result, 17);

    Ok(())
}
