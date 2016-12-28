#[macro_use]
extern crate WebAssembler;

use WebAssembler::*;
use WebAssembler::builder::*;

use std::io::Write;
use std::fs::File;

fn main() {
    let (args, fb) = FunctionBodyBuilder::new(funtype!((i32, i32) -> i32));
    let (ty, fbody) = fb.code(|cb|
            cb.constant(-3256)
            .get_local(args[0])
            .i32_add()
    ).build();


    let mut mb = ModuleBuilder::new();
    let f = mb.new_function(ty , fbody);
    mb.export("addTwo", f);

    let mut buf = Vec::new();
    let module = mb.build();
    module.dump(&mut buf);

    let mut out = File::create(std::env::args().nth(1).unwrap()).unwrap();
    let _ = out.write(&buf).unwrap();

}
