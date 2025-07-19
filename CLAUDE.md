# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Nio is a programming language that compiles to WebAssembly. The project consists of:
- A Rust-based compiler with parser, type checker, and WASM code generator
- A Next.js documentation website with an interactive playground
- Support for functional programming features like lambdas and function calls

## Development Commands

### Core Rust Development
- `cargo build` - Build the Nio compiler
- `cargo test` - Run all tests including parser snapshot tests
- `cargo run -- parse` - Parse Nio code from stdin and display AST
- `cargo run -- compile <input.nio> [-o output.wasm]` - Compile Nio source to WASM

### WebAssembly Build
- `cargo build --release --target wasm32-wasi` or `npm run build:wasm` - Build WASM version of compiler

### Website Development  
- `npm run dev` or `npm run dev -w website` - Start Next.js dev server for documentation website
- `cd website && npm run build` - Build static website
- `cd website && npm run start` - Start production website server

### Testing
- Parser tests use snapshot testing with insta crate
- Test files are in `crates/parser/tests/inputs/*.nio`
- Run `cargo test` to execute all tests

## Architecture

### Compilation Pipeline
1. **Lexer** (`crates/parser/src/lexer.rs`) - Tokenizes Nio source code
2. **Parser** (`crates/parser/src/grammar.lalrpop`) - LALRPOP grammar generates AST
3. **AST to IR** (`src/ast_to_ir.rs`) - Converts AST to intermediate representation
4. **Type Checker** (`src/typecheck.rs`) - Validates types in IR
5. **Code Generator** (`src/codegen.rs`) - Generates WASM module from IR
6. **WASM Emitter** (`crates/wasm/`) - Outputs binary WASM format

### Workspace Structure
- Root crate: Main compiler CLI (`src/main.rs`, `src/lib.rs`)
- `crates/parser/`: Lexer, LALRPOP parser, AST definitions
- `crates/wasm/`: WASM module representation and binary emission
- `website/`: Next.js documentation site with Nextra and Tailwind
- `docs/`: Language documentation including grammar specification

### Key Entry Points
- `src/main.rs` - CLI interface with parse and compile subcommands
- `crates/parser/src/lib.rs` - Main parse function using LALRPOP
- `src/codegen.rs:30` - CodeGenerator::generate() method
- `website/features/playground/` - Interactive playground component

### Language Features
- Integer literals and string literals
- Binary operations (+, -, *, etc.) with precedence
- Lambda expressions: `|x| x + 1`
- Function calls: `a + b(x, y)`
- Type system with type checking phase

## Development Notes

- Uses Rust 2024 edition
- LALRPOP for parser generation with custom lexer
- Snapshot testing for parser validation
- Workspace setup with multiple crates
- Website uses Nextra theme for documentation
- WASM target supports both native and WASI compilation
