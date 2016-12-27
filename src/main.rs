extern crate WebAssembler;

use WebAssembler::*;

use std::io::Write;
use std::fs::File;

fn main() {
    let mut code = Vec::new();

    Constant::I32Const(-3256).dump(&mut code);
    VariableAccess::GetLocal(1).dump(&mut code);
    Numeric::I32Add.dump(&mut code);
    code.push(0x0b);

    let mut mb = builder::ModuleBuilder::new();
    let fty = mb.add_type(FuncType {
                params: vec![ValueType::I32, ValueType::I32],
                ret: Some(ValueType::I32)
            });

    mb.add_function(Function(fty));
    let f = mb.add_code(FunctionBody {
            locals: vec![],
            code: code,
        });
    mb.add_export(ExportEntry {
        field: "addTwo".to_string(),
        kind: ExternalKind::Function,
        index: f,
    });

    let mut buf = Vec::new();
    let module = mb.build();
    module.dump(&mut buf);

    let mut out = File::create(std::env::args().nth(1).unwrap()).unwrap();
    let _ = out.write(&buf).unwrap();

}
