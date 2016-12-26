use util::*;
use Dump;

#[derive(Debug, Clone)]
pub enum Control {
    Unreachable,
    Nop,
    Block,
    Loop,
    If,
    Else,
    End,
    Br{ depth: u32 },
    BrIf{ depth: u32},
    BrTable(BrTarget),
    Return,
}

impl Dump for Control {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::Control::*;
        match self {
            &Unreachable => write_uint8(buf, 0x00),
            &Nop => write_uint8(buf, 0x01),
            &Block => write_uint8(buf, 0x02),
            &Loop => write_uint8(buf, 0x03),
            &If => write_uint8(buf, 0x04),
            &Else => write_uint8(buf, 0x05),
            &End => write_uint8(buf, 0x06),
            &Br{ref depth} => {
                let mut size = 0;
                size += write_uint8(buf, 0x0c);
                size += write_varuint32(buf, *depth);
                size
                },
            &BrIf{ref depth} => {
                let mut size = 0;
                size += write_uint8(buf, 0x0d);
                size += write_varuint32(buf, *depth);
                size
            },
            &BrTable(ref br_target) => {
                let mut size = 0;
                size += write_uint8(buf, 0x0e);
                size += br_target.dump(buf);
                size
            },
            &Return => write_uint8(buf, 0x0f),
        }
    }
}


#[derive(Debug, Clone)]
pub struct BrTarget {
    table: Vec<u32>,
    default_targt: u32,
}

impl Dump for BrTarget {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        let table = &self.table;

        size += write_varuint32(buf, table.len() as u32);
        for t in table {
            size += write_varuint32(buf, *t);
        }

        size += write_varuint32(buf, self.default_targt);

        size
    }
}


#[derive(Debug, Clone)]
pub enum Call {
    Call{index: u32},
    CallIndirect{index: u32, reserved: bool},
}

impl Dump for Call {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::Call::*;
        match self {
            &Call{ref index} => {
                let mut size = 0;

                size += write_uint8(buf, 0x10);
                size += write_varuint32(buf, *index);

                size
            },

            &CallIndirect{ref index, ref reserved} => {
                let mut size = 0;

                size += write_uint8(buf, 0x11);
                size += write_varuint32(buf, *index);
                size += write_varuint1(buf, *reserved as u8);

                size
            },
        }
    }
}


#[derive(Debug, Clone)]
pub enum Parametric {
    Drop = 0x1a,
    Select = 0x1b,
}

impl Dump for Parametric {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_uint8(buf, self.clone() as u8)
    }
}


#[derive(Debug, Clone)]
pub enum VariableAccess {
    GetLocal(u32),
    SetLocal(u32),
    TeeLocal(u32),
    GetGlobal(u32),
    SetGlobal(u32),
}


impl Dump for VariableAccess {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::VariableAccess::*;
        let mut size = 0;
        match self {
            &GetLocal(ref i) => {
                size += write_uint8(buf, 0x20);
                size += write_varuint32(buf, *i);
            },
            &SetLocal(ref i) => {
                size += write_uint8(buf, 0x21);
                size += write_varuint32(buf, *i);
            },
            &TeeLocal(ref i) => {
                size += write_uint8(buf, 0x22);
                size += write_varuint32(buf, *i);
            },
            &GetGlobal(ref i) => {
                size += write_uint8(buf, 0x023);
                size += write_varuint32(buf, *i);
            },
            &SetGlobal(ref i) => {
                size += write_uint8(buf, 0x24);
                size += write_varuint32(buf, *i);
            },
        };
        size
    }
}


#[derive(Debug, Clone)]
pub enum MemoryRelated {
    I32Load{imm: MemoryImmediate},
    I64Load{imm: MemoryImmediate},
    F32Load{imm: MemoryImmediate},
    F64Load{imm: MemoryImmediate},
    I32Load8S{imm: MemoryImmediate},
    I32Load8U{imm: MemoryImmediate},
    I32Load16S{imm: MemoryImmediate},
    I32Load16U{imm: MemoryImmediate},
    I64Load8S{imm: MemoryImmediate},
    I64Load8U{imm: MemoryImmediate},
    I64Load16S{imm: MemoryImmediate},
    I64Load16U{imm: MemoryImmediate},
    I64load32S{imm: MemoryImmediate},
    I64load32U{imm: MemoryImmediate},
    I32Store{imm: MemoryImmediate},
    I64Store{imm: MemoryImmediate},
    F32Store{imm: MemoryImmediate},
    F64Store{imm: MemoryImmediate},
    I32Store8{imm: MemoryImmediate},
    I32Store16{imm: MemoryImmediate},
    I64Store8{imm: MemoryImmediate},
    I64Store16{imm: MemoryImmediate},
    I64Store32{imm: MemoryImmediate},
    CurrentMemory{ reserved: bool },
    GrowMemory{ reserved: bool },
}

impl Dump for MemoryRelated{
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::MemoryRelated::*;
        match self {
            &I32Load{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x28);
                size += imm.dump(buf);

                size
            },
            &I64Load{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x29);
                size += imm.dump(buf);

                size
            },
            &F32Load{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x2a);
                size += imm.dump(buf);

                size
            },
            &F64Load{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x2b);
                size += imm.dump(buf);

                size
            },
            &I32Load8S{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x2c);
                size += imm.dump(buf);

                size
            },
            &I32Load8U{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x2d);
                size += imm.dump(buf);

                size
            },
            &I32Load16S{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x2e);
                size += imm.dump(buf);

                size
            },
            &I32Load16U{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x2f);
                size += imm.dump(buf);

                size
            },
            &I64Load8S{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x30);
                size += imm.dump(buf);

                size
            },
            &I64Load8U{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x31);
                size += imm.dump(buf);

                size
            },
            &I64Load16S{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x32);
                size += imm.dump(buf);

                size
            },
            &I64Load16U{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x33);
                size += imm.dump(buf);

                size
            },
            &I64load32S{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x34);
                size += imm.dump(buf);

                size
            },
            &I64load32U{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x35);
                size += imm.dump(buf);

                size
            },
            &I32Store{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x36);
                size += imm.dump(buf);

                size
            },
            &I64Store{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x37);
                size += imm.dump(buf);

                size
            },
            &F32Store{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x38);
                size += imm.dump(buf);

                size
            },
            &F64Store{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x39);
                size += imm.dump(buf);

                size
            },
            &I32Store8{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x3a);
                size += imm.dump(buf);

                size
            },
            &I32Store16{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x3b);
                size += imm.dump(buf);

                size
            },
            &I64Store8{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x3c);
                size += imm.dump(buf);

                size
            },
            &I64Store16{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x3d);
                size += imm.dump(buf);

                size
            },
            &I64Store32{ref imm} => {
                let mut size = 0;

                size += write_uint8(buf, 0x3e);
                size += imm.dump(buf);

                size
            },
            &CurrentMemory{ref reserved} => {
                let mut size = 0;

                size += write_uint8(buf, 0x3f);
                size += write_varuint1(buf, *reserved as u8);

                size
            },
            &GrowMemory{ref reserved} => {
                let mut size = 0;

                size += write_uint8(buf, 0x40);
                size += write_varuint1(buf, *reserved as u8);

                size
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryImmediate {
    flags: u32,
    offset: u32,
}


impl Dump for MemoryImmediate {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        size += write_varuint32(buf, self.flags);
        size += write_varuint32(buf, self.offset);

        size
    }
}


#[derive(Debug, Clone)]
pub enum Constant {
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
}

impl Dump for Constant {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use Constant::*;

        match self {
            &I32Const(ref i) => {
                let mut size = 0;

                size += write_uint8(buf, 0x41);
                size += write_varint32(buf, *i);

                size
            },
            &I64Const(ref i) => {
                let mut size = 0;

                size += write_uint8(buf, 0x42);
                size += write_varint64(buf, *i);

                size
            },
            &F32Const(ref f) => {
                let mut size = 0;

                size += write_uint8(buf, 0x43);
                unsafe {
                    size += write_uint32(buf, ::std::mem::transmute::<f32, u32>(*f));
                }

                size
            },
            &F64Const(ref f) => {
                let mut size = 0;

                size += write_uint8(buf, 0x44);
                unsafe {
                    size += write_uint64(buf, ::std::mem::transmute::<f64, u64>(*f));
                }

                size
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Comparison {
    I32Eqz = 0x45,
    I32Eq = 0x46,
    I32NE = 0x47,
    I32LtS = 0x48,
    I32LtU = 0x49,
    I32GtS = 0x4a,
    I32GtU = 0x4b,
    I32LeS = 0x4c,
    I32LeU = 0x4d,
    I32GeS = 0x4e,
    I32GeU = 0x4f,
    I64Eqz = 0x50,
    I64Eq = 0x51,
    I64Ne = 0x52,
    I64LtS = 0x53,
    I64LtU = 0x54,
    I64GtS = 0x55,
    I64GtU = 0x56,
    I64LeS = 0x57,
    I64LeU = 0x58,
    I64GeS = 0x59,
    I64GeU = 0x5a,
    F32Eq = 0x5b,
    F32Ne = 0x5c,
    F32Lt = 0x5d,
    F32Gt = 0x5e,
    F32Le = 0x5f,
    F32Ge = 0x60,
    F64Eq = 0x61,
    F64Ne = 0x62,
    F64Lt = 0x63,
    F64Gt = 0x64,
    F64Le = 0x65,
    F64Ge = 0x66,
}

impl Dump for Comparison {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_uint8(buf, self.clone() as u8)
    }
}


#[derive(Debug, Clone)]
pub enum Numeric {
    I32Clz = 0x67,
    I32Ctz = 0x68,
    I32Popcnt = 0x69,
    I32Add = 0x6a,
    I32Sub = 0x6b,
    I32Mul = 0x6c,
    I32DivS = 0x6d,
    I32DivU = 0x6e,
    I32RemS = 0x6f,
    I32RemU = 0x70,
    I32And = 0x71,
    I32Or = 0x72,
    I32Xor = 0x73,
    I32Shl = 0x74,
    I32ShrS = 0x75,
    I32ShrU = 0x76,
    I32Rotl = 0x77,
    I32Rotr = 0x78,
    I64Clz = 0x79,
    I64Ctz = 0x7a,
    I64Popcnt = 0x7b,
    I64Add = 0x7c,
    I64Sub = 0x7d,
    I64Mul = 0x7e,
    I64DivS = 0x7f,
    I64DivU = 0x80,
    I64RemS = 0x81,
    I64RemU = 0x82,
    I64And = 0x83,
    I64Or = 0x84,
    I64Xor = 0x85,
    I64Shl = 0x86,
    I64ShrS = 0x87,
    I64ShrU = 0x88,
    I64Rotl = 0x89,
    I64Rotr = 0x8a,
    F32Abs = 0x8b,
    F32Neg = 0x8c,
    F32Ceil = 0x8d,
    F32Floor = 0x8e,
    F32Trunc = 0x8f,
    F32Nearest = 0x90,
    F32Sqrt = 0x91,
    F32Add = 0x92,
    F32Sub = 0x93,
    F32Mul = 0x94,
    F32Div = 0x95,
    F32Min = 0x96,
    F32Max = 0x97,
    F32Copysign = 0x98,
    F64Abs = 0x99,
    F64Neg = 0x9a,
    F64Ceil = 0x9b,
    F64Floor = 0x9c,
    F64Trunc = 0x9d,
    F64Nearest = 0x9e,
    F64Sqrt = 0x9f,
    F64Add = 0xa0,
    F64Sub = 0xa1,
    F64Mul = 0xa2,
    F64Div = 0xa3,
    F64Min = 0xa4,
    F64Max = 0xa5,
    F64Copysign = 0xa6,
}

impl Dump for Numeric {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_uint8(buf, self.clone() as u8)
    }
}



#[derive(Debug, Clone)]
pub enum Conversion {
    I32wrapI64 = 0xa7,
    I32TruncSF32 = 0xa8,
    I32TruncUF32 = 0xa9,
    I32TruncSF64 = 0xaa,
    I32TruncUF64 = 0xab,
    I64ExtendSI32 = 0xac,
    I64ExtendUI32 = 0xad,
    I64TruncSF32 = 0xae,
    I64TruncUF32 = 0xaf,
    I64TruncSF64 = 0xb0,
    I64TruncUF64 = 0xb1,
    F32ConvertSI32 = 0xb2,
    F32ConvertUI32 = 0xb3,
    F32ConvertSI64 = 0xb4,
    F32ConvertUI64 = 0xb5,
    F32DemoteF64 = 0xb6,
    F64ConvertSI32 = 0xb7,
    F64ConvertUI32 = 0xb8,
    F64ConvertSI64 = 0xb9,
    F64ConvertUI64 = 0xba,
    F64PromoteF32 = 0xbb,
}

impl Dump for Conversion {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_uint8(buf, self.clone() as u8)
    }
}


#[derive(Debug, Clone)]
pub enum ReInterpret {
    I32ReinterpretF32 = 0xbc,
    I64ReinterpretF64 = 0xbd,
    F32ReinterpretI32 = 0xbe,
    F64ReinterpretI64 = 0xbf,
}

impl Dump for ReInterpret {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_uint8(buf, self.clone() as u8)
    }
}
