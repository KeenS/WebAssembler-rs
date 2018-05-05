#[macro_use]
extern crate web_assembler as wasm;

use wasm::builder::*;
use wasm::*;
use std::fs::File;
use std::io::Write;
use std::env;

fn main() {
    let out_file = env::args().nth(1).expect("argument missing: output file");

    let mut md = ModuleBuilder::new();
    let f = FunctionBuilder::new(funtype!((i32, i32) -> i32))
        .code(|cb, params| {
            let a = params[0];
            let b = params[1];
            cb.get_local(a).get_local(b).i32_add().return_()
        })
        .build();
    md.new_function(f);

    let module = md.build();
    let mut code = Vec::new();
    module.dump(&mut code);
    let mut out = File::create(out_file).unwrap();
    out.write(&code).unwrap();
}
