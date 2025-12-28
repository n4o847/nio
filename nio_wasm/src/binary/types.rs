#![allow(dead_code)]

use super::super::syntax::*;
use super::*;

// https://webassembly.github.io/spec/core/binary/types.html

// 5.3 Types

impl<W: io::Write> Emitter<W> {
    // 5.3.1 Number Types
    pub fn emit_num_type(&mut self, num_type: &NumType) -> io::Result<()> {
        match num_type {
            NumType::I32 => self.write(&[0x7F])?,
            NumType::I64 => self.write(&[0x7E])?,
            NumType::F32 => self.write(&[0x7D])?,
            NumType::F64 => self.write(&[0x7C])?,
        }
        Ok(())
    }

    // 5.3.2 Vector Types
    pub fn emit_vec_type(&mut self, vec_type: &VecType) -> io::Result<()> {
        match vec_type {
            VecType::V128 => self.write(&[0x7B])?,
        }
        Ok(())
    }

    // 5.3.3 Reference Types
    pub fn emit_ref_type(&mut self, ref_type: &RefType) -> io::Result<()> {
        match ref_type {
            RefType::FuncRef => self.write(&[0x70])?,
            RefType::ExternRef => self.write(&[0x6F])?,
        }
        Ok(())
    }

    // 5.3.4 Value Types
    pub fn emit_val_type(&mut self, val_type: &ValType) -> io::Result<()> {
        match val_type {
            ValType::NumType(t) => self.emit_num_type(t)?,
            ValType::VecType(t) => self.emit_vec_type(t)?,
            ValType::RefType(t) => self.emit_ref_type(t)?,
        }
        Ok(())
    }

    // 5.3.5 Result Types
    pub fn emit_result_type(&mut self, result_type: &ResultType) -> io::Result<()> {
        self.write_vec(&result_type.0, |e, val_type| {
            e.emit_val_type(val_type)?;
            Ok(())
        })?;
        Ok(())
    }

    // 5.3.6 Function Types
    pub fn emit_func_type(&mut self, func_type: &FuncType) -> io::Result<()> {
        self.write(&[0x60])?;
        self.emit_result_type(&func_type.0)?;
        self.emit_result_type(&func_type.1)?;
        Ok(())
    }

    // 5.3.7 Limits
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

    // 5.3.8 Memory Types
    pub fn emit_mem_type(&mut self, mem_type: &MemType) -> io::Result<()> {
        self.emit_limits(&mem_type.0)?;
        Ok(())
    }

    // 5.3.9 Table Types
    pub fn emit_table_type(&mut self, table_type: &TableType) -> io::Result<()> {
        self.emit_ref_type(&table_type.1)?;
        self.emit_limits(&table_type.0)?;
        Ok(())
    }

    // 5.3.10 Global Types
    pub fn emit_global_type(&mut self, global_type: &GlobalType) -> io::Result<()> {
        self.emit_val_type(&global_type.1)?;
        match global_type.0 {
            Mut::Const => self.write(&[0x00])?,
            Mut::Var => self.write(&[0x01])?,
        }
        Ok(())
    }
}
