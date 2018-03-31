use types::*;
use util::*;
use Dump;

#[derive(Debug, Clone)]
pub enum Op {
    Unreachable,
    Nop,
    Block { sig: BlockType },
    Loop { sig: BlockType },
    If,
    Else,
    End,
    // TODO: use relative block index
    Br { depth: u32 },
    BrIf { depth: u32 },
    BrTable(BrTarget),
    Return,
    Call { index: FunctionSpaceIndex },
    // TODO: use table index
    CallIndirect { index: u32, reserved: bool },
    Drop,
    Select,
    GetLocal(LocalIndex),
    SetLocal(LocalIndex),
    TeeLocal(LocalIndex),
    GetGlobal(GlobalIndex),
    SetGlobal(GlobalIndex),
    I32Load { imm: MemoryImmediate },
    I64Load { imm: MemoryImmediate },
    F32Load { imm: MemoryImmediate },
    F64Load { imm: MemoryImmediate },
    I32Load8S { imm: MemoryImmediate },
    I32Load8U { imm: MemoryImmediate },
    I32Load16S { imm: MemoryImmediate },
    I32Load16U { imm: MemoryImmediate },
    I64Load8S { imm: MemoryImmediate },
    I64Load8U { imm: MemoryImmediate },
    I64Load16S { imm: MemoryImmediate },
    I64Load16U { imm: MemoryImmediate },
    I64load32S { imm: MemoryImmediate },
    I64load32U { imm: MemoryImmediate },
    I32Store { imm: MemoryImmediate },
    I64Store { imm: MemoryImmediate },
    F32Store { imm: MemoryImmediate },
    F64Store { imm: MemoryImmediate },
    I32Store8 { imm: MemoryImmediate },
    I32Store16 { imm: MemoryImmediate },
    I64Store8 { imm: MemoryImmediate },
    I64Store16 { imm: MemoryImmediate },
    I64Store32 { imm: MemoryImmediate },
    CurrentMemory { reserved: bool },
    GrowMemory { reserved: bool },
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32NE,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32wrapI64,
    I32TruncSF32,
    I32TruncUF32,
    I32TruncSF64,
    I32TruncUF64,
    I64ExtendSI32,
    I64ExtendUI32,
    I64TruncSF32,
    I64TruncUF32,
    I64TruncSF64,
    I64TruncUF64,
    F32ConvertSI32,
    F32ConvertUI32,
    F32ConvertSI64,
    F32ConvertUI64,
    F32DemoteF64,
    F64ConvertSI32,
    F64ConvertUI32,
    F64ConvertSI64,
    F64ConvertUI64,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}

impl Op {
    pub fn resolve_functions(&mut self, nimports: u32) {
        match *self {
            Op::Call { ref mut index } => {
                use InnerFunctionSpaceIndex::*;
                match index.0 {
                    Function(ref mut f) => {
                        f.0 += nimports;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

impl Dump for Op {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::Op::*;
        fn do_imm(buf: &mut Vec<u8>, imm: &MemoryImmediate, code: u8) -> usize {
            let mut size = 0;

            size += write_uint8(buf, code);
            size += imm.dump(buf);

            size

        };
        let mut size = 0;

        match self {
            &Unreachable => size += write_uint8(buf, 0x00),
            &Nop => size += write_uint8(buf, 0x01),
            &Block { ref sig } => {
                size += write_uint8(buf, 0x02);
                size += sig.dump(buf);
            }
            &Loop { ref sig } => {
                size += write_uint8(buf, 0x03);
                size += sig.dump(buf);
            }
            &If => size += write_uint8(buf, 0x04),
            &Else => size += write_uint8(buf, 0x05),
            &End => size += write_uint8(buf, 0x0b),
            &Br { ref depth } => {
                size += write_uint8(buf, 0x0c);
                size += write_varuint32(buf, *depth);
            }
            &BrIf { ref depth } => {
                size += write_uint8(buf, 0x0d);
                size += write_varuint32(buf, *depth);
            }
            &BrTable(ref br_target) => {
                size += write_uint8(buf, 0x0e);
                size += br_target.dump(buf);
            }
            &Return => size += write_uint8(buf, 0x0f),
            &Call { ref index } => {
                size += write_uint8(buf, 0x10);
                size += write_varuint32(buf, **index);
            }

            &CallIndirect {
                 ref index,
                 ref reserved,
             } => {
                size += write_uint8(buf, 0x11);
                size += write_varuint32(buf, *index);
                size += write_varuint1(buf, *reserved as u8);
            }
            &Drop => size += write_uint8(buf, 0x1a),
            &Select => size += write_uint8(buf, 0x1b),
            &GetLocal(ref i) => {
                size += write_uint8(buf, 0x20);
                size += write_varuint32(buf, **i);
            }
            &SetLocal(ref i) => {
                size += write_uint8(buf, 0x21);
                size += write_varuint32(buf, **i);
            }
            &TeeLocal(ref i) => {
                size += write_uint8(buf, 0x22);
                size += write_varuint32(buf, **i);
            }
            &GetGlobal(ref i) => {
                size += write_uint8(buf, 0x023);
                size += write_varuint32(buf, **i);
            }
            &SetGlobal(ref i) => {
                size += write_uint8(buf, 0x24);
                size += write_varuint32(buf, **i);
            }
            &I32Load { ref imm } => size += do_imm(buf, imm, 0x28),
            &I64Load { ref imm } => size += do_imm(buf, imm, 0x29),
            &F32Load { ref imm } => size += do_imm(buf, imm, 0x2a),
            &F64Load { ref imm } => size += do_imm(buf, imm, 0x2b),
            &I32Load8S { ref imm } => size += do_imm(buf, imm, 0x2c),
            &I32Load8U { ref imm } => size += do_imm(buf, imm, 0x2d),
            &I32Load16S { ref imm } => size += do_imm(buf, imm, 0x2e),
            &I32Load16U { ref imm } => size += do_imm(buf, imm, 0x2f),
            &I64Load8S { ref imm } => size += do_imm(buf, imm, 0x30),
            &I64Load8U { ref imm } => size += do_imm(buf, imm, 0x31),
            &I64Load16S { ref imm } => size += do_imm(buf, imm, 0x32),
            &I64Load16U { ref imm } => size += do_imm(buf, imm, 0x33),
            &I64load32S { ref imm } => size += do_imm(buf, imm, 0x34),
            &I64load32U { ref imm } => size += do_imm(buf, imm, 0x35),
            &I32Store { ref imm } => size += do_imm(buf, imm, 0x36),
            &I64Store { ref imm } => size += do_imm(buf, imm, 0x37),
            &F32Store { ref imm } => size += do_imm(buf, imm, 0x38),
            &F64Store { ref imm } => size += do_imm(buf, imm, 0x39),
            &I32Store8 { ref imm } => size += do_imm(buf, imm, 0x3a),
            &I32Store16 { ref imm } => size += do_imm(buf, imm, 0x3b),
            &I64Store8 { ref imm } => size += do_imm(buf, imm, 0x3c),
            &I64Store16 { ref imm } => size += do_imm(buf, imm, 0x3d),
            &I64Store32 { ref imm } => size += do_imm(buf, imm, 0x3e),
            &CurrentMemory { ref reserved } => {
                size += write_uint8(buf, 0x3f);
                size += write_varuint1(buf, *reserved as u8);
            }
            &GrowMemory { ref reserved } => {
                size += write_uint8(buf, 0x40);
                size += write_varuint1(buf, *reserved as u8);
            }
            &I32Const(ref i) => {
                size += write_uint8(buf, 0x41);
                size += write_varint32(buf, *i);
            }
            &I64Const(ref i) => {
                size += write_uint8(buf, 0x42);
                size += write_varint64(buf, *i);
            }
            &F32Const(ref f) => {
                size += write_uint8(buf, 0x43);
                unsafe {
                    size += write_uint32(buf, ::std::mem::transmute::<f32, u32>(*f));
                }
            }
            &F64Const(ref f) => {
                size += write_uint8(buf, 0x44);
                unsafe {
                    size += write_uint64(buf, ::std::mem::transmute::<f64, u64>(*f));
                }
            }
            &I32Eqz => size += write_uint8(buf, 0x45),
            &I32Eq => size += write_uint8(buf, 0x46),
            &I32NE => size += write_uint8(buf, 0x47),
            &I32LtS => size += write_uint8(buf, 0x48),
            &I32LtU => size += write_uint8(buf, 0x49),
            &I32GtS => size += write_uint8(buf, 0x4a),
            &I32GtU => size += write_uint8(buf, 0x4b),
            &I32LeS => size += write_uint8(buf, 0x4c),
            &I32LeU => size += write_uint8(buf, 0x4d),
            &I32GeS => size += write_uint8(buf, 0x4e),
            &I32GeU => size += write_uint8(buf, 0x4f),
            &I64Eqz => size += write_uint8(buf, 0x50),
            &I64Eq => size += write_uint8(buf, 0x51),
            &I64Ne => size += write_uint8(buf, 0x52),
            &I64LtS => size += write_uint8(buf, 0x53),
            &I64LtU => size += write_uint8(buf, 0x54),
            &I64GtS => size += write_uint8(buf, 0x55),
            &I64GtU => size += write_uint8(buf, 0x56),
            &I64LeS => size += write_uint8(buf, 0x57),
            &I64LeU => size += write_uint8(buf, 0x58),
            &I64GeS => size += write_uint8(buf, 0x59),
            &I64GeU => size += write_uint8(buf, 0x5a),
            &F32Eq => size += write_uint8(buf, 0x5b),
            &F32Ne => size += write_uint8(buf, 0x5c),
            &F32Lt => size += write_uint8(buf, 0x5d),
            &F32Gt => size += write_uint8(buf, 0x5e),
            &F32Le => size += write_uint8(buf, 0x5f),
            &F32Ge => size += write_uint8(buf, 0x60),
            &F64Eq => size += write_uint8(buf, 0x61),
            &F64Ne => size += write_uint8(buf, 0x62),
            &F64Lt => size += write_uint8(buf, 0x63),
            &F64Gt => size += write_uint8(buf, 0x64),
            &F64Le => size += write_uint8(buf, 0x65),
            &F64Ge => size += write_uint8(buf, 0x66),
            &I32Clz => size += write_uint8(buf, 0x67),
            &I32Ctz => size += write_uint8(buf, 0x68),
            &I32Popcnt => size += write_uint8(buf, 0x69),
            &I32Add => size += write_uint8(buf, 0x6a),
            &I32Sub => size += write_uint8(buf, 0x6b),
            &I32Mul => size += write_uint8(buf, 0x6c),
            &I32DivS => size += write_uint8(buf, 0x6d),
            &I32DivU => size += write_uint8(buf, 0x6e),
            &I32RemS => size += write_uint8(buf, 0x6f),
            &I32RemU => size += write_uint8(buf, 0x70),
            &I32And => size += write_uint8(buf, 0x71),
            &I32Or => size += write_uint8(buf, 0x72),
            &I32Xor => size += write_uint8(buf, 0x73),
            &I32Shl => size += write_uint8(buf, 0x74),
            &I32ShrS => size += write_uint8(buf, 0x75),
            &I32ShrU => size += write_uint8(buf, 0x76),
            &I32Rotl => size += write_uint8(buf, 0x77),
            &I32Rotr => size += write_uint8(buf, 0x78),
            &I64Clz => size += write_uint8(buf, 0x79),
            &I64Ctz => size += write_uint8(buf, 0x7a),
            &I64Popcnt => size += write_uint8(buf, 0x7b),
            &I64Add => size += write_uint8(buf, 0x7c),
            &I64Sub => size += write_uint8(buf, 0x7d),
            &I64Mul => size += write_uint8(buf, 0x7e),
            &I64DivS => size += write_uint8(buf, 0x7f),
            &I64DivU => size += write_uint8(buf, 0x80),
            &I64RemS => size += write_uint8(buf, 0x81),
            &I64RemU => size += write_uint8(buf, 0x82),
            &I64And => size += write_uint8(buf, 0x83),
            &I64Or => size += write_uint8(buf, 0x84),
            &I64Xor => size += write_uint8(buf, 0x85),
            &I64Shl => size += write_uint8(buf, 0x86),
            &I64ShrS => size += write_uint8(buf, 0x87),
            &I64ShrU => size += write_uint8(buf, 0x88),
            &I64Rotl => size += write_uint8(buf, 0x89),
            &I64Rotr => size += write_uint8(buf, 0x8a),
            &F32Abs => size += write_uint8(buf, 0x8b),
            &F32Neg => size += write_uint8(buf, 0x8c),
            &F32Ceil => size += write_uint8(buf, 0x8d),
            &F32Floor => size += write_uint8(buf, 0x8e),
            &F32Trunc => size += write_uint8(buf, 0x8f),
            &F32Nearest => size += write_uint8(buf, 0x90),
            &F32Sqrt => size += write_uint8(buf, 0x91),
            &F32Add => size += write_uint8(buf, 0x92),
            &F32Sub => size += write_uint8(buf, 0x93),
            &F32Mul => size += write_uint8(buf, 0x94),
            &F32Div => size += write_uint8(buf, 0x95),
            &F32Min => size += write_uint8(buf, 0x96),
            &F32Max => size += write_uint8(buf, 0x97),
            &F32Copysign => size += write_uint8(buf, 0x98),
            &F64Abs => size += write_uint8(buf, 0x99),
            &F64Neg => size += write_uint8(buf, 0x9a),
            &F64Ceil => size += write_uint8(buf, 0x9b),
            &F64Floor => size += write_uint8(buf, 0x9c),
            &F64Trunc => size += write_uint8(buf, 0x9d),
            &F64Nearest => size += write_uint8(buf, 0x9e),
            &F64Sqrt => size += write_uint8(buf, 0x9f),
            &F64Add => size += write_uint8(buf, 0xa0),
            &F64Sub => size += write_uint8(buf, 0xa1),
            &F64Mul => size += write_uint8(buf, 0xa2),
            &F64Div => size += write_uint8(buf, 0xa3),
            &F64Min => size += write_uint8(buf, 0xa4),
            &F64Max => size += write_uint8(buf, 0xa5),
            &F64Copysign => size += write_uint8(buf, 0xa6),
            &I32wrapI64 => size += write_uint8(buf, 0xa7),
            &I32TruncSF32 => size += write_uint8(buf, 0xa8),
            &I32TruncUF32 => size += write_uint8(buf, 0xa9),
            &I32TruncSF64 => size += write_uint8(buf, 0xaa),
            &I32TruncUF64 => size += write_uint8(buf, 0xab),
            &I64ExtendSI32 => size += write_uint8(buf, 0xac),
            &I64ExtendUI32 => size += write_uint8(buf, 0xad),
            &I64TruncSF32 => size += write_uint8(buf, 0xae),
            &I64TruncUF32 => size += write_uint8(buf, 0xaf),
            &I64TruncSF64 => size += write_uint8(buf, 0xb0),
            &I64TruncUF64 => size += write_uint8(buf, 0xb1),
            &F32ConvertSI32 => size += write_uint8(buf, 0xb2),
            &F32ConvertUI32 => size += write_uint8(buf, 0xb3),
            &F32ConvertSI64 => size += write_uint8(buf, 0xb4),
            &F32ConvertUI64 => size += write_uint8(buf, 0xb5),
            &F32DemoteF64 => size += write_uint8(buf, 0xb6),
            &F64ConvertSI32 => size += write_uint8(buf, 0xb7),
            &F64ConvertUI32 => size += write_uint8(buf, 0xb8),
            &F64ConvertSI64 => size += write_uint8(buf, 0xb9),
            &F64ConvertUI64 => size += write_uint8(buf, 0xba),
            &F64PromoteF32 => size += write_uint8(buf, 0xbb),
            &I32ReinterpretF32 => size += write_uint8(buf, 0xbc),
            &I64ReinterpretF64 => size += write_uint8(buf, 0xbd),
            &F32ReinterpretI32 => size += write_uint8(buf, 0xbe),
            &F64ReinterpretI64 => size += write_uint8(buf, 0xbf),
        };
        size
    }
}

impl From<i32> for Op {
    fn from(i: i32) -> Self {
        Op::I32Const(i)
    }
}
impl From<i64> for Op {
    fn from(i: i64) -> Self {
        Op::I64Const(i)
    }
}
impl From<f32> for Op {
    fn from(f: f32) -> Self {
        Op::F32Const(f)
    }
}
impl From<f64> for Op {
    fn from(f: f64) -> Self {
        Op::F64Const(f)
    }
}



#[derive(Debug, Clone)]
pub struct MemoryImmediate {
    pub flags: u32,
    pub offset: u32,
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
pub struct BrTarget {
    // TODO: use relative block index
    pub table: Vec<u32>,
    pub default_target: u32,
}

impl Dump for BrTarget {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        let table = &self.table;

        size += write_varuint32(buf, table.len() as u32);
        for t in table {
            size += write_varuint32(buf, *t);
        }

        size += write_varuint32(buf, self.default_target);

        size
    }
}
