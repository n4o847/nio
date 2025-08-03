pub mod ast_to_ir;
pub mod codegen;
pub mod compiler;
pub mod ir;
pub mod typecheck;

pub use nio_parser as parser;
pub use nio_wasm as wasm;
