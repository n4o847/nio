use super::super::syntax::*;
use super::*;
use crate::binary::values::{S33, U64};

// https://webassembly.github.io/spec/core/binary/types.html

// 5.3 Types

// 5.3.1 Number Types

impl Binary for NumType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            NumType::F64 => b.emit(0x7C)?,
            NumType::F32 => b.emit(0x7D)?,
            NumType::I64 => b.emit(0x7E)?,
            NumType::I32 => b.emit(0x7F)?,
        }
        Ok(())
    }
}

// 5.3.2 Vector Types

impl Binary for VecType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            VecType::V128 => b.emit(0x7B)?,
        }
        Ok(())
    }
}

// 5.3.3 Heap Types

impl Binary for AbsHeapType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            AbsHeapType::Exn => b.emit(0x69)?,
            AbsHeapType::Array => b.emit(0x6A)?,
            AbsHeapType::Struct => b.emit(0x6B)?,
            AbsHeapType::I31 => b.emit(0x6C)?,
            AbsHeapType::Eq => b.emit(0x6D)?,
            AbsHeapType::Any => b.emit(0x6E)?,
            AbsHeapType::Extern => b.emit(0x6F)?,
            AbsHeapType::Func => b.emit(0x70)?,
            AbsHeapType::None => b.emit(0x71)?,
            AbsHeapType::NoExtern => b.emit(0x72)?,
            AbsHeapType::NoFunc => b.emit(0x73)?,
            AbsHeapType::NoExn => b.emit(0x74)?,
        }
        Ok(())
    }
}

impl Binary for HeapType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            HeapType::AbsHeapType(ht) => b.emit(ht)?,
            HeapType::TypeIdx(x) => b.emit(S33(x.0 as i64))?,
        }
        Ok(())
    }
}

// 5.3.4 Reference Types

impl Binary for RefType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            RefType(Some(Null), HeapType::AbsHeapType(ht)) => {
                b.emit(ht)?;
            }
            RefType(Some(Null), ht) => {
                b.emit(0x63)?;
                b.emit(ht)?;
            }
            RefType(None, ht) => {
                b.emit(0x64)?;
                b.emit(ht)?;
            }
        }
        Ok(())
    }
}

// 5.3.5 Value Types

impl Binary for ValType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            ValType::NumType(nt) => b.emit(nt)?,
            ValType::VecType(vt) => b.emit(vt)?,
            ValType::RefType(rt) => b.emit(rt)?,
        }
        Ok(())
    }
}

// 5.3.6 Result Types

impl Binary for ResultType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            ResultType(t) => b.emit(t)?,
        }
        Ok(())
    }
}

// 5.3.7 Composite Types

impl Binary for Option<Mut> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            None => b.emit(0x00)?,
            Some(Mut) => b.emit(0x01)?,
        }
        Ok(())
    }
}

impl Binary for CompType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            CompType::Array(ft) => {
                b.emit(0x5E)?;
                b.emit(ft)?;
            }
            CompType::Struct(ft) => {
                b.emit(0x5F)?;
                b.emit(ft)?;
            }
            CompType::Func(t1, t2) => {
                b.emit(0x60)?;
                b.emit(t1)?;
                b.emit(t2)?;
            }
        }
        Ok(())
    }
}

impl Binary for FieldType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            FieldType(mut_, zt) => {
                b.emit(zt)?;
                b.emit(mut_)?;
            }
        }
        Ok(())
    }
}

impl Binary for StorageType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            StorageType::ValType(t) => b.emit(t)?,
            StorageType::PackType(pt) => b.emit(pt)?,
        }
        Ok(())
    }
}

impl Binary for PackType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            PackType::I16 => b.emit(0x77)?,
            PackType::I8 => b.emit(0x78)?,
        }
        Ok(())
    }
}

// 5.3.8 Recursive Types

impl Binary for RecType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let RecType(st) = self;
        match st.as_slice() {
            [st] => {
                b.emit(st)?;
            }
            _ => {
                b.emit(0x4E)?;
                b.emit(st)?;
            }
        }
        Ok(())
    }
}

impl Binary for SubType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            SubType(Some(Final), x, ct) if x.is_empty() => {
                b.emit(ct)?;
            }
            SubType(Some(Final), x, ct) => {
                b.emit(0x4F)?;
                b.emit(x.iter().map(|TypeUse(x)| x.clone()).collect::<Vec<_>>())?;
                b.emit(ct)?;
            }
            SubType(None, x, ct) => {
                b.emit(0x4F)?;
                b.emit(x.iter().map(|TypeUse(x)| x.clone()).collect::<Vec<_>>())?;
                b.emit(ct)?;
            }
        }
        Ok(())
    }
}

// 5.3.9 Limits

impl Binary for (&AddrType, &Limits) {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match *self {
            (AddrType::I32, &Limits(n, None)) => {
                b.emit(0x00)?;
                b.emit(U64(n))?;
            }
            (AddrType::I32, &Limits(n, Some(m))) => {
                b.emit(0x01)?;
                b.emit(U64(n))?;
                b.emit(U64(m))?;
            }
            (AddrType::I64, &Limits(n, None)) => {
                b.emit(0x04)?;
                b.emit(U64(n))?;
            }
            (AddrType::I64, &Limits(n, Some(m))) => {
                b.emit(0x05)?;
                b.emit(U64(n))?;
                b.emit(U64(m))?;
            }
        }
        Ok(())
    }
}

// 5.3.10 Tag Types

impl Binary for TagType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let TagType(TypeUse(x)) = self;
        b.emit(0x00)?;
        b.emit(x)?;
        Ok(())
    }
}

// 5.3.11 Global Types

impl Binary for GlobalType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let GlobalType(mut_, t) = self;
        b.emit(t)?;
        b.emit(mut_)?;
        Ok(())
    }
}

// 5.3.12 Memory Types

impl Binary for MemType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let MemType(at, lim) = self;
        b.emit((at, lim))?;
        Ok(())
    }
}

// 5.3.13 Table Types

impl Binary for TableType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let TableType(at, lim, rt) = self;
        b.emit(rt)?;
        b.emit((at, lim))?;
        Ok(())
    }
}

// 5.3.14 External Types

impl Binary for ExternType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            ExternType::Func(TypeUse(x)) => {
                b.emit(0x00)?;
                b.emit(x)?;
            }
            ExternType::Table(tt) => {
                b.emit(0x01)?;
                b.emit(tt)?;
            }
            ExternType::Mem(mt) => {
                b.emit(0x02)?;
                b.emit(mt)?;
            }
            ExternType::Global(gt) => {
                b.emit(0x03)?;
                b.emit(gt)?;
            }
            ExternType::Tag(jt) => {
                b.emit(0x04)?;
                b.emit(jt)?;
            }
        }
        Ok(())
    }
}
