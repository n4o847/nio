#![allow(dead_code)]

use super::super::syntax::*;
use super::*;

// https://webassembly.github.io/spec/core/binary/modules.html

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

    // Sections
    fn emit_section<F>(&mut self, id: u8, f: F) -> io::Result<()>
    where
        F: FnOnce(&mut Emitter<&mut Vec<u8>>) -> io::Result<()>,
    {
        self.write(&[id])?;
        self.write_sized(f)?;
        Ok(())
    }

    // Type Section
    fn emit_type_sec(&mut self, types: &Vec<FuncType>) -> io::Result<()> {
        if types.is_empty() {
            return Ok(());
        }
        self.emit_section(1, |e| {
            e.write_u32(types.len() as u32)?;
            for func_type in types.iter() {
                e.emit_func_type(func_type)?;
            }
            Ok(())
        })
    }

    // Import Section
    fn emit_import_sec(&mut self, imports: &Vec<Import>) -> io::Result<()> {
        if imports.is_empty() {
            return Ok(());
        }
        self.emit_section(2, |e| {
            e.write_u32(imports.len() as u32)?;
            for import in imports.iter() {
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
            }
            Ok(())
        })
    }

    // Function Section
    fn emit_func_sec(&mut self, funcs: &Vec<Func>) -> io::Result<()> {
        if funcs.is_empty() {
            return Ok(());
        }
        self.emit_section(3, |e| {
            e.write_u32(funcs.len() as u32)?;
            for func in funcs.iter() {
                e.write_u32(func.r#type.0)?;
            }
            Ok(())
        })
    }

    // Table Section
    fn emit_table_sec(&mut self, tables: &Vec<Table>) -> io::Result<()> {
        if tables.is_empty() {
            return Ok(());
        }
        self.emit_section(4, |e| {
            e.write_u32(tables.len() as u32)?;
            for table in tables.iter() {
                e.emit_table_type(&table.r#type)?;
            }
            Ok(())
        })
    }

    // Memory Section
    fn emit_mem_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // Global Section
    fn emit_global_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // Export Section
    fn emit_export_sec(&mut self, exports: &Vec<Export>) -> io::Result<()> {
        if exports.is_empty() {
            return Ok(());
        }
        self.emit_section(7, |e| {
            e.write_u32(exports.len() as u32)?;
            for export in exports.iter() {
                e.write_name(&export.name)?;
                use ExportDesc::*;
                match &export.desc {
                    Func(x) => {
                        e.write_u32(0x00)?;
                        e.write_u32(x.0)?;
                    }
                    Table(x) => {
                        e.write_u32(0x01)?;
                        e.write_u32(x.0)?;
                    }
                    Mem(x) => {
                        e.write_u32(0x02)?;
                        e.write_u32(x.0)?;
                    }
                    Global(x) => {
                        e.write_u32(0x03)?;
                        e.write_u32(x.0)?;
                    }
                }
            }
            Ok(())
        })
    }

    // Start Section
    fn emit_start_sec(&mut self) -> io::Result<()> {
        todo!()
    }

    // Element Section
    fn emit_elem_sec(&mut self, elems: &Vec<Elem>) -> io::Result<()> {
        if elems.is_empty() {
            return Ok(());
        }
        self.emit_section(9, |e| {
            e.write_u32(elems.len() as u32)?;
            for elem in elems.iter() {
                e.write_u32(elem.table.0)?;
                e.emit_expr(&elem.offset)?;
                e.write_u32(elem.init.len() as u32)?;
                for func_idx in elem.init.iter() {
                    e.write_u32(func_idx.0)?;
                }
            }
            Ok(())
        })
    }

    // Code Section
    fn emit_code_sec(&mut self, funcs: &Vec<Func>) -> io::Result<()> {
        if funcs.is_empty() {
            return Ok(());
        }
        self.emit_section(10, |e| {
            e.write_u32(funcs.len() as u32)?;
            for func in funcs.iter() {
                e.write_sized(|e| {
                    let mut chunks = Vec::new();
                    for i in 0..func.locals.len() {
                        if i == 0 || func.locals[i - 1] != func.locals[i] {
                            chunks.push((1, i));
                        } else {
                            chunks.last_mut().unwrap().0 += 1;
                        }
                    }

                    e.write_u32(chunks.len() as u32)?;
                    for chunk in chunks.iter() {
                        e.write_u32(chunk.0)?;
                        e.emit_val_type(&func.locals[chunk.1])?;
                    }

                    e.emit_expr(&func.body)?;

                    Ok(())
                })?;
            }
            Ok(())
        })
    }

    // Data Section
    fn emit_data_sec(&mut self) -> io::Result<()> {
        todo!()
    }

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

        self.emit_elem_sec(&module.elem)?;

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
