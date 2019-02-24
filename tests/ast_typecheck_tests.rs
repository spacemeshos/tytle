extern crate tytle;

use tytle::ast::expression::*;
use tytle::ast::semantic::*;
use tytle::parser::{Parser, TytleParser};

macro_rules! assert_type_err {
    ($expected:expr, $code:expr) => {{
        let mut ast = TytleParser.parse($code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        let mut sym_table = generator.generate(&mut ast).unwrap();
        let mut sym_visitor = SymbolTableVisitor::new(&mut sym_table);
        let mut checker = AstTypeCheck::new(&mut sym_visitor);

        let actual = checker.check(&mut ast).err().unwrap();

        assert_eq!($expected, actual);
    }};
}

#[test]
fn ast_typecheck_error_assigning_global_int_var_a_boolean_value() {
    let code = r#"
            MAKEGLOBAL A = 10
            MAKE A = TRUE
        "#;

    let expected = AstWalkError::TypeMismatch(ExpressionType::Int, ExpressionType::Bool);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_assigning_global_int_var_a_string_value() {
    let code = r#"
            MAKEGLOBAL A = 10
            MAKE A = "Hello"
        "#;

    let expected = AstWalkError::TypeMismatch(ExpressionType::Int, ExpressionType::Str);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_assigning_global_int_var_and_than_proc_call_result_which_returns_a_string() {
    let code = r#"
            TO MYPROC(): BOOL
            END

            MAKEGLOBAL A = 10
            MAKE A = MYPROC()
        "#;

    let expected = AstWalkError::TypeMismatch(ExpressionType::Int, ExpressionType::Bool);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_adding_int_and_string_expressions() {
    let code = r#"
            MAKEGLOBAL A = "Hello"
            MAKEGLOBAL B = A + 10
        "#;

    let expected =
        AstWalkError::InvalidBinaryOp(BinaryOp::Add, ExpressionType::Str, ExpressionType::Int);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_adding_int_and_proc_call_having_str_return_type() {
    let code = r#"
            TO MYPROC(): STR
            END

            MAKEGLOBAL B = 10 + MYPROC()
        "#;

    let expected =
        AstWalkError::InvalidBinaryOp(BinaryOp::Add, ExpressionType::Int, ExpressionType::Str);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_proc_call_wrong_args_count() {
    let code = r#"
            TO MYPROC(A: INT): BOOL
            END

            MAKEGLOBAL B = MYPROC(1, 2)
        "#;

    let expected = AstWalkError::InvalidProcCallArgsCount("MYPROC".to_string(), 1, 2);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_proc_call_args_type_mismatch() {
    let code = r#"
            TO MYPROC(A: INT): BOOL
            END

            MAKEGLOBAL B = "Hello"
            MAKEGLOBAL C = MYPROC(B)
        "#;

    let expected =
        AstWalkError::InvalidProcCallArgType(1, ExpressionType::Int, ExpressionType::Str);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_adding_int_and_proc_call_having_no_return_type() {
    let code = r#"
            TO MYPROC()
            END

            MAKEGLOBAL B = 10 + MYPROC()
        "#;

    let expected =
        AstWalkError::InvalidBinaryOp(BinaryOp::Add, ExpressionType::Int, ExpressionType::Unit);

    assert_type_err!(expected, code);
}


#[test]
fn ast_typecheck_error_variable_declaration_type_must_not_be_unit() {
    let code = r#"
            TO MYPROC()
            END

            MAKEGLOBAL A = MYPROC()
        "#;

    let expected =
        AstWalkError::VariableTypeMissing("A".to_string());

    assert_type_err!(expected, code);
}
