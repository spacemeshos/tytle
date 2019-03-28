extern crate tytle;

use tytle::prelude::*;
use wasm_bindgen::prelude::*;

mod host;
use host::BrowserHost;

#[wasm_bindgen]
extern "C" {
    fn compilation_error(error: &str);
}

#[wasm_bindgen]
pub fn execute(code: &str) {
    let mut host = BrowserHost::new();
    let parse_res = TytleParser.parse(code);

    if let Err(err) = parse_res {
        host.compilation_error(&err.to_string());
        return;
    }

    let mut ast = parse_res.unwrap();
    let generator = SymbolTableGenerator::new();
    let mut env = generator.generate(&mut ast).unwrap();
    let mut type_checker = AstTypeCheck::new(&mut env);

    let type_res = type_checker.check(&mut ast);
    if let Err(err) = type_res {
        host.compilation_error(&err.to_string());
        return;
    }

    let cfg_builder = CfgBuilder::new(&mut env);
    let cfg = cfg_builder.build(&ast);

    let mut intr = Interpreter::new(&cfg, &env, &mut host);

    let _ = intr.exec_code();
}
