#![allow(dead_code)]
#![allow(unused)]
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
    let mut ast = TytleParser.parse(code).unwrap();

    let generator = SymbolTableGenerator::new();
    let mut env = generator.generate(&mut ast).unwrap();
    let mut type_checker = AstTypeCheck::new(&mut env);

    let res = type_checker.check(&mut ast);

    if res.is_err() {
        compilation_error(":(");
        return;
    }

    let builder = CfgBuilder::new(&mut env);
    let cfg = builder.build(&ast);

    let mut host = BrowserHost::new();
    let mut intr = Interpreter::new(&cfg, &env, &mut host);

    intr.exec_code();
}
