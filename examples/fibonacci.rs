extern crate web_assembler as wasm;

use std::env;
use std::fs::File;
use std::io::Write;
use wasm::builder::*;
use wasm::*;

fn main() {
    let out_file = env::args().nth(1).expect("argument missing: output file");

    let mut md = ModuleBuilder::new();
    // function to create must be the 0th function of the module...
    let fib = FunctionIndex(0).into();
    let f = FunctionBuilder::new(funtype!((i32) -> i32))
        .code(|cb, params| {
            let n = params[0];
            cb.get_local(n)
                .constant(1i32)
                .i32_sub()
                .call(fib)
                .get_local(n)
                .constant(2i32)
                .i32_sub()
                .call(fib)
                .i32_add()
                .return_()
        })
        .build();
    md.new_function(f);

    let module = md.build();
    let mut code = Vec::new();
    module.dump(&mut code);
    let mut out = File::create(out_file).unwrap();
    out.write(&code).unwrap();
}
