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
    fn emit_type_sec(&mut self, types: &Vec<FuncType>) -> io::Result<()> {
        if types.is_empty() {
            return Ok(());
        }
        self.emit_section(1, |e| {
            e.write_vec(types, |e, func_type| {
                e.emit_func_type(func_type)?;
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
            e.write_vec(imports, |e, import| {
                e.write_name(&import.module)?;
                e.write_name(&import.name)?;
                match &import.desc {
                    ImportDesc::Func(x) => {
                        e.write(&[0x00])?;
                        e.write_u32(x.0)?;
                    }
                    ImportDesc::Table(_tt) => todo!(),
                    ImportDesc::Mem(_mt) => todo!(),
                    ImportDesc::Global(_gt) => todo!(),
                }
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
            e.write_vec(funcs, |e, func| {
                e.write_u32(func.type_.0)?;
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
            e.write_vec(tables, |e, table| {
                e.emit_table_type(&table.type_)?;
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
            e.write_vec(exports, |e, export| {
                e.write_name(&export.name)?;
                match &export.desc {
                    ExportDesc::Func(x) => {
                        e.write_u32(0x00)?;
                        e.write_u32(x.0)?;
                    }
                    ExportDesc::Table(x) => {
                        e.write_u32(0x01)?;
                        e.write_u32(x.0)?;
                    }
                    ExportDesc::Mem(x) => {
                        e.write_u32(0x02)?;
                        e.write_u32(x.0)?;
                    }
                    ExportDesc::Global(x) => {
                        e.write_u32(0x03)?;
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
            e.write_vec(elems, |e, elem| {
                // Note:
                // The initial integer can be interpreted as a bitfield. Bit 0 distinguishes a passive or declarative segment from
                // an active segment, bit 1 indicates the presence of an explicit table index for an active segment and otherwise
                // distinguishes passive from declarative segments, bit 2 indicates the use of element type and element expressions
                // instead of element kind and element indices.

                let mut bits = 0;

                let can_use_elem_kind = matches!(elem.type_, RefType::FuncRef);

                let can_use_elem_indices = 'can_use_elem_indices: {
                    let mut indices = vec![];
                    for expr in elem.init.iter() {
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

                match (&elem.type_, &elem.mode) {
                    (
                        RefType::FuncRef,
                        ElemMode::Active {
                            table: TableIdx(0),
                            offset,
                        },
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
                    (et, ElemMode::Active { table, offset }) => {
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
                    e.write_vec(indices, |e, func_idx| {
                        e.write_u32(func_idx.0)?;
                        Ok(())
                    })?;
                } else {
                    e.write_vec(&elem.init, |e, expr| {
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
            e.write_vec(funcs, |e, func| {
                e.write_sized(|e| {
                    let mut chunks = Vec::new();
                    for i in 0..func.locals.len() {
                        if i == 0 || func.locals[i - 1] != func.locals[i] {
                            chunks.push((1, i));
                        } else {
                            chunks.last_mut().unwrap().0 += 1;
                        }
                    }

                    e.write_vec(&chunks, |e, chunk| {
                        e.write_u32(chunk.0)?;
                        e.emit_val_type(&func.locals[chunk.1])?;
                        Ok(())
                    })?;

                    e.emit_expr(&func.body)?;

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

    // Modules
    pub fn emit_module(&mut self, module: &Module) -> io::Result<()> {
        let magic = [0x00, 0x61, 0x73, 0x6d];
        self.write(&magic)?;

        let version = [0x01, 0x00, 0x00, 0x00];
        self.write(&version)?;

        self.emit_type_sec(&module.types)?;

        self.emit_import_sec(&module.imports)?;

        self.emit_func_sec(&module.funcs)?;

        self.emit_table_sec(&module.tables)?;

        // mem

        // global

        self.emit_export_sec(&module.exports)?;

        // start

        self.emit_elem_sec(&module.elems)?;

        // data count

        self.emit_code_sec(&module.funcs)?;

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
