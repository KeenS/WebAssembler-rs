use std::ops::{Range, RangeFrom};

use module::*;
use types::*;
use ops;
use ops::Op;
use Dump;
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
    };

    (prv, $name: tt ($param: tt, $ty: ty) -> $ret: tt, $field: tt) => {
        fn $name(&mut self, ty: $ty) -> $ret {

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
    };
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
    gen_add!(add_import(import, ImportEntry) -> ImportIndex,
             imports);
    gen_add!(prv, add_function(func, Function) -> FunctionIndex,
             functions);
    gen_add!(add_table(table, TableType) -> TableIndex,
             tables);
    gen_add!(add_memory(memory, MemoryType) -> MemoryIndex,
             memories);
    gen_add!(add_global(global, GlobalVariable) -> GlobalIndex,
             globals);
    gen_add!(add_export(export, ExportEntry) -> ExportIndex,
             exports);
    pub fn start(&mut self, index: FunctionIndex) {
        self.0.start = Some(index);
    }
    gen_add!(add_element(element, ElemSegment) -> ElementIndex,
                 elements);
    gen_add!(prv, add_code(code, FunctionBody) -> CodeIndex,
             codes);
    gen_add!(add_data(data, DataSegment) -> DataIndex,
             data);

    pub fn new_function(&mut self, (t, body): (FuncType, FunctionBody)) -> FunctionIndex {
        let tidx = self.add_type(t);
        let fidx = self.add_function(Function(tidx));
        let cidx = self.add_code(body);
        assert_eq!(*cidx, *fidx);
        fidx
    }

    pub fn new_data(&mut self, idx: MemoryIndex, offset: Code, data: Vec<u8>) -> DataIndex {
        let seg = DataSegment {
            index: idx,
            offset: InitExpr(offset),
            data: data,
        };
        self.add_data(seg)
    }

    pub fn new_global(&mut self, ty: GlobalType, init: Code) -> GlobalIndex {
        self.add_global(GlobalVariable {
                            ty: ty,
                            init: InitExpr(init),
                        })
    }
}

pub trait NewTable<T> {
    fn new_table(&mut self, element: ElemType, range: T) -> TableIndex;
}

impl NewTable<Range<u32>> for ModuleBuilder {
    fn new_table(&mut self, element: ElemType, range: Range<u32>) -> TableIndex {
        let table = TableType {
            element: element,
            limits: ResizableLimits::new(range.start).max(range.end),
        };
        self.add_table(table)
    }
}

impl NewTable<RangeFrom<u32>> for ModuleBuilder {
    fn new_table(&mut self, element: ElemType, range: RangeFrom<u32>) -> TableIndex {
        let table = TableType {
            element: element,
            limits: ResizableLimits::new(range.start),
        };
        self.add_table(table)
    }
}

pub trait NewMemory<T> {
    fn new_memory(&mut self, range: T) -> MemoryIndex;
}

impl NewMemory<Range<u32>> for ModuleBuilder {
    fn new_memory(&mut self, range: Range<u32>) -> MemoryIndex {
        let memory = MemoryType { limits: ResizableLimits::new(range.start).max(range.end) };
        self.add_memory(memory)
    }
}

impl NewMemory<RangeFrom<u32>> for ModuleBuilder {
    fn new_memory(&mut self, range: RangeFrom<u32>) -> MemoryIndex {
        let memory = MemoryType { limits: ResizableLimits::new(range.start) };
        self.add_memory(memory)
    }
}

pub trait Export<T> {
    fn export<S: Into<String>>(&mut self, name: S, index: T) -> ExportIndex;
}


macro_rules! gen_export {
    ($name: ty, $variant: tt) => {
        impl Export<$name> for ModuleBuilder {
        fn export<S: Into<String>>(&mut self, name: S, index: $name) -> ExportIndex {
            let entry = ExportEntry {
                field: name.into(),
                kind: ExportKind::$variant(index),
            };
            self.add_export(entry)
        }
        }
    }
}

gen_export!(FunctionIndex, Function);
gen_export!(TableIndex, Table);
gen_export!(MemoryIndex, Memory);
gen_export!(GlobalIndex, Global);


pub trait Import<Ty> {
    fn import<S, T>(&mut self, module: S, name: T, index: Ty) -> ImportIndex
        where S: Into<String>,
              T: Into<String>;
}

macro_rules! gen_import {
    ($name: ty, $variant: tt) => {
        impl Import<$name> for ModuleBuilder {
            fn import<S, T>(&mut self, module: S, name: T, index: $name) -> ImportIndex
                where S: Into<String>,
                      T: Into<String>
            {
            let entry = ImportEntry {
                module: module.into(),
                field: name.into(),
                kind: ImportKind::$variant(index),
            };
            self.add_import(entry)
        }
        }
    }
}

gen_import!(TypeIndex, Function);
gen_import!(TableType, Table);
gen_import!(MemoryType, Memory);
gen_import!(GlobalType, Global);


pub trait NewFunction<T> {
    fn new_function(&mut self, t: T, body: FunctionBody) -> FunctionIndex;
}

impl NewFunction<TypeIndex> for ModuleBuilder {
    fn new_function(&mut self, t: TypeIndex, body: FunctionBody) -> FunctionIndex {
        let fidx = self.add_function(Function(t));
        let cidx = self.add_code(body);
        assert_eq!(*cidx, *fidx);
        fidx
    }
}

impl NewFunction<FuncType> for ModuleBuilder {
    fn new_function(&mut self, t: FuncType, body: FunctionBody) -> FunctionIndex {
        self.new_function((t, body))
    }
}



pub struct CodeBuilder {
    code: Vec<Op>,
}

macro_rules! gen_builder {
    ($variant: path, $fname: ident) => {
        pub fn $fname(mut self) -> Self {
            self.code.push($variant);
            self
        }
    };
    ($variant: tt {$($arg: ident : $argty: ty, )* }, $fname: ident) => {
        pub fn $fname(mut self, $($arg: $argty, )*) -> Self {
            self.code.push($variant {
                $($arg : $arg, )*
            });
            self
        }
    };

    ($variant: tt { $arg: ident : $argty: ty $(, $args: ident : $argtys: ty)* }, $fname: ident) => {
        gen_builder!($variant {$arg: $argty $(, $args : $argtys)*, }, $fname);
    };

    ($variant: tt [$($arg: ident : $argty: ty, )*], $fname: ident) => {
        pub fn $fname(mut self, $($arg: $argty, )*) -> Self {
            self.code.push($variant($($arg, )*));
            self
        }
    };

    ($variant: tt [ $arg: ident : $argty: ty  $(, $args: ident : $argtys: ty)*], $fname: ident) => {
        gen_builder!($variant [$arg: $argty $(, $args : $argtys)*, ], $fname);
    };
}

macro_rules! gen_memory_builder {
    ($variant: tt, $fname: ident, $align: expr) => {
        pub fn $fname(mut self, offset: u32) -> Self {
            let imm = ops::MemoryImmediate{
                flags: $align,
                offset: offset
            };
            self.code.push($variant{imm: imm});
            self
        }
    };

}
use Op::*;

impl CodeBuilder {
    pub fn new() -> Self {
        CodeBuilder { code: Vec::new() }
    }

    pub fn build(mut self) -> Code {
        Code(self.code)
    }

    gen_builder!(Unreachable, unreachable);
    gen_builder!(Nop, nop);
    gen_builder!(Block { sig: BlockType }, block);
    gen_builder!(Loop { sig: BlockType }, loop_);
    gen_builder!(If, if_);
    gen_builder!(Else, else_);
    gen_builder!(End, end);
    gen_builder!(Br { depth: u32 }, br);
    gen_builder!(BrIf { depth: u32 }, br_if);
    pub fn br_table(mut self, table: Vec<u32>, default: u32) -> Self {
        self.code
            .push(BrTable(ops::BrTarget {
                              table: table,
                              default_target: default,
                          }));
        self
    }
    gen_builder!(Return, return_);

    gen_builder!(Call { index: u32 }, call);
    gen_builder!(CallIndirect {
                     index: u32,
                     reserved: bool,
                 },
                 call_indirect);

    gen_builder!(Drop, drop);
    gen_builder!(Select, select);

    gen_builder!(GetLocal[idx: LocalIndex], get_local);
    gen_builder!(SetLocal[idx: LocalIndex], set_local);
    gen_builder!(TeeLocal[idx: LocalIndex], tee_local);
    gen_builder!(GetGlobal[idx: GlobalIndex], get_global);
    gen_builder!(SetGlobal[idx: GlobalIndex], set_global);

    // TODO: generate with-flag API too.
    gen_memory_builder!(I32Load, i32_load, 5);
    gen_memory_builder!(I64Load, i64_load, 6);
    gen_memory_builder!(F32Load, f32_load, 5);
    gen_memory_builder!(F64Load, f64_load, 6);
    gen_memory_builder!(I32Load8S, i32_load8_s, 3);
    gen_memory_builder!(I32Load8U, i32_load8_u, 3);
    gen_memory_builder!(I32Load16S, i32_load16_s, 4);
    gen_memory_builder!(I32Load16U, i32_load16_u, 4);
    gen_memory_builder!(I64Load8S, i64_load8_s, 3);
    gen_memory_builder!(I64Load8U, i64_load8_u, 3);
    gen_memory_builder!(I64Load16S, i64_load16_s, 4);
    gen_memory_builder!(I64Load16U, i64_load16_u, 4);
    gen_memory_builder!(I64load32S, i64_load32_s, 5);
    gen_memory_builder!(I64load32U, i64_load32_u, 5);
    gen_memory_builder!(I32Store, i32_store, 5);
    gen_memory_builder!(I64Store, i64_store, 6);
    gen_memory_builder!(F32Store, f32_store, 5);
    gen_memory_builder!(F64Store, f64_store, 6);
    gen_memory_builder!(I32Store8, i32_store8, 3);
    gen_memory_builder!(I32Store16, i32_store16, 4);
    gen_memory_builder!(I64Store8, i64_store8, 3);
    gen_memory_builder!(I64Store16, i64_store16, 4);
    gen_memory_builder!(I64Store32, i64_store32, 5);
    gen_builder!(CurrentMemory { reserved: bool }, current_memory);
    gen_builder!(GrowMemory { reserved: bool }, grow_memory);


    pub fn constant<C>(mut self, c: C) -> Self
        where Op: From<C>
    {
        self.code.push(Op::from(c));
        self
    }

    gen_builder!(I32Eqz, i32_eqz);
    gen_builder!(I32Eq, i32_eq);
    gen_builder!(I32NE, i32_ne);
    gen_builder!(I32LtS, i32_lt_s);
    gen_builder!(I32LtU, i32_lt_u);
    gen_builder!(I32GtS, i32_gt_s);
    gen_builder!(I32GtU, i32_gt_u);
    gen_builder!(I32LeS, i32_le_s);
    gen_builder!(I32LeU, i32_le_u);
    gen_builder!(I32GeS, i32_ge_s);
    gen_builder!(I32GeU, i32_ge_u);
    gen_builder!(I64Eqz, i64_eqz);
    gen_builder!(I64Eq, i64_eq);
    gen_builder!(I64Ne, i64_ne);
    gen_builder!(I64LtS, i64_lt_s);
    gen_builder!(I64LtU, i64_lt_u);
    gen_builder!(I64GtS, i64_gt_s);
    gen_builder!(I64GtU, i64_gt_u);
    gen_builder!(I64LeS, i64_le_s);
    gen_builder!(I64LeU, i64_le_u);
    gen_builder!(I64GeS, i64_ge_s);
    gen_builder!(I64GeU, i64_ge_u);
    gen_builder!(F32Eq, f32_eq);
    gen_builder!(F32Ne, f32_ne);
    gen_builder!(F32Lt, f32_lt);
    gen_builder!(F32Gt, f32_gt);
    gen_builder!(F32Le, f32_le);
    gen_builder!(F32Ge, f32_ge);
    gen_builder!(F64Eq, f64_eq);
    gen_builder!(F64Ne, f64_ne);
    gen_builder!(F64Lt, f64_lt);
    gen_builder!(F64Gt, f64_gt);
    gen_builder!(F64Le, f64_le);
    gen_builder!(F64Ge, f64_ge);

    gen_builder!(I32Clz, i32_clz);
    gen_builder!(I32Ctz, i32_ctz);
    gen_builder!(I32Popcnt, i32_popcnt);
    gen_builder!(I32Add, i32_add);
    gen_builder!(I32Sub, i32_sub);
    gen_builder!(I32Mul, i32_mul);
    gen_builder!(I32DivS, i32_div_s);
    gen_builder!(I32DivU, i32_div_u);
    gen_builder!(I32RemS, i32_rem_s);
    gen_builder!(I32RemU, i32_rem_u);
    gen_builder!(I32And, i32_and);
    gen_builder!(I32Or, i32_or);
    gen_builder!(I32Xor, i32_xor);
    gen_builder!(I32Shl, i32_shl);
    gen_builder!(I32ShrS, i32_shr_s);
    gen_builder!(I32ShrU, i32_shr_u);
    gen_builder!(I32Rotl, i32_rotl);
    gen_builder!(I32Rotr, i32_rotr);
    gen_builder!(I64Clz, i64_clz);
    gen_builder!(I64Ctz, i64_ctz);
    gen_builder!(I64Popcnt, i64_popcnt);
    gen_builder!(I64Add, i64_add);
    gen_builder!(I64Sub, i64_sub);
    gen_builder!(I64Mul, i64_mul);
    gen_builder!(I64DivS, i64_div_s);
    gen_builder!(I64DivU, i64_div_u);
    gen_builder!(I64RemS, i64_rem_s);
    gen_builder!(I64RemU, i64_rem_u);
    gen_builder!(I64And, i64_and);
    gen_builder!(I64Or, i64_or);
    gen_builder!(I64Xor, i64_xor);
    gen_builder!(I64Shl, i64_shl);
    gen_builder!(I64ShrS, i64_shr_s);
    gen_builder!(I64ShrU, i64_shr_u);
    gen_builder!(I64Rotl, i64_rotl);
    gen_builder!(I64Rotr, i64_rotr);
    gen_builder!(F32Abs, f32_abs);
    gen_builder!(F32Neg, f32_neg);
    gen_builder!(F32Ceil, f32_ceil);
    gen_builder!(F32Floor, f32_floor);
    gen_builder!(F32Trunc, f32_trunc);
    gen_builder!(F32Nearest, f32_nearest);
    gen_builder!(F32Sqrt, f32_sqrt);
    gen_builder!(F32Add, f32_add);
    gen_builder!(F32Sub, f32_sub);
    gen_builder!(F32Mul, f32_mul);
    gen_builder!(F32Div, f32_div);
    gen_builder!(F32Min, f32_min);
    gen_builder!(F32Max, f32_max);
    gen_builder!(F32Copysign, f32_copysign);
    gen_builder!(F64Abs, f64_abs);
    gen_builder!(F64Neg, f64_neg);
    gen_builder!(F64Ceil, f64_ceil);
    gen_builder!(F64Floor, f64_floor);
    gen_builder!(F64Trunc, f64_trunc);
    gen_builder!(F64Nearest, f64_nearest);
    gen_builder!(F64Sqrt, f64_sqrt);
    gen_builder!(F64Add, f64_add);
    gen_builder!(F64Sub, f64_sub);
    gen_builder!(F64Mul, f64_mul);
    gen_builder!(F64Div, f64_div);
    gen_builder!(F64Min, f64_min);
    gen_builder!(F64Max, f64_max);
    gen_builder!(F64Copysign, f64_copysign);

    gen_builder!(I32wrapI64, i32_wrap_i64);
    gen_builder!(I32TruncSF32, i32_trunc_s_f32);
    gen_builder!(I32TruncUF32, i32_trunc_u_f32);
    gen_builder!(I32TruncSF64, i32_trunc_s_f64);
    gen_builder!(I32TruncUF64, i32_trunc_u_f64);
    gen_builder!(I64ExtendSI32, i64_extend_s_i32);
    gen_builder!(I64ExtendUI32, i64_extend_u_i32);
    gen_builder!(I64TruncSF32, i64_trunc_s_f32);
    gen_builder!(I64TruncUF32, i64_trunc_u_f32);
    gen_builder!(I64TruncSF64, i64_trunc_s_f64);
    gen_builder!(I64TruncUF64, i64_trunc_u_f64);
    gen_builder!(F32ConvertSI32, f32_convert_s_i32);
    gen_builder!(F32ConvertUI32, f32_convert_u_i32);
    gen_builder!(F32ConvertSI64, f32_convert_s_i64);
    gen_builder!(F32ConvertUI64, f32_convert_u_i64);
    gen_builder!(F32DemoteF64, f32_demote_f64);
    gen_builder!(F64ConvertSI32, f64_convert_s_i32);
    gen_builder!(F64ConvertUI32, f64_convert_u_i32);
    gen_builder!(F64ConvertSI64, f64_convert_s_i64);
    gen_builder!(F64ConvertUI64, f64_convert_u_i64);
    gen_builder!(F64PromoteF32, f64_promote_f32);

    gen_builder!(I32ReinterpretF32, i32_reinterpret_f32);
    gen_builder!(I64ReinterpretF64, i64_reinterpret_f64);
    gen_builder!(F32ReinterpretI32, f32_reinterpret_i32);
    gen_builder!(F64ReinterpretI64, f64_reinterpret_i64);
}

pub struct FunctionBuilder {
    ty: FuncType,
    args: Vec<LocalIndex>,
    locals: Vec<ValueType>,
    cb: CodeBuilder,
}


impl FunctionBuilder {
    pub fn new(ty: FuncType) -> Self {
        let args = (0..ty.params.len())
            .map(|i| LocalIndex::new(i as u32))
            .collect();
        let fb = FunctionBuilder {
            ty: ty,
            args: args,
            locals: Vec::new(),
            cb: CodeBuilder::new(),
        };
        fb
    }

    pub fn build(self) -> (FuncType, FunctionBody) {
        // TODO: compact local entry
        let locals = self.locals
            .into_iter()
            .map(|l| LocalEntry { count: 1, ty: l })
            .collect();
        let body = FunctionBody {
            locals: locals,
            code: self.cb.build(),
        };
        (self.ty, body)
    }

    pub fn new_local(&mut self, ty: ValueType) -> LocalIndex {
        self.locals.push(ty);
        LocalIndex::new((self.ty.params.len() + self.locals.len() - 1) as u32)
    }

    pub fn new_locals(&mut self, tys: Vec<ValueType>) -> Vec<LocalIndex> {
        tys.into_iter()
            .map(|ty| {
                     self.locals.push(ty);
                     LocalIndex::new((self.ty.params.len() + self.locals.len() - 1) as u32)
                 })
            .collect()
    }

    pub fn code<F: Fn(CodeBuilder, &[LocalIndex]) -> CodeBuilder>(mut self, f: F) -> Self {
        self.cb = f(self.cb, &self.args);
        self
    }
}

#[macro_export]
macro_rules! ty {
    (i32) => (ValueType::I32);
    (i64) => (ValueType::I64);
    (f32) => (ValueType::F32);
    (f64) => (ValueType::F64);
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
