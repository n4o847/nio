pub mod ast_to_ir;
pub mod codegen;
pub mod ir;
pub use nio_parser as parser;
pub mod typecheck;
pub use nio_wasm as wasm;
