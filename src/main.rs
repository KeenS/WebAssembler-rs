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

    let module = Module {
        types: Some(vec![
            FuncType {
                params: vec![ValueType::I32, ValueType::I32],
                ret: Some(ValueType::I32)
            }
        ]),
        functions: Some(vec![Function(0)]),
        exports: Some(vec![
            ExportEntry {
                field: "addTwo".to_string(),
                kind: ExternalKind::Function,
                index: 0,
            }
        ]),
        codes: Some(vec![
            FunctionBody {
                locals: vec![],
                code: code,
            }
        ]),
        unknown: None,
        imports: None,
        tables: None,
        memories: None,
        globals: None,
        start: None,
        elements: None,
        data: None,
    };

    let mut buf = Vec::new();
    module.dump(&mut buf);

    let mut out = File::create(std::env::args().nth(1).unwrap()).unwrap();
    let _ = out.write(&buf).unwrap();

}
