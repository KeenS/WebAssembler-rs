#[macro_use]
extern crate web_assembler as wasm;

use wasm::*;
use wasm::builder::*;

use std::io::Write;
use std::fs::File;

fn main() {
    let mut mb = ModuleBuilder::new();
    let f = mb.new_function(FunctionBuilder::new(funtype!((i32, i32) -> i32))
                                .code(|cb, args| {
                                          cb.constant(-3256).get_local(args[0]).i32_store(4)
                                      })
                                .build());
    mb.export("addTwo", f);
    let module = mb.build();

    let mut buf = Vec::new();
    module.dump(&mut buf);

    let mut out = File::create(std::env::args().nth(1).unwrap()).unwrap();
    let _ = out.write(&buf).unwrap();

}
