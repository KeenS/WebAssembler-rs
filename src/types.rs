use util::*;
use ::Dump;

#[derive(Debug, Clone)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}


#[derive(Debug, Clone)]
pub struct BlockType(pub Option<ValueType>);
#[derive(Debug, Clone)]
pub enum ElemType {
    AnyFunc
}

#[derive(Debug, Clone)]
pub struct FuncType{
    pub params: Vec<ValueType>,
    pub ret: Option<ValueType>,
}

#[derive(Debug, Clone)]
pub struct GlobalType {
    pub content: ValueType,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct TableType {
    pub element: ElemType,
    pub limits: ResizableLimits,
}


#[derive(Debug, Clone)]
pub struct MemoryType {
    pub limits: ResizableLimits,
}

#[derive(Debug, Clone)]
pub enum ExternalKind {
    Function = 0,
    Table = 1,
    Memory = 2,
    Global = 3,
}

#[derive(Debug, Clone)]
pub struct ResizableLimits {
    pub flags: u32,
    pub initial: u32,
    pub maximum: Option<u32>,
}

impl Dump for ValueType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::ValueType::*;
        match self {
            &I32 => {
                write_varint7(buf, -0x01)
            },
            &I64 => {
                write_varint7(buf, -0x02)
            },
            &F32 => {
                write_varint7(buf, -0x03)
            },
            &F64 => {
                write_varint7(buf, -0x04)
            },
        }
   }
}

impl Dump for BlockType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        match &self.0 {
            &Some(ref v) => v.dump(buf),
            &None => write_varint7(buf, -0x40)
        }
    }
}

impl Dump for ElemType {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::ElemType::*;
        match self {
            &AnyFunc => {
                write_varint7(buf, -0x10)
            },
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

impl Dump for ExternalKind {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_uint8(buf, self.clone() as u8)
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


// FIXME: assembly
#[derive(Debug, Clone)]
pub struct InitExpr;

impl Dump for InitExpr {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        // FIXME
        let mut size = 0;
        size
    }
}

pub type TypeIndex = u32;
pub type ImportIndex = u32;
pub type FunctionIndex = u32;
pub type TableIndex = u32;
pub type MemoryIndex = u32;
pub type GlobalIndex = u32;
pub type ExportIndex = u32;
pub type ElementIndex = u32;
pub type CodeIndex = u32;
pub type DataIndex = u32;

