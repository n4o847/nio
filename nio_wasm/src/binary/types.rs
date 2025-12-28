// https://webassembly.github.io/spec/core/binary/types.html

use super::super::syntax::*;
use super::*;

impl<W: io::Write> Emitter<W> {
    // Value Types
    pub fn emit_val_type(&mut self, val_type: &ValType) -> io::Result<()> {
        match val_type {
            ValType::I32 => self.write(&[0x7F])?,
            ValType::I64 => self.write(&[0x7E])?,
            ValType::F32 => self.write(&[0x7D])?,
            ValType::F64 => self.write(&[0x7C])?,
        }
        Ok(())
    }

    // Result Types
    pub fn emit_result_type(&mut self, result_type: &ResultType) -> io::Result<()> {
        self.write_u32(result_type.0.len() as u32)?;
        for val_type in result_type.0.iter() {
            self.emit_val_type(val_type)?;
        }
        Ok(())
    }

    // Function Types
    pub fn emit_func_type(&mut self, func_type: &FuncType) -> io::Result<()> {
        self.write(&[0x60])?;
        self.emit_result_type(&func_type.0)?;
        self.emit_result_type(&func_type.1)?;
        Ok(())
    }

    // Limits
    pub fn emit_limits(&mut self, limits: &Limits) -> io::Result<()> {
        match limits.max {
            None => {
                self.write(&[0x00])?;
                self.write_u32(limits.min)?;
            }
            Some(max) => {
                self.write(&[0x01])?;
                self.write_u32(limits.min)?;
                self.write_u32(max)?;
            }
        }
        Ok(())
    }

    // Table Types
    pub fn emit_table_type(&mut self, table_type: &TableType) -> io::Result<()> {
        match table_type.1 {
            ElemType::FuncRef => self.write(&[0x70])?,
        }
        self.emit_limits(&table_type.0)?;
        Ok(())
    }
}
