use std::io;

use thiserror::Error;

use crate::{codegen, typecheck};

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Parse error: {0}")]
    Parse(#[from] nio_parser::ParseError),
    #[error("Type error: {0}")]
    Type(#[from] typecheck::TypeError),
    #[error("Code generation error: {0}")]
    Codegen(#[from] codegen::CodegenError),
}

pub struct Compile {
    module: nio_wasm::Module,
}

pub fn compile(code: &str) -> Result<Compile, CompileError> {
    let program = nio_parser::parse(&code)?;

    let mut program = program.into();

    typecheck::typecheck(&mut program)?;

    let module = codegen::CodeGenerator::generate(&program)?;

    Ok(Compile { module })
}

impl Compile {
    pub fn emit_to(&self, output: impl io::Write) -> io::Result<()> {
        nio_wasm::emit(output, &self.module)
    }

    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.emit_to(&mut bytes)?;
        Ok(bytes)
    }
}
