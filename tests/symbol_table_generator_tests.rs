extern crate tytle;

use tytle::ast::expression::ExpressionType;
use tytle::ast::semantic::*;
use tytle::parser::{Parser, TytleParser};

macro_rules! assert_symbol_err {
    ($expected:expr, $code:expr) => {{
        let mut ast = TytleParser.parse($code).unwrap();
        let mut generator = SymbolTableGenerator::new();

        let res = generator.generate(&mut ast);
        assert!(res.is_err());

        let actual = res.err().unwrap();

        assert_eq!($expected, actual);
    }};
}

macro_rules! gen_symbols {
    ($code:expr, $sym_table_var: ident) => {
        let mut ast = TytleParser.parse($code).unwrap();
        let mut generator = SymbolTableGenerator::new();

        let res = generator.generate(&mut ast);
        assert!(res.is_ok());

        let mut $sym_table_var = res.unwrap().clone();
    };
}

#[test]
fn sym_generate_global_var() {
    let code = r#"
            MAKEGLOBAL A=20
        "#;

    gen_symbols!(code, sym_table);

    let visitor = SymbolTableVisitor::new(&mut sym_table);

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, true);
    assert_eq!(var.name, "A".to_string());
}

#[test]
fn sym_generate_proc_param() {
    let code = r#"
            TO MYPROC(A: INT)
            END
        "#;

    gen_symbols!(code, sym_table);

    let mut visitor = SymbolTableVisitor::new(&mut sym_table);
    visitor.next_scope(); // entering the `MYPROC` scope

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, false);
    assert_eq!(var.name, "A".to_string());
    assert_eq!(var.var_type, Some(ExpressionType::Int))
}

#[test]
fn sym_generate_proc_local_var() {
    let code = r#"
            TO MYPROC()
                MAKELOCAL A = 10
            END
        "#;

    gen_symbols!(code, sym_table);

    let mut visitor = SymbolTableVisitor::new(&mut sym_table);
    visitor.next_scope(); // entering the `MYPROC` scope

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, false);
    assert_eq!(var.name, "A".to_string());
    assert_eq!(var.var_type, None);
}

#[test]
fn sym_generate_proc_if_stmt_local_var() {
    let code = r#"
            TO MYPROC()
                IF 1 + 2 [MAKELOCAL A = 1]
            END
        "#;

    gen_symbols!(code, sym_table);

    let mut visitor = SymbolTableVisitor::new(&mut sym_table);
    visitor.next_scope(); // entering the `MYPROC` scope
    visitor.next_scope(); // entering the `if statement` scope

    let symbol = visitor.lookup("A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, false);
    assert_eq!(var.name, "A".to_string());
    assert_eq!(var.var_type, None);
}

#[test]
fn sym_generate_error_global_use_before_declare() {
    let code = r#"
            MAKE A=20
        "#;

    let expected = AstWalkError::MissingVarDeclaration("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_local_use_before_declare() {
    let code = r#"
            TO MYPROC()
                MAKE A=20
            END
        "#;

    let expected = AstWalkError::MissingVarDeclaration("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_duplicate_global_variable_declaration() {
    let code = r#"
            MAKEGLOBAL A=10
            MAKEGLOBAL A=20
        "#;

    let expected = AstWalkError::DuplicateGlobalVar("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_duplicate_local_variable_declaration() {
    let code = r#"
        TO MYPROC()
            MAKELOCAL A=10
            MAKELOCAL A=20
        END
        "#;

    let expected = AstWalkError::DuplicateProcLocalVar("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_duplicate_proc_declaration() {
    let code = r#"
            TO MYPROC()
            END

            TO MYPROC()
            END
        "#;

    let expected = AstWalkError::DuplicateProc("MYPROC".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_proc_cannot_declare_global_variables() {
    let code = r#"
            TO MYPROC()
                MAKEGLOBAL A = 10
            END
        "#;

    let expected = AstWalkError::ProcNotAllowedToDeclareGlobals("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_duplicate_proc_param() {
    let code = r#"
            TO MYPROC(A: INT, A: STR)
            END
        "#;

    let expected = AstWalkError::DuplicateProcParam("MYPROC".to_string(), "A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_proc_param_is_considered_a_local_variable() {
    let code = r#"
            TO MYPROC(A: INT)
                MAKELOCAL A = 10
            END
        "#;

    let expected = AstWalkError::DuplicateProcLocalVar("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_locals_not_allowed_under_root_scope() {
    let code = r#"
            MAKELOCAL A = 10
        "#;

    let expected = AstWalkError::LocalsNotAllowedUnderRootScope("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_lookup_global_var_from_within_an_inner_scope() {
    let code = r#"
            MAKEGLOBAL A = 10
            TO MYPROC()
                IF 1 + 2 [MAKELOCAL B = 1]
            END
        "#;

    gen_symbols!(code, sym_table);

    let mut visitor = SymbolTableVisitor::new(&mut sym_table);
    visitor.next_scope(); // entering the `MYPROC` scope
    visitor.next_scope(); // entering the `if statement` scope

    let symbol = visitor.lookup_recur("A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, true);
    assert_eq!(var.name, "A".to_string());
}
