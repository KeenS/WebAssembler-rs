use module::*;
use types::*;
use types::internal::*;

pub struct ModuleBuilder(Module);

macro_rules! gen_add {
    ($name: tt ($param: tt, $ty: ty) -> $ret: tt, $field: tt) => {
        pub fn $name(&mut self, ty: $ty) -> $ret {

            match &mut (self.0).$field {
                &mut Some(ref mut v) => {
                    v.push(ty);
                    $ret::new((v.len() - 1) as u32)
                },
                none => {
                    *none = Some(vec![ty]);
                    $ret::new(0)
                },
            }
        }
    }
}

impl ModuleBuilder {
    pub fn new() -> Self {
        ModuleBuilder(Module {
            unknown: None,
            types: None,
            imports: None,
            functions: None,
            tables: None,
            memories: None,
            globals: None,
            exports: None,
            start: None,
            elements: None,
            codes: None,
            data: None,
        })
    }

    pub fn build(self) -> Module {
        self.0
    }


    gen_add!(add_type(ty, FuncType) -> TypeIndex,
             types);
    gen_add!(add_import(import, Import) -> ImportIndex,
             imports);
    gen_add!(add_function(func, Function) -> FunctionIndex,
             functions);
    gen_add!(add_table(table, TableType) -> TableIndex,
             tables);
    gen_add!(add_memorie(memory, MemoryType) -> MemoryIndex,
             memories);
    gen_add!(add_global(global, GlobalVariable) -> GlobalIndex,
             globals);
    gen_add!(add_export(export, ExportEntry) -> ExportIndex,
             exports);
    pub fn add_start(&mut self, index: FunctionIndex) {
        self.0.start = Some(index);
    }
    gen_add!(add_element(element, ElemSegment) -> ElementIndex,
                 elements);
    gen_add!(add_code(code, FunctionBody) -> CodeIndex,
             codes);
    gen_add!(add_data(data, DataSegment) -> DataIndex,
             data);
}

pub trait Export<T> {
    fn export<S: Into<String>>(&mut self, name: S, index: T) -> ExportIndex;
}

impl Export<FunctionIndex> for ModuleBuilder {
    fn export<S: Into<String>>(&mut self, name: S, index: FunctionIndex) -> ExportIndex {
        let entry = ExportEntry {
            field: name.into(),
            kind: ExportKind::Function(index),
        };
        self.add_export(entry)
    }
}

// impl Export<TableIndex> for ModuleBuilder {
//     fn export<S: Into<String>>(&mut self, name: S, index: TableIndex) -> ExportIndex {
//         let entry = ExportEntry {
//             field: name.into(),
//             kind: ExportKind::Function(index),
//         };
//         self.add_export(entry)
//     }
// }



#[macro_export]
macro_rules! ty {
    (i32) => (ValueType::I32)
}


#[macro_export]
macro_rules! ty_vec {
    ($($t: tt, )*) => {
        vec!($(ty!($t), )*)
    };

    ($t: tt $(, $ts: tt)*) => {
        vec!(ty!($t) $(, ty!($ts))*)
    };
}

#[macro_export]
macro_rules! funtype {
    (($($params: tt)*) -> $ret: tt) => {
        FuncType {
            params: ty_vec!($($params)*),
            ret: Some(ty!($ret))
        }
    };
}
