extern crate tytle;

use tytle::ast::semantic::*;
use tytle::parser::{Parser, TytleParser};

#[test]
fn ast_typecheck_error_assigning_global_int_var_a_boolean_value() {
    let code = r#"
            MAKEGLOBAL A = 10
            MAKE A = true
        "#;

    let mut ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();
    let mut sym_table = generator.generate(&mut ast).unwrap();
    let mut sym_visitor = SymbolTableVisitor::new(&mut sym_table);
    let mut checker = AstTypeCheck::new(&mut sym_visitor);

    let actual = checker.check(&mut ast);
    dbg!(actual);

    // assert_eq!(expected, actual);
}

#[test]
#[ignore]
fn ast_typecheck_error_assigning_local_int_var_a_string_value() {}

#[test]
#[ignore]
fn ast_typecheck_error_adding_int_and_string_expressions() {}

#[test]
#[ignore]
fn ast_typecheck_error_adding_int_and_proc_call_having_no_return_type() {}

#[test]
#[ignore]
fn ast_typecheck_error_adding_int_and_proc_call_having_str_return_type() {}

#[test]
#[ignore]
fn ast_typecheck_error_proc_call_type_mismatch() {}
