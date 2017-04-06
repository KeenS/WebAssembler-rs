use util::*;
use types::*;
use Dump;

#[derive(Debug, Clone)]
pub struct Module {
    //    version: usize
    pub unknown: Option<String>,
    pub types: Option<Vec<FuncType>>,
    pub imports: Option<Vec<ImportEntry>>,
    pub functions: Option<Vec<Function>>,
    pub tables: Option<Vec<TableType>>,
    pub memories: Option<Vec<MemoryType>>,
    pub globals: Option<Vec<GlobalVariable>>,
    pub exports: Option<Vec<ExportEntry>>,
    pub start: Option<FunctionIndex>,
    pub elements: Option<Vec<ElemSegment>>,
    pub codes: Option<Vec<FunctionBody>>,
    pub data: Option<Vec<DataSegment>>,
}

impl Dump for Module {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        let magic = b"\0asm";
        size += write_slice(buf, magic);

        let version = 0x01;
        size += write_uint32(buf, version);


        let mut v = Vec::new();
        macro_rules! do_section {
            ($code: expr, $field: expr) => {{
                let field = &$field;
                for xs in field {
                    v.clear();
                    let mut section_size = 0;
                    let sec = &mut v;
                    section_size += write_varuint32(sec, xs.len() as u32);
                    for x in xs {
                        section_size += x.dump(sec);
                    }

                    size += write_uint8(buf, $code);
                    size += write_varuint32(buf, section_size as u32);
                    size += write_slice(buf, &sec);
                }
            }};
        }


        do_section!(0x01, self.types);
        do_section!(0x02, self.imports);
        do_section!(0x03, self.functions);
        do_section!(0x04, self.tables);
        do_section!(0x05, self.memories);
        do_section!(0x06, self.globals);
        do_section!(0x07, self.exports);
        self.start
            .as_ref()
            .map(|index| {
                     size += write_uint8(buf, 0x08);
                     size += write_varuint32(buf, **index)
                 })
            .unwrap_or(());
        do_section!(0x09, self.elements);
        do_section!(0x0a, self.codes);
        do_section!(0x0b, self.data);
        size
    }
}

#[derive(Debug, Clone)]
pub struct ImportEntry {
    pub module: String,
    pub field: String,
    pub kind: ImportKind,
}

#[derive(Debug, Clone)]
pub enum ImportKind {
    Function(TypeIndex),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

impl Dump for ImportEntry {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        let module = &self.module;
        let field = &self.field;

        size += write_varuint32(buf, module.len() as u32);
        size += write_slice(buf, module.as_bytes());

        size += write_varuint32(buf, field.len() as u32);
        size += write_slice(buf, field.as_bytes());

        size += self.kind.dump(buf);

        size

    }
}

impl Dump for ImportKind {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::ImportKind::*;
        let mut size = 0;
        match self {
            &Function(ref id) => {
                size += write_uint8(buf, 0);
                size += write_varuint32(buf, **id);
                size
            }
            &Table(ref tbl) => {
                size += write_uint8(buf, 1);
                size += tbl.dump(buf);
                size
            }
            &Memory(ref m) => {
                size += write_uint8(buf, 2);
                size += m.dump(buf);
                size

            }
            &Global(ref glb) => {
                size += write_uint8(buf, 3);
                size += glb.dump(buf);
                size

            }

        }
    }
}


#[derive(Debug, Clone)]
pub struct Function(pub TypeIndex);

impl Dump for Function {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_varuint32(buf, *self.0)
    }
}

#[derive(Debug, Clone)]
pub struct GlobalVariable {
    pub ty: GlobalType,
    pub init: InitExpr,
}


impl Dump for GlobalVariable {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        size += self.ty.dump(buf);
        size += self.init.dump(buf);
        size
    }
}

#[derive(Debug, Clone)]
pub struct ExportEntry {
    pub field: String,
    pub kind: ExportKind,
}

#[derive(Debug, Clone)]
pub enum ExportKind {
    Function(FunctionIndex),
    Table(TableIndex),
    Memory(MemoryIndex),
    Global(GlobalIndex),
}

impl Dump for ExportEntry {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        let field = &self.field;

        size += write_varuint32(buf, field.len() as u32);
        size += write_slice(buf, field.as_bytes());

        size += self.kind.dump(buf);

        size
    }
}

impl Dump for ExportKind {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use self::ExportKind::*;
        let mut size = 0;
        match self {
            &Function(ref i) => {
                size += write_uint8(buf, 0);
                size += write_varuint32(buf, **i);
            }
            &Table(ref i) => {
                size += write_uint8(buf, 1);
                size += write_varuint32(buf, **i);
            }
            &Memory(ref i) => {
                size += write_uint8(buf, 2);
                size += write_varuint32(buf, **i);
            }
            &Global(ref i) => {
                size += write_uint8(buf, 3);
                size += write_varuint32(buf, **i);
            }
        }
        size
    }
}

#[derive(Debug, Clone)]
pub struct ElemSegment {
    pub index: TableIndex,
    pub offest: InitExpr,
    // TODO: use function indices
    pub elems: Vec<u32>,
}

impl Dump for ElemSegment {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        let elems = &self.elems;

        size += write_varuint32(buf, *self.index);
        size += self.offest.dump(buf);

        size += write_varuint32(buf, elems.len() as u32);
        for e in elems.iter() {
            size += write_varuint32(buf, *e);

        }
        size
    }
}

#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub locals: Vec<LocalEntry>,
    pub code: Code,
}

impl Dump for FunctionBody {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        let locals = &self.locals;
        let code = &self.code;

        let body_size;
        let mut body = Vec::new();
        {
            let mut size = 0;
            let buf = &mut body;
            size += write_varuint32(buf, locals.len() as u32);
            for l in locals.iter() {
                size += l.dump(buf);
            }

            size += code.dump(buf);
            size += write_uint8(buf, 0x0b);

            body_size = size;
        }


        size += write_varuint32(buf, body_size as u32);
        size += write_slice(buf, &body);

        size
    }
}


#[derive(Debug, Clone)]
pub struct LocalEntry {
    pub count: u32,
    pub ty: ValueType,
}


impl Dump for LocalEntry {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        size += write_varuint32(buf, self.count);
        size += self.ty.dump(buf);

        size
    }
}

#[derive(Debug, Clone)]
pub struct DataSegment {
    pub index: MemoryIndex,
    pub offset: InitExpr,
    pub data: Vec<u8>,
}

impl Dump for DataSegment {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        let data = &self.data;

        size += write_varuint32(buf, *self.index);
        size += self.offset.dump(buf);

        size += write_varuint32(buf, data.len() as u32);
        size += write_slice(buf, data);

        size
    }
}
