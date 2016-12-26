extern crate WebAssembler;

use WebAssembler::*;

use std::io::Write;
use std::fs::File;

fn main() {
    let mut code = Vec::new();

    Constant::I32Const(-3256).dump(&mut code);
    VariableAccess::GetLocal(1).dump(&mut code);
    Numeric::I32Add.dump(&mut code);

    let module = Module {
        sections: vec![
            Section::TYPE(vec![
                FuncType {
                    params: vec![ValueType::I32, ValueType::I32],
                    ret: Some(ValueType::I32)
                }
            ]),
            Section::FUNCTION(vec![Function(0)]),
            Section::EXPORT(vec![
                ExportEntry {
                    field: "addTwo".to_string(),
                    kind: ExternalKind::Function,
                    index: 0,
                }
            ]),
            Section::CODE(vec![
                FunctionBody {
                    locals: vec![],
                    code: code,
                }
            ])
        ]
    };

    let mut buf = Vec::new();
    module.dump(&mut buf);

    let mut out = File::create(std::env::args().nth(1).unwrap()).unwrap();
    let _ = out.write(&buf).unwrap();

}
