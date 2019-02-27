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

macro_rules! do_typecheck {
    ($code:expr, $sym_table_var: ident) => {
        let mut ast = TytleParser.parse($code).unwrap();

        let mut sym_generator = SymbolTableGenerator::new();
        let mut $sym_table_var = sym_generator.generate(&mut ast).unwrap();
        let mut sym_visitor = SymbolTableVisitor::new(&mut $sym_table_var);
        let mut type_checker = AstTypeCheck::new(&mut sym_visitor);

        let actual = type_checker.check(&mut ast);

        assert_eq!(Ok(()), actual);
    };
}

#[test]
fn ast_typecheck_var_assign_bool_literal() {
    let code = r#"
            MAKEGLOBAL A = TRUE
            MAKEGLOBAL B = FALSE
        "#;

    do_typecheck!(code, sym_table);

    let visitor = SymbolTableVisitor::new(&mut sym_table);

    // variable A
    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var_a = symbol.unwrap().as_var();
    assert_eq!(var_a.var_type, Some(ExpressionType::Bool));

    // variable B
    let symbol = visitor.lookup("B", &SymbolKind::Var);
    let var_b = symbol.unwrap().as_var();
    assert_eq!(var_b.var_type, Some(ExpressionType::Bool));
}

#[test]
fn ast_typecheck_var_assign_cmp_expr() {
    let code = r#"
            MAKEGLOBAL A = 1 < 2
        "#;

    do_typecheck!(code, sym_table);

    let visitor = SymbolTableVisitor::new(&mut sym_table);

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var_a = symbol.unwrap().as_var();
    assert_eq!(var_a.var_type, Some(ExpressionType::Bool));
}

#[test]
fn ast_typecheck_var_assign_not_expr() {
    let code = r#"
            MAKEGLOBAL A = NOT FALSE
        "#;

    do_typecheck!(code, sym_table);

    let visitor = SymbolTableVisitor::new(&mut sym_table);

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var_a = symbol.unwrap().as_var();
    assert_eq!(var_a.var_type, Some(ExpressionType::Bool));
}

#[test]
fn ast_typecheck_var_assign_int_literal() {
    let code = r#"
            MAKEGLOBAL A = 10
        "#;

    do_typecheck!(code, sym_table);

    let visitor = SymbolTableVisitor::new(&mut sym_table);

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var_a = symbol.unwrap().as_var();
    assert_eq!(var_a.var_type, Some(ExpressionType::Int));
}

#[test]
fn ast_typecheck_var_assign_int_expr() {
    let code = r#"
            MAKEGLOBAL A = (1 + 2) * (3 + 4)
        "#;

    do_typecheck!(code, sym_table);

    let visitor = SymbolTableVisitor::new(&mut sym_table);

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var_a = symbol.unwrap().as_var();
    assert_eq!(var_a.var_type, Some(ExpressionType::Int));
}

#[test]
fn ast_typecheck_var_assign_str_literal() {
    let code = r#"
            MAKEGLOBAL A = "Hello"
        "#;

    do_typecheck!(code, sym_table);

    let visitor = SymbolTableVisitor::new(&mut sym_table);

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var_a = symbol.unwrap().as_var();
    assert_eq!(var_a.var_type, Some(ExpressionType::Str));
}

#[test]
fn ast_typecheck_error_cannot_add_strings() {
    let code = r#"
            MAKEGLOBAL A = "Hello" + "World"
        "#;

    let expected =
        AstWalkError::InvalidBinaryOp(BinaryOp::Add, ExpressionType::Str, ExpressionType::Str);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_cannot_add_bools() {
    let code = r#"
            MAKEGLOBAL A = TRUE + FALSE
        "#;

    let expected =
        AstWalkError::InvalidBinaryOp(BinaryOp::Add, ExpressionType::Bool, ExpressionType::Bool);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_cannot_order_bools() {
    let code = r#"
            MAKEGLOBAL A = TRUE > FALSE
        "#;

    let expected =
        AstWalkError::InvalidBinaryOp(BinaryOp::GT, ExpressionType::Bool, ExpressionType::Bool);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_cannot_order_strings() {
    let code = r#"
            MAKEGLOBAL A = "Hello" < "World"
        "#;

    let expected =
        AstWalkError::InvalidBinaryOp(BinaryOp::LT, ExpressionType::Str, ExpressionType::Str);

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_cannot_negate_int() {
    let code = r#"
            MAKEGLOBAL A = NOT(1 + 2)
        "#;

    let expected = AstWalkError::NotBooleanExpr("(1 + 2)".to_string());

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_cannot_negate_string() {
    let code = r#"
            MAKEGLOBAL A = NOT "Hello"
        "#;

    let expected = AstWalkError::NotBooleanExpr("\"Hello\"".to_string());

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_declaring_a_local_var_with_proc_call_returning_unit() {
    let code = r#"
            TO MYPROC()
            END

            TO MYPROC_2(Y: STR)
                MAKELOCAL A = MYPROC()
            END
        "#;

    let expected = AstWalkError::VariableTypeMissing("A".to_string());

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_if_stmt_expr_must_be_bool() {
    let code = r#"
            MAKEGLOBAL A = 10
            IF 1 + 2 [MAKE A = 20]
        "#;

    let expected = AstWalkError::NotBooleanExpr("1 + 2".to_string());

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_repeat_count_expr_must_be_int() {
    let code = r#"
            MAKEGLOBAL A = 10

            REPEAT 1 < 2 [
                MAKE A = 20
            ]
        "#;

    let expected = AstWalkError::NotIntExpr("1 < 2".to_string());

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_if_stmt_block_with_assigning_var_expr_with_wrong_type() {
    let code = r#"
            TO MYPROC()
                REPEAT 3 [
                    IF 1 < 2 [
                        MAKELOCAL A = TRUE
                        IF 3 < 4 [MAKE A = 1]
                    ]
                ]
            END
        "#;

    let expected = AstWalkError::TypeMismatch(ExpressionType::Bool, ExpressionType::Int);

    assert_type_err!(expected, code);
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

    let expected = AstWalkError::VariableTypeMissing("A".to_string());

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_direct_stmt_expr_must_be_an_integer() {
    let code = r#"
            FORWARD 1 < 2
        "#;

    let expected = AstWalkError::NotIntExpr("1 < 2".to_string());

    assert_type_err!(expected, code);
}

#[test]
fn ast_typecheck_error_wrong_return_type() {
    let code = r#"
            TO MYPROC(): INT
                RETURN TRUE
            END
        "#;

    let expected = AstWalkError::NotIntExpr("TRUE".to_string());

    assert_type_err!(expected, code);
}

#[test]
#[ignore]
fn ast_typecheck_error_cannot_use_return_for_proc_returning_unit() {
    let code = r#"
            TO MYPROC()
                RETURN 10
            END
        "#;

    let expected = AstWalkError::NotIntExpr("TRUE".to_string());

    assert_type_err!(expected, code);
}
