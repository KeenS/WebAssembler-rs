use std::ops::Deref;
use util::*;
use ops::Op;
use Dump;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockType(pub Option<ValueType>);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElemType {
    AnyFunc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncType {
    pub params: Vec<ValueType>,
    pub ret: Option<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalType {
    pub content: ValueType,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableType {
    pub element: ElemType,
    pub limits: ResizableLimits,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemoryType {
    pub limits: ResizableLimits,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResizableLimits {
    pub flags: u32,
    pub initial: u32,
    pub maximum: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Code(pub Vec<Op>);

#[derive(Debug, Clone)]
pub struct InitExpr(pub Code);


impl Dump for ValueType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::ValueType::*;
        match self {
            &I32 => write_varint7(buf, -0x01),
            &I64 => write_varint7(buf, -0x02),
            &F32 => write_varint7(buf, -0x03),
            &F64 => write_varint7(buf, -0x04),
        }
    }
}

impl Dump for BlockType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        match &self.0 {
            &Some(ref v) => v.dump(buf),
            &None => write_varint7(buf, -0x40),
        }
    }
}

impl Dump for ElemType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::ElemType::*;
        match self {
            &AnyFunc => write_varint7(buf, -0x10),
        }
    }
}
impl Dump for FuncType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let params = &self.params;
        let ret = &self.ret;

        let mut size = 0;
        size += write_varint7(buf, -0x20);

        size += write_varuint32(buf, params.len() as u32);
        for param in params {
            size += param.dump(buf);
        }

        size += write_varuint1(buf, ret.is_some() as u8);
        for ret in ret {
            size += ret.dump(buf);
        }
        size
    }
}

impl Dump for GlobalType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        size += self.content.dump(buf);
        size += write_varuint1(buf, self.mutable as u8);
        size
    }
}

impl Dump for TableType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        size += self.element.dump(buf);
        size += self.limits.dump(buf);
        size
    }
}

impl Dump for MemoryType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        self.limits.dump(buf)
    }
}


impl ResizableLimits {
    pub fn new(limit: u32) -> Self {
        ResizableLimits {
            flags: 0,
            initial: limit,
            maximum: None,
        }
    }

    pub fn max(mut self, maximum: u32) -> Self {
        self.maximum = Some(maximum);
        self
    }

    /// unstable
    pub fn flags(mut self, flags: u32) -> Self {
        self.flags = flags;
        self
    }
}

impl Dump for ResizableLimits {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        let flags = self.flags | (self.maximum.is_some() as u32);
        size += write_varuint32(buf, flags);
        size += write_varuint32(buf, self.initial);
        if let Some(m) = self.maximum {
            size += write_varuint32(buf, m);
        }
        size
    }
}

impl Dump for Code {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        for op in self.0.iter() {
            size += op.dump(buf);
        }
        size
    }
}


impl Dump for InitExpr {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        self.0.dump(buf)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeIndex(u32);
impl Deref for TypeIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImportIndex(u32);
impl Deref for ImportIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// restrict to this crate
pub struct FunctionIndex(pub u32);
impl Deref for FunctionIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImportedFunctionIndex(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InnerFunctionSpaceIndex {
    Import(ImportedFunctionIndex),
    Function(FunctionIndex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionSpaceIndex(pub InnerFunctionSpaceIndex);

impl Deref for FunctionSpaceIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        use self::InnerFunctionSpaceIndex::*;
        match self.0 {
            Import(ref i) => &i.0,
            Function(ref f) => &**f,
        }
    }
}

impl Into<FunctionSpaceIndex> for FunctionIndex {
    fn into(self) -> FunctionSpaceIndex {
        FunctionSpaceIndex(InnerFunctionSpaceIndex::Function(self))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableIndex(u32);
impl Deref for TableIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryIndex(u32);
impl Deref for MemoryIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalIndex(u32);
impl Deref for GlobalIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExportIndex(u32);
impl Deref for ExportIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementIndex(u32);
impl Deref for ElementIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeIndex(u32);
impl Deref for CodeIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataIndex(u32);
impl Deref for DataIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalIndex(u32);
impl Deref for LocalIndex {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}


pub mod internal {
    use types::*;
    macro_rules! impl_new {
        ($name: tt) => {
            impl $name {
                pub fn new(u: u32) -> Self {
                    $name(u)
                }
            }
        }
    }

    impl_new!(TypeIndex);
    impl_new!(ImportIndex);
    impl_new!(FunctionIndex);
    impl_new!(TableIndex);
    impl_new!(MemoryIndex);
    impl_new!(GlobalIndex);
    impl_new!(ExportIndex);
    impl_new!(ElementIndex);
    impl_new!(CodeIndex);
    impl_new!(DataIndex);

    impl_new!(LocalIndex);
}
