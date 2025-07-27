#![cfg(feature = "test_node")]

use std::{
    error,
    io::Write,
    process::{Command, Stdio},
};

use indoc::indoc;

#[test]
fn test_node_add() -> Result<(), Box<dyn error::Error>> {
    let nio_code = concat! {
        r#"@export("add") def add(x: Int, y: Int): Int = x + y"#,
    };

    let program = nio_parser::parse(nio_code)?;
    let mut program = program.into();
    nio::typecheck::typecheck(&mut program)?;
    let module = nio::codegen::CodeGenerator::generate(&program)?;

    let mut wasm_bytes = Vec::new();
    nio::wasm::emit(&mut wasm_bytes, &module)?;

    let process = Command::new("node")
        .arg("-e")
        .arg(indoc! {r#"
            import assert from "node:assert/strict";
            import { readFile } from "node:fs/promises";

            const bytes = await readFile("/dev/stdin");
            const module = await WebAssembly.compile(bytes);
            const instance = await WebAssembly.instantiate(module);
            const add = instance.exports.add;
            assert.equal(add(3, 4), 7);
        "#})
        .stdin(Stdio::piped())
        .spawn()?;

    process
        .stdin
        .as_ref()
        .ok_or("Failed to open stdin")?
        .write_all(&wasm_bytes)?;

    let output = process.wait_with_output()?;
    if !output.status.success() {
        return Err("Node execution failed".into());
    }

    Ok(())
}
