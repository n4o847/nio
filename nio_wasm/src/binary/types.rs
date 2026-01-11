use super::super::syntax::*;
use super::*;
use crate::binary::values::{S33, U64};

// https://webassembly.github.io/spec/core/binary/types.html

// 5.3 Types

// 5.3.1 Number Types

impl Binary for NumType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            NumType::F64 => emit!(b, 0x7C),
            NumType::F32 => emit!(b, 0x7D),
            NumType::I64 => emit!(b, 0x7E),
            NumType::I32 => emit!(b, 0x7F),
        }
    }
}

// 5.3.2 Vector Types

impl Binary for VecType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            VecType::V128 => emit!(b, 0x7B),
        }
    }
}

// 5.3.3 Heap Types

impl Binary for AbsHeapType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            AbsHeapType::Exn => emit!(b, 0x69),
            AbsHeapType::Array => emit!(b, 0x6A),
            AbsHeapType::Struct => emit!(b, 0x6B),
            AbsHeapType::I31 => emit!(b, 0x6C),
            AbsHeapType::Eq => emit!(b, 0x6D),
            AbsHeapType::Any => emit!(b, 0x6E),
            AbsHeapType::Extern => emit!(b, 0x6F),
            AbsHeapType::Func => emit!(b, 0x70),
            AbsHeapType::None => emit!(b, 0x71),
            AbsHeapType::NoExtern => emit!(b, 0x72),
            AbsHeapType::NoFunc => emit!(b, 0x73),
            AbsHeapType::NoExn => emit!(b, 0x74),
        }
    }
}

impl Binary for HeapType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            HeapType::AbsHeapType(ht) => {
                emit!(b, ht)
            }
            HeapType::TypeIdx(x) => {
                emit!(b, S33(x.0 as i64))
            }
        }
    }
}

// 5.3.4 Reference Types

impl Binary for RefType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            RefType(Some(Null), HeapType::AbsHeapType(ht)) => {
                emit!(b, ht)
            }
            RefType(Some(Null), ht) => {
                emit!(b, 0x63, ht)
            }
            RefType(None, ht) => {
                emit!(b, 0x64, ht)
            }
        }
    }
}

// 5.3.5 Value Types

impl Binary for ValType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            ValType::NumType(nt) => emit!(b, nt),
            ValType::VecType(vt) => emit!(b, vt),
            ValType::RefType(rt) => emit!(b, rt),
        }
    }
}

// 5.3.6 Result Types

impl Binary for ResultType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            ResultType(t) => emit!(b, t),
        }
    }
}

// 5.3.7 Composite Types

impl Binary for Option<Mut> {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            None => emit!(b, 0x00),
            Some(Mut) => emit!(b, 0x01),
        }
    }
}

impl Binary for CompType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            CompType::Array(ft) => {
                emit!(b, 0x5E, ft)
            }
            CompType::Struct(ft) => {
                emit!(b, 0x5F, ft)
            }
            CompType::Func(t1, t2) => {
                emit!(b, 0x60, t1, t2)
            }
        }
    }
}

impl Binary for FieldType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            FieldType(mut_, zt) => {
                emit!(b, zt, mut_)
            }
        }
    }
}

impl Binary for StorageType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            StorageType::ValType(t) => emit!(b, t),
            StorageType::PackType(pt) => emit!(b, pt),
        }
    }
}

impl Binary for PackType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            PackType::I16 => emit!(b, 0x77),
            PackType::I8 => emit!(b, 0x78),
        }
    }
}

// 5.3.8 Recursive Types

impl Binary for RecType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let RecType(st) = self;
        match st.as_slice() {
            [st] => {
                emit!(b, st)
            }
            _ => {
                emit!(b, 0x4E, st)
            }
        }
    }
}

impl Binary for SubType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            SubType(Some(Final), x, ct) if x.is_empty() => {
                emit!(b, ct)
            }
            SubType(Some(Final), x, ct) => {
                let x = x.iter().map(|TypeUse(x)| x.clone()).collect::<Vec<_>>();
                emit!(b, 0x4F, x, ct)
            }
            SubType(None, x, ct) => {
                let x = x.iter().map(|TypeUse(x)| x.clone()).collect::<Vec<_>>();
                emit!(b, 0x4F, x, ct)
            }
        }
    }
}

// 5.3.9 Limits

impl Binary for (&AddrType, &Limits) {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match *self {
            (AddrType::I32, &Limits(n, None)) => {
                emit!(b, 0x00, U64(n))
            }
            (AddrType::I32, &Limits(n, Some(m))) => {
                emit!(b, 0x01, U64(n), U64(m))
            }
            (AddrType::I64, &Limits(n, None)) => {
                emit!(b, 0x04, U64(n))
            }
            (AddrType::I64, &Limits(n, Some(m))) => {
                emit!(b, 0x05, U64(n), U64(m))
            }
        }
    }
}

// 5.3.10 Tag Types

impl Binary for TagType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let TagType(TypeUse(x)) = self;
        emit!(b, 0x00, x)
    }
}

// 5.3.11 Global Types

impl Binary for GlobalType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let GlobalType(mut_, t) = self;
        emit!(b, t, mut_)
    }
}

// 5.3.12 Memory Types

impl Binary for MemType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let MemType(at, lim) = self;
        emit!(b, (at, lim))
    }
}

// 5.3.13 Table Types

impl Binary for TableType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        let TableType(at, lim, rt) = self;
        emit!(b, rt, (at, lim))
    }
}

// 5.3.14 External Types

impl Binary for ExternType {
    fn binary<W: io::Write>(&self, b: &mut Emitter<W>) -> io::Result<()> {
        match self {
            ExternType::Func(TypeUse(x)) => {
                emit!(b, 0x00, x)
            }
            ExternType::Table(tt) => {
                emit!(b, 0x01, tt)
            }
            ExternType::Mem(mt) => {
                emit!(b, 0x02, mt)
            }
            ExternType::Global(gt) => {
                emit!(b, 0x03, gt)
            }
            ExternType::Tag(jt) => {
                emit!(b, 0x04, jt)
            }
        }
    }
}
