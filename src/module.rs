use util::*;
use types::*;
use Dump;

#[derive(Debug, Clone)]
pub struct Module {
//    version: usize
    pub sections: Vec<Section>
}

impl Dump for Module {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        let magic = b"\0asm";
        size += write_slice(buf, magic);

        let version = 0x0d;
        size += write_uint32(buf, version);

        for sec in self.sections.iter() {
            size += sec.dump(buf);
        }

        size
    }
}

#[derive(Debug, Clone)]
pub enum Section {
    UNKNOWN(String),
    TYPE(Vec<FuncType>),
    IMPORT(Vec<Import>),
    FUNCTION(Vec<Function>),
    TABLE(Vec<TableType>),
    MEMORY(Vec<MemoryType>),
    GLOBAL(Vec<GlobalVariable>),
    EXPORT(Vec<ExportEntry>),
    START(u32),
    ELEMENT(Vec<ElemSegment>),
    CODE(Vec<FunctionBody>),
    DATA(Vec<DataSegment>),
}

impl Section {
    fn code(&self) -> u8 {
        use Section::*;
        match self {
            &UNKNOWN(_) => 0,
            &TYPE(_) => 1,
            &IMPORT(_) => 2,
            &FUNCTION(_) => 3,
            &TABLE(_) => 4,
            &MEMORY(_) => 5,
            &GLOBAL(_) => 6,
            &EXPORT(_) => 7,
            &START(_) => 8,
            &ELEMENT(_) => 9,
            &CODE(_) => 10,
            &DATA(_) => 11,
        }
    }
}

impl Dump for Section {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        use Section::*;
        let mut size: usize = 0;

        size += write_uint8(buf, self.code());
        let mut v = Vec::new();
        let sec;
        let section_size;
        {
            let buf = &mut v;
            section_size = match self {
                &UNKNOWN(_) => 0,
                &TYPE(ref types) => {
                    let mut size = 0;
                    size += write_varuint32(buf, types.len() as u32);
                    for t in types {
                        size += t.dump(buf);
                    }
                    size
                },
                &IMPORT(ref imports) => {
                    let mut size = 0;
                    size += write_varuint32(buf, imports.len() as u32);
                    for i in imports {
                        size += i.dump(buf);
                    }
                    size
                },
                &FUNCTION(ref functions) => {
                    let mut size = 0;
                    size += write_varuint32(buf, functions.len() as u32);
                    for f in functions {
                        size += f.dump(buf);
                    }
                    size
                },
                &TABLE(ref tbls) => {
                    let mut size = 0;
                    size += write_varuint32(buf, tbls.len() as u32);
                    for t in tbls {
                        size += t.dump(buf);
                    }
                    size
                },
                &MEMORY(ref mems) => {
                    let mut size = 0;
                    size += write_varuint32(buf, mems.len() as u32);
                    for m in mems {
                        size += m.dump(buf);
                    }
                    size
                },
                &GLOBAL(ref glbs) => {
                    let mut size = 0;
                    size += write_varuint32(buf, glbs.len() as u32);
                    for g in glbs {
                        size += g.dump(buf);
                    }
                    size
                },
                &EXPORT(ref exps) => {
                    let mut size = 0;
                    size += write_varuint32(buf, exps.len() as u32);
                    for e in exps {
                        size += e.dump(buf);
                    }
                    size
                },
                &START(ref index) => {
                    write_varuint32(buf, *index)
                },
                &ELEMENT(ref elms) => {
                    let mut size = 0;
                    size += write_varuint32(buf, elms.len() as u32);
                    for e in elms {
                        size += e.dump(buf);
                    }
                    size
                },
                &CODE(ref bodies) => {
                    let mut size = 0;
                    size += write_varuint32(buf, bodies.len() as u32);
                    for b in bodies {
                        size += b.dump(buf);
                    }
                    size
                },
                &DATA(ref data_segs) => {
                    let mut size = 0;
                    size += write_varuint32(buf, data_segs.len() as u32);
                    for s in data_segs {
                        size += s.dump(buf);
                    }
                    size

                },
            };
            sec = buf;
        };
        size += write_varuint32(buf, section_size as u32);
        size += write_slice(buf, &sec);
        size
    }
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub field: String,
    pub kind: ImportKind,
}

#[derive(Debug, Clone)]
pub enum ImportKind {
    Function(u32),
    Table(TableType),
    Memory(MemoryType),
    GlobalType(GlobalType),
}

impl Dump for Import {
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
                size += write_varuint32(buf, *id);
                size
            },
            &Table(ref tbl) => {
                size += write_uint8(buf, 1);
                size += tbl.dump(buf);
                size
            },
            &Memory(ref m) => {
                size += write_uint8(buf, 2);
                size += m.dump(buf);
                size

            },
            &GlobalType(ref glb) => {
                size += write_uint8(buf, 3);
                size += glb.dump(buf);
                size

            },

        }
    }
}


#[derive(Debug, Clone)]
pub struct Function(pub u32);

impl Dump for Function {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        write_varuint32(buf, self.0)
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
    pub kind: ExternalKind,
    pub index: u32,
}

impl Dump for ExportEntry {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;
        let field = &self.field;

        size += write_varuint32(buf, field.len() as u32);
        size += write_slice(buf, field.as_bytes());

        size += self.kind.dump(buf);

        size += write_varuint32(buf, self.index);

        size
    }
}

#[derive(Debug, Clone)]
pub struct ElemSegment {
    pub index: u32,
    pub offest: InitExpr,
    pub elems: Vec<u32>,
}

impl Dump for ElemSegment {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        let elems = &self.elems;

        size += write_varuint32(buf, self.index);
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
    pub code: Vec<u8>,
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

            size += write_slice(buf, code);

            body_size = size;
        }


        size += write_varuint32(buf, body_size as u32);
        size += write_slice(buf, &body);

        size
    }
}


#[derive(Debug, Clone)]
pub struct LocalEntry {
    count: u32,
    ty: ValueType,
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
    index: u32,
    offset: InitExpr,
    data: Vec<u8>,
}

impl Dump for DataSegment {
    fn dump(&self, buf: &mut Vec<u8>) -> usize {
        let mut size = 0;

        let data = &self.data;

        size += write_varuint32(buf, self.index);
        size += self.offset.dump(buf);

        size += write_varuint32(buf, data.len() as u32);
        size += write_slice(buf, data);

        size
    }
}

