#![cfg(feature = "test_node")]

use std::{
    error,
    io::Write,
    process::{Command, Stdio},
};

use indoc::indoc;

#[test]
fn test_node_add() -> Result<(), Box<dyn error::Error>> {
    let nio_code = indoc! {r#"
        @export("add") def add(x: Int, y: Int): Int = x + y
    "#};

    let wasm_bytes = nio::compiler::compile(nio_code)?.to_bytes()?;

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
