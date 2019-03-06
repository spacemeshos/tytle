extern crate tytle;

use tytle::ast::semantic::{AstTypeCheck, SymbolTableGenerator};
use tytle::ir::CfgBuilder;
use tytle::parser::{Parser, TytleParser};

macro_rules! prepare_ast {
    ($code:expr) => {{
        let mut ast = TytleParser.parse($code).unwrap();
        let mut generator = SymbolTableGenerator::new();
        let mut symbol_table = generator.generate(&mut ast).unwrap();
        let mut checker = AstTypeCheck::new(&mut symbol_table);

        let _ = checker.check(&mut ast);
        ast
    }};
}

#[test]
fn cfg_build_translate_sanity() {
    let code = r#"
        PENUP
        FORWARD 20
    "#;

    let ast = prepare_ast!(code);
    let builder = CfgBuilder::new();
    // let graph = builder.build(&ast);
    // dbg!(graph);
}
