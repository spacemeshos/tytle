extern crate tytle;

use tytle::ast::semantic::*;
use tytle::ir::*;
use tytle::parser::{Parser, TytleParser};
use tytle::vm::*;

#[test]
pub fn interpreter_forward() {
    let code = "FORWARD 10";
    let mut ast = TytleParser.parse(code).unwrap();
    let generator = SymbolTableGenerator::new();

    let mut env = generator.generate(&mut ast).unwrap();
    let mut checker = AstTypeCheck::new(&mut env);

    let res = checker.check(&mut ast);
    assert!(res.is_ok());

    let builder = CfgBuilder::new(&mut env);
    let cfg = builder.build(&ast);

    let mut host = DummyHost::new();
    let mut intr = Interpreter::new(&cfg, &env, &mut host);

    intr.exec_next();
    intr.exec_next();

    assert_eq!((0, 10), host.xycors());
}
