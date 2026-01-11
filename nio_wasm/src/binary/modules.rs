use super::super::syntax::*;
use super::*;
use crate::binary::values::U32;

// https://webassembly.github.io/spec/core/binary/modules.html

// 5.5 Modules

// 5.5.1 Indices

macro_rules! impl_binary_for_idx {
    ($idx:ty) => {
        impl Binary for $idx {
            fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
                b.emit(U32(self.0))?;
                Ok(())
            }
        }
    };
}

impl_binary_for_idx!(TypeIdx);
impl_binary_for_idx!(FuncIdx);
impl_binary_for_idx!(TableIdx);
impl_binary_for_idx!(MemIdx);
impl_binary_for_idx!(GlobalIdx);
impl_binary_for_idx!(TagIdx);
impl_binary_for_idx!(ElemIdx);
impl_binary_for_idx!(DataIdx);
impl_binary_for_idx!(LocalIdx);
impl_binary_for_idx!(LabelIdx);

impl Binary for ExternIdx {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            ExternIdx::Func(x) => emit!(b, 0x00, x)?,
            ExternIdx::Global(x) => emit!(b, 0x01, x)?,
            ExternIdx::Table(x) => emit!(b, 0x02, x)?,
            ExternIdx::Mem(x) => emit!(b, 0x03, x)?,
            ExternIdx::Tag(x) => emit!(b, 0x04, x)?,
        }
        Ok(())
    }
}

// 5.5.2 Sections

struct Section<'a, const N: u8, T: Binary>(&'a T);

impl<const N: u8, T: Binary> Binary for Section<'_, N, T> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        b.emit(N)?;
        b.write_sized(|b| {
            b.emit(self.0)?;
            Ok(())
        })?;
        Ok(())
    }
}

// 5.5.4 Type Section

struct TypeSec<'a>(Section<'a, 1, Vec<Type>>);

impl Binary for TypeSec<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let TypeSec(ty) = self;
        if ty.0.is_empty() {
            return Ok(());
        }
        emit!(b, ty)?;
        Ok(())
    }
}

impl Binary for Type {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let Type(qt) = self;
        emit!(b, qt)?;
        Ok(())
    }
}

// 5.5.5 Import Section

struct ImportSec<'a>(Section<'a, 2, Vec<Import>>);

impl Binary for ImportSec<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let ImportSec(im) = self;
        if im.0.is_empty() {
            return Ok(());
        }
        emit!(b, im)?;
        Ok(())
    }
}

impl Binary for Import {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let Import(nm1, nm2, xt) = self;
        emit!(b, nm1, nm2, xt)?;
        Ok(())
    }
}

// 5.5.6 Function Section

struct FuncSec<'a>(Section<'a, 3, Vec<TypeIdx>>);

impl Binary for FuncSec<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let FuncSec(x) = self;
        if x.0.is_empty() {
            return Ok(());
        }
        emit!(b, x)?;
        Ok(())
    }
}

// 5.5.7 Table Section

struct TableSec<'a>(Section<'a, 4, Vec<Table>>);

impl Binary for TableSec<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let TableSec(tab) = self;
        if tab.0.is_empty() {
            return Ok(());
        }
        emit!(b, tab)?;
        Ok(())
    }
}

impl Binary for Table {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let Table(tt, e) = self;
        match (tt, e.0.as_slice()) {
            (TableType(_, _, RefType(_, ht1)), [Instr::RefNull(ht2)]) if ht1 == ht2 => {
                emit!(b, tt)?;
            }
            _ => {
                emit!(b, 0x40, 0x00, tt, e)?;
            }
        }
        Ok(())
    }
}

// 5.5.8 Memory Section
// TODO

// 5.5.9 Global Section
// TODO

// 5.5.10 Export Section

struct ExportSec<'a>(Section<'a, 7, Vec<Export>>);

impl Binary for ExportSec<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let ExportSec(ex) = self;
        if ex.0.is_empty() {
            return Ok(());
        }
        emit!(b, ex)?;
        Ok(())
    }
}

impl Binary for Export {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let Export(nm, xx) = self;
        emit!(b, nm, xx)?;
        Ok(())
    }
}

// 5.5.11 Start Section
// TODO

// 5.5.12 Element Section

struct ElemSec<'a>(Section<'a, 9, Vec<Elem>>);

impl Binary for ElemSec<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let ElemSec(elem) = self;
        if elem.0.is_empty() {
            return Ok(());
        }
        emit!(b, elem)?;
        Ok(())
    }
}

struct ElemKind;

impl Binary for ElemKind {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        emit!(b, 0x00)?;
        Ok(())
    }
}

impl Binary for Elem {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        // Note:
        // The initial integer can be interpreted as a bitfield. Bit 0 distinguishes a passive or declarative segment from
        // an active segment, bit 1 indicates the presence of an explicit table index for an active segment and otherwise
        // distinguishes passive from declarative segments, bit 2 indicates the use of element type and element expressions
        // instead of element kind and element indices.

        const REF_NULL_FUNC: RefType =
            RefType(Some(Null), HeapType::AbsHeapType(AbsHeapType::Func));
        const REF_FUNC: RefType = RefType(None, HeapType::AbsHeapType(AbsHeapType::Func));

        let kind = if matches!(self.0, REF_NULL_FUNC) {
            Some(ElemKind)
        } else {
            None
        };

        let indices = 'indices: {
            let mut indices = vec![];
            for expr in self.1.iter() {
                match expr.0.as_slice() {
                    [Instr::RefFunc(y)] => indices.push(y.clone()),
                    _ => break 'indices None,
                }
            }
            Some(indices)
        };

        match (self, &kind, &indices) {
            (Elem(REF_FUNC, _, ElemMode::Active(TableIdx(0), eo)), _, Some(y)) => {
                emit!(b, &U32(0), eo, y)?;
            }
            (Elem(_, _, ElemMode::Passive), Some(rt), Some(y)) => {
                emit!(b, &U32(1), rt, y)?;
            }
            (Elem(_, _, ElemMode::Active(x, e)), Some(rt), Some(y)) => {
                emit!(b, &U32(2), x, e, rt, y)?;
            }
            (Elem(_, _, ElemMode::Declare), Some(rt), Some(y)) => {
                emit!(b, &U32(3), rt, y)?;
            }
            (Elem(REF_NULL_FUNC, e, ElemMode::Active(TableIdx(0), eo)), _, _) => {
                emit!(b, &U32(4), eo, e)?;
            }
            (Elem(rt, e, ElemMode::Passive), _, _) => {
                emit!(b, &U32(5), rt, e)?;
            }
            (Elem(REF_NULL_FUNC, e, ElemMode::Active(x, eo)), _, _) => {
                emit!(b, &U32(6), x, eo, e)?;
            }
            (Elem(rt, e, ElemMode::Declare), _, _) => {
                emit!(b, &U32(7), rt, e)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

// 5.5.13 Code Section

struct CodeSec<'a>(Section<'a, 10, Vec<Code<'a>>>);

impl Binary for CodeSec<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let CodeSec(code) = self;
        if code.0.is_empty() {
            return Ok(());
        }
        emit!(b, code)?;
        Ok(())
    }
}

struct Code<'a>(&'a Vec<Local>, &'a Expr);

impl Binary for Code<'_> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let Code(loc, e) = self;
        b.write_sized(|b| {
            let mut chunks = Vec::new();
            for i in 0..loc.len() {
                if i == 0 || loc[i - 1].0 != loc[i].0 {
                    chunks.push((1, &loc[i].0));
                } else {
                    chunks.last_mut().unwrap().0 += 1;
                }
            }

            b.emit(U32(chunks.len() as u32))?;
            for &(n, t) in chunks.iter() {
                b.emit(U32(n))?;
                b.emit(t)?;
            }

            b.emit(e)?;

            Ok(())
        })?;
        Ok(())
    }
}

// 5.5.14 Data Section
// TODO

// 5.5.15 Data Count Section
// TODO

// 5.5.16 Tag Section
// TODO

// 5.5.17 Modules

impl Binary for Module {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let Module(type_, import, _tag, _global, _mem, table, func, _data, elem, _start, export) =
            self;

        let magic = [0x00, 0x61, 0x73, 0x6D];
        b.write(&magic)?;

        let version = [0x01, 0x00, 0x00, 0x00];
        b.write(&version)?;

        b.emit(TypeSec(Section(type_)))?;

        b.emit(ImportSec(Section(import)))?;

        let typeidx = func
            .iter()
            .map(|Func(typeidx, _, _)| typeidx.clone())
            .collect();
        b.emit(FuncSec(Section(&typeidx)))?;

        b.emit(TableSec(Section(table)))?;

        // mem

        // tag

        // global

        b.emit(ExportSec(Section(export)))?;

        // start

        b.emit(ElemSec(Section(elem)))?;

        // data count

        let code = func
            .iter()
            .map(|Func(_, local, expr)| Code(local, expr))
            .collect();
        b.emit(CodeSec(Section(&code)))?;

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
        let mut buf = Vec::new();
        let mut emitter = Emitter::new(&mut buf);
        let result = emitter.emit(&module);
        assert!(result.is_ok());
        assert_eq!(buf, &[0, 97, 115, 109, 1, 0, 0, 0]);
    }
}
