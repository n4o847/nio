#![allow(dead_code)]

use super::super::syntax::*;
use super::*;

// https://webassembly.github.io/spec/core/binary/modules.html

// 5.5 Modules

impl<W: io::Write> Emitter<W> {
    fn write_sized<F>(&mut self, f: F) -> io::Result<()>
    where
        F: FnOnce(&mut Emitter<&mut Vec<u8>>) -> io::Result<()>,
    {
        let mut buffer = Vec::new();
        let mut emitter = Emitter::new(&mut buffer);
        f(&mut emitter)?;
        self.write_u32(buffer.len() as u32)?;
        self.write(&buffer)?;
        Ok(())
    }

    // 5.5.2 Sections

    fn emit_section<F>(&mut self, id: u8, f: F) -> io::Result<()>
    where
        F: FnOnce(&mut Emitter<&mut Vec<u8>>) -> io::Result<()>,
    {
        self.write(&[id])?;
        self.write_sized(f)?;
        Ok(())
    }

    // 5.5.4 Type Section

    fn emit_type_sec(&mut self, types: &Vec<Type>) -> io::Result<()> {
        if types.is_empty() {
            return Ok(());
        }
        self.emit_section(1, |e| {
            e.write_list(types, |e, Type(rec_type)| {
                e.emit_rec_type(rec_type)?;
                Ok(())
            })?;
            Ok(())
        })
    }

    // 5.5.5 Import Section

    fn emit_import_sec(&mut self, imports: &Vec<Import>) -> io::Result<()> {
        if imports.is_empty() {
            return Ok(());
        }
        self.emit_section(2, |e| {
            e.write_list(imports, |e, Import(nm1, nm2, xt)| {
                e.write_name(&nm1)?;
                e.write_name(&nm2)?;
                e.emit_extern_type(xt)?;
                Ok(())
            })?;
            Ok(())
        })
    }

    // 5.5.6 Function Section

    fn emit_func_sec(&mut self, funcs: &Vec<Func>) -> io::Result<()> {
        if funcs.is_empty() {
            return Ok(());
        }
        self.emit_section(3, |e| {
            e.write_list(funcs, |e, Func(x, _, _)| {
                e.write_u32(x.0)?;
                Ok(())
            })?;
            Ok(())
        })
    }

    // 5.5.7 Table Section

    fn emit_table_sec(&mut self, tables: &Vec<Table>) -> io::Result<()> {
        if tables.is_empty() {
            return Ok(());
        }
        self.emit_section(4, |e| {
            e.write_list(tables, |e, Table(tt, e_)| {
                match (tt, e_.0.as_slice()) {
                    (TableType(_, _, RefType(_, ht1)), [Instr::RefNull(ht2)]) if ht1 == ht2 => {
                        e.emit_table_type(tt)?;
                    }
                    _ => {
                        e.write(&[0x40, 0x00])?;
                        e.emit_table_type(tt)?;
                        e.emit_expr(e_)?;
                    }
                }
                Ok(())
            })?;
            Ok(())
        })
    }

    // 5.5.8 Memory Section

    fn emit_mem_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // 5.5.9 Global Section

    fn emit_global_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // 5.5.10 Export Section

    fn emit_export_sec(&mut self, exports: &Vec<Export>) -> io::Result<()> {
        if exports.is_empty() {
            return Ok(());
        }
        self.emit_section(7, |e| {
            e.write_list(exports, |e, Export(nm, xx)| {
                e.write_name(nm)?;
                match xx {
                    ExternIdx::Func(x) => {
                        e.write_u32(0x00)?;
                        e.write_u32(x.0)?;
                    }
                    ExternIdx::Table(x) => {
                        e.write_u32(0x01)?;
                        e.write_u32(x.0)?;
                    }
                    ExternIdx::Mem(x) => {
                        e.write_u32(0x02)?;
                        e.write_u32(x.0)?;
                    }
                    ExternIdx::Global(x) => {
                        e.write_u32(0x03)?;
                        e.write_u32(x.0)?;
                    }
                    ExternIdx::Tag(x) => {
                        e.write_u32(0x04)?;
                        e.write_u32(x.0)?;
                    }
                }
                Ok(())
            })?;
            Ok(())
        })
    }

    // 5.5.11 Start Section

    fn emit_start_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // 5.5.12 Element Section

    fn emit_elem_sec(&mut self, elems: &Vec<Elem>) -> io::Result<()> {
        if elems.is_empty() {
            return Ok(());
        }
        self.emit_section(9, |e| {
            e.write_list(elems, |e, elem| {
                // Note:
                // The initial integer can be interpreted as a bitfield. Bit 0 distinguishes a passive or declarative segment from
                // an active segment, bit 1 indicates the presence of an explicit table index for an active segment and otherwise
                // distinguishes passive from declarative segments, bit 2 indicates the use of element type and element expressions
                // instead of element kind and element indices.

                let mut bits = 0;

                let can_use_elem_kind = matches!(
                    elem.0,
                    RefType(None, HeapType::AbsHeapType(AbsHeapType::Func))
                );

                let can_use_elem_indices = 'can_use_elem_indices: {
                    let mut indices = vec![];
                    for expr in elem.1.iter() {
                        match expr.0.as_slice() {
                            [Instr::RefFunc(y)] => indices.push(y),
                            _ => break 'can_use_elem_indices None,
                        }
                    }
                    Some(indices)
                };

                let use_elem_kind_and_indices =
                    if can_use_elem_kind && let Some(indices) = can_use_elem_indices {
                        Some(indices)
                    } else {
                        bits |= 1 << 2;
                        None
                    };

                match (&elem.0, &elem.2) {
                    (
                        RefType(None, HeapType::AbsHeapType(AbsHeapType::Func)),
                        ElemMode::Active(TableIdx(0), offset),
                    ) => {
                        e.write_u32(bits)?;
                        e.emit_expr(offset)?;
                    }
                    (et, ElemMode::Passive) => {
                        bits |= 1 << 0;
                        e.write_u32(bits)?;
                        if use_elem_kind_and_indices.is_some() {
                            e.write(&[0x00])?;
                        } else {
                            e.emit_ref_type(et)?;
                        }
                    }
                    (et, ElemMode::Active(table, offset)) => {
                        bits |= 1 << 1;
                        e.write_u32(bits)?;
                        e.write_u32(table.0)?;
                        e.emit_expr(offset)?;
                        if use_elem_kind_and_indices.is_some() {
                            e.write(&[0x00])?;
                        } else {
                            e.emit_ref_type(et)?;
                        }
                    }
                    (et, ElemMode::Declarative) => {
                        bits |= 1 << 0;
                        bits |= 1 << 1;
                        e.write_u32(bits)?;
                        if use_elem_kind_and_indices.is_some() {
                            e.write(&[0x00])?;
                        } else {
                            e.emit_ref_type(et)?;
                        }
                    }
                };

                if let Some(indices) = &use_elem_kind_and_indices {
                    e.write_list(indices, |e, func_idx| {
                        e.write_u32(func_idx.0)?;
                        Ok(())
                    })?;
                } else {
                    e.write_list(&elem.1, |e, expr| {
                        e.emit_expr(expr)?;
                        Ok(())
                    })?;
                }

                Ok(())
            })?;
            Ok(())
        })
    }

    // 5.5.13 Code Section

    fn emit_code_sec(&mut self, funcs: &Vec<Func>) -> io::Result<()> {
        if funcs.is_empty() {
            return Ok(());
        }
        self.emit_section(10, |e| {
            e.write_list(funcs, |e, Func(_, loc, e_)| {
                e.write_sized(|e| {
                    let mut chunks = Vec::new();
                    for i in 0..loc.len() {
                        if i == 0 || loc[i - 1].0 != loc[i].0 {
                            chunks.push((1, i));
                        } else {
                            chunks.last_mut().unwrap().0 += 1;
                        }
                    }

                    e.write_list(&chunks, |e, chunk| {
                        e.write_u32(chunk.0)?;
                        e.emit_val_type(&loc[chunk.1].0)?;
                        Ok(())
                    })?;

                    e.emit_expr(e_)?;

                    Ok(())
                })?;
                Ok(())
            })?;
            Ok(())
        })
    }

    // 5.5.14 Data Section

    fn emit_data_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // 5.5.15 Data Count Section
    // TODO

    // 5.5.16 Tag Section

    fn emit_tag_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // Modules
    pub fn emit_module(&mut self, module: &Module) -> io::Result<()> {
        let Module(type_, import, _tag, _global, _mem, table, func, _data, elem, _start, export) =
            module;

        let magic = [0x00, 0x61, 0x73, 0x6d];
        self.write(&magic)?;

        let version = [0x01, 0x00, 0x00, 0x00];
        self.write(&version)?;

        self.emit_type_sec(type_)?;

        self.emit_import_sec(import)?;

        self.emit_func_sec(func)?;

        self.emit_table_sec(table)?;

        // mem

        // tag

        // global

        self.emit_export_sec(export)?;

        // start

        self.emit_elem_sec(elem)?;

        // data count

        self.emit_code_sec(func)?;

        // data

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_module() {
        let module = Module::new();
        let mut buffer = Vec::new();
        let mut emitter = Emitter::new(&mut buffer);
        let result = emitter.emit_module(&module);
        assert!(result.is_ok());
        assert_eq!(buffer, &[0, 97, 115, 109, 1, 0, 0, 0]);
    }
}
