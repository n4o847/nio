#![allow(dead_code)]

use super::super::syntax::*;
use super::*;

// https://webassembly.github.io/spec/core/binary/types.html

// 5.3 Types

impl<W: io::Write> Emitter<W> {
    // 5.3.1 Number Types

    pub fn emit_num_type(&mut self, num_type: &NumType) -> io::Result<()> {
        match num_type {
            NumType::F64 => self.write(&[0x7C])?,
            NumType::F32 => self.write(&[0x7D])?,
            NumType::I64 => self.write(&[0x7E])?,
            NumType::I32 => self.write(&[0x7F])?,
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

    // 5.3.3 Heap Types

    pub fn emit_heap_type(&mut self, heap_type: &HeapType) -> io::Result<()> {
        match heap_type {
            HeapType::AbsHeapType(ht) => match ht {
                AbsHeapType::Exn => self.write(&[0x69])?,
                AbsHeapType::Array => self.write(&[0x6A])?,
                AbsHeapType::Struct => self.write(&[0x6B])?,
                AbsHeapType::I31 => self.write(&[0x6C])?,
                AbsHeapType::Eq => self.write(&[0x6D])?,
                AbsHeapType::Any => self.write(&[0x6E])?,
                AbsHeapType::Extern => self.write(&[0x6F])?,
                AbsHeapType::Func => self.write(&[0x70])?,
                AbsHeapType::None => self.write(&[0x71])?,
                AbsHeapType::NoExtern => self.write(&[0x72])?,
                AbsHeapType::NoFunc => self.write(&[0x73])?,
                AbsHeapType::NoExn => self.write(&[0x74])?,
            },
            HeapType::TypeIdx(x) => {
                self.write_s33(x.0 as i64)?;
            }
        }
        Ok(())
    }

    // 5.3.4 Reference Types

    pub fn emit_ref_type(&mut self, ref_type: &RefType) -> io::Result<()> {
        match ref_type {
            RefType(Some(Null), ht @ HeapType::AbsHeapType(_)) => {
                self.emit_heap_type(ht)?;
            }
            RefType(Some(Null), ht) => {
                self.write(&[0x63])?;
                self.emit_heap_type(ht)?;
            }
            RefType(None, ht) => {
                self.write(&[0x64])?;
                self.emit_heap_type(ht)?;
            }
        }
        Ok(())
    }

    // 5.3.5 Value Types

    pub fn emit_val_type(&mut self, val_type: &ValType) -> io::Result<()> {
        match val_type {
            ValType::NumType(t) => self.emit_num_type(t)?,
            ValType::VecType(t) => self.emit_vec_type(t)?,
            ValType::RefType(t) => self.emit_ref_type(t)?,
        }
        Ok(())
    }

    // 5.3.6 Result Types

    pub fn emit_result_type(&mut self, result_type: &ResultType) -> io::Result<()> {
        self.write_list(&result_type.0, |e, val_type| {
            e.emit_val_type(val_type)?;
            Ok(())
        })?;
        Ok(())
    }

    // 5.3.7 Composite Types

    pub fn emit_comp_type(&mut self, comp_type: &CompType) -> io::Result<()> {
        match comp_type {
            CompType::Array(ft) => {
                self.write(&[0x5E])?;
                self.emit_field_type(ft)?;
            }
            CompType::Struct(ft) => {
                self.write(&[0x5F])?;
                self.write_list(ft, |e, field_type| {
                    e.emit_field_type(field_type)?;
                    Ok(())
                })?;
            }
            CompType::Func(t1, t2) => {
                self.write(&[0x60])?;
                self.emit_result_type(t1)?;
                self.emit_result_type(t2)?;
            }
        }
        Ok(())
    }

    fn emit_field_type(&mut self, field_type: &FieldType) -> io::Result<()> {
        match &field_type.1 {
            StorageType::ValType(t) => self.emit_val_type(t)?,
            StorageType::PackType(pt) => match pt {
                PackType::I16 => self.write(&[0x77])?,
                PackType::I8 => self.write(&[0x78])?,
            },
        }
        match &field_type.0 {
            None => self.write(&[0x00])?,
            Some(Mut) => self.write(&[0x01])?,
        }
        Ok(())
    }

    // 5.3.8 Recursive Types

    pub fn emit_rec_type(&mut self, rec_type: &RecType) -> io::Result<()> {
        match rec_type.0.as_slice() {
            [st] => {
                self.emit_sub_type(st)?;
            }
            st => {
                self.write(&[0x4E])?;
                self.write_list(st, |e, sub_type| {
                    e.emit_sub_type(sub_type)?;
                    Ok(())
                })?;
            }
        }
        Ok(())
    }

    fn emit_sub_type(&mut self, sub_type: &SubType) -> io::Result<()> {
        match sub_type {
            SubType(Some(Final), x, ct) => {
                self.write(&[0x4F])?;
                self.write_list(x, |e, TypeUse(type_idx)| {
                    e.write_u32(type_idx.0)?;
                    Ok(())
                })?;
                self.emit_comp_type(ct)?;
            }
            SubType(None, x, ct) => {
                self.write(&[0x4F])?;
                self.write_list(x, |e, TypeUse(type_idx)| {
                    e.write_u32(type_idx.0)?;
                    Ok(())
                })?;
                self.emit_comp_type(ct)?;
            }
        }
        Ok(())
    }

    // 5.3.9 Limits

    pub fn emit_limits(&mut self, addr_type: &AddrType, limits: &Limits) -> io::Result<()> {
        match (addr_type, limits) {
            (AddrType::I32, &Limits(n, None)) => {
                self.write(&[0x00])?;
                self.write_u64(n)?;
            }
            (AddrType::I32, &Limits(n, Some(m))) => {
                self.write(&[0x01])?;
                self.write_u64(n)?;
                self.write_u64(m)?;
            }
            (AddrType::I64, &Limits(n, None)) => {
                self.write(&[0x04])?;
                self.write_u64(n)?;
            }
            (AddrType::I64, &Limits(n, Some(m))) => {
                self.write(&[0x05])?;
                self.write_u64(n)?;
                self.write_u64(m)?;
            }
        }
        Ok(())
    }

    // 5.3.10 Tag Types

    pub fn emit_tag_type(&mut self, tag_type: &TagType) -> io::Result<()> {
        self.write(&[0x00])?;
        self.write_u32(tag_type.0.0.0)?;
        Ok(())
    }

    // 5.3.11 Global Types

    pub fn emit_global_type(&mut self, global_type: &GlobalType) -> io::Result<()> {
        self.emit_val_type(&global_type.1)?;
        match global_type.0 {
            None => self.write(&[0x00])?,
            Some(Mut) => self.write(&[0x01])?,
        }
        Ok(())
    }

    // 5.3.12 Memory Types

    pub fn emit_mem_type(&mut self, mem_type: &MemType) -> io::Result<()> {
        self.emit_limits(&mem_type.0, &mem_type.1)?;
        Ok(())
    }

    // 5.3.13 Table Types

    pub fn emit_table_type(&mut self, table_type: &TableType) -> io::Result<()> {
        self.emit_ref_type(&table_type.2)?;
        self.emit_limits(&table_type.0, &table_type.1)?;
        Ok(())
    }

    // 5.3.14 External Types

    pub fn emit_extern_type(&mut self, extern_type: &ExternType) -> io::Result<()> {
        match extern_type {
            ExternType::Func(TypeUse(x)) => {
                self.write(&[0x00])?;
                self.write_u32(x.0)?;
            }
            ExternType::Table(tt) => {
                self.write(&[0x01])?;
                self.emit_table_type(tt)?;
            }
            ExternType::Mem(mt) => {
                self.write(&[0x02])?;
                self.emit_mem_type(mt)?;
            }
            ExternType::Global(gt) => {
                self.write(&[0x03])?;
                self.emit_global_type(gt)?;
            }
            ExternType::Tag(jt) => {
                self.write(&[0x04])?;
                self.emit_tag_type(jt)?;
            }
        }
        Ok(())
    }
}
