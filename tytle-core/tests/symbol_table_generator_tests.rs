extern crate tytle;

#[macro_use]
extern crate maplit;

use tytle::ast::expression::*;
use tytle::ast::semantic::*;
use tytle::ast::statement::*;
use tytle::parser::{Parser, TytleParser};

macro_rules! assert_symbol_err {
    ($expected:expr, $code:expr) => {{
        let mut ast = TytleParser.parse($code).unwrap();
        let generator = SymbolTableGenerator::new();

        let res = generator.generate(&mut ast);
        assert!(res.is_err());

        let actual = res.err().unwrap();

        assert_eq!($expected, actual);
    }};
}

macro_rules! gen_symbols {
    ($code: expr, $env: ident) => {
        gen_symbols!($code, $env, _ast__)
    };

    ($code:expr, $env: ident, $ast: ident) => {
        let mut $ast = TytleParser.parse($code).unwrap();
        let generator = SymbolTableGenerator::new();

        let res = generator.generate(&mut $ast);
        assert!(res.is_ok());

        let $env = res.unwrap();
    };
}

#[test]
fn sym_generate_global_var_int() {
    let code = r#"
            MAKEGLOBAL A = 20
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(0, "A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, true);
    assert_eq!(var.name, "A".to_string());

    assert_eq!(hashmap! { 0 => var.id }, env.globals_symbols);
}

#[test]
fn sym_generate_ast_records_var_global_index() {
    let code = r#"
            MAKEGLOBAL A = 10
            MAKEGLOBAL B = A
        "#;

    gen_symbols!(code, env, actual_ast);

    let symbol = env.symbol_table.lookup(0, "B", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, true);
    assert_eq!(var.name, "B".to_string());
    assert_eq!(var.id, 2);

    let lit_expr = LiteralExpr::Var("A".to_string(), Some(1));
    let expr_ast = ExpressionAst::Literal(lit_expr);

    let make_stmt = MakeStmt {
        kind: MakeStmtKind::Global,
        var_name: "B".to_string(),
        var_id: Some(2),
        expr: Expression {
            expr_ast,
            expr_type: None,
        },
    };

    let expected = Statement::Make(make_stmt);

    assert_eq!(expected, actual_ast.statements[1]);

    assert_eq!(
        hashmap! { 0 => var.id - 1, 1 => var.id },
        env.globals_symbols
    );
}

#[test]
fn sym_generate_global_var_bool() {
    let code = r#"
            MAKEGLOBAL A = TRUE
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(0, "A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, true);
    assert_eq!(var.name, "A".to_string());
}

#[test]
fn sym_generate_proc_param_int() {
    let code = r#"
            TO MYPROC(A: INT): BOOL
            END
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(1, "A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, false);
    assert_eq!(var.name, "A".to_string());
    assert_eq!(var.var_type, Some(ExpressionType::Int));

    let symbol = env.symbol_table.lookup(0, "MYPROC", &SymbolKind::Proc);
    let proc = symbol.unwrap().as_proc();

    assert_eq!(hashmap! { proc.id => vec![var.id] }, env.locals_symbols);
}

#[test]
fn sym_generate_proc_params() {
    let code = r#"
            TO MYPROC(A: INT, B: STR, C: BOOL): BOOL
            END
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(0, "MYPROC", &SymbolKind::Proc);
    let proc = symbol.unwrap().as_proc();

    let expected_params = vec![
        ExpressionType::Int,
        ExpressionType::Str,
        ExpressionType::Bool,
    ];

    assert_eq!(expected_params, proc.params_types);

    let symbol = env.symbol_table.lookup(1, "A", &SymbolKind::Var);
    let var_a = symbol.unwrap().as_var();

    assert_eq!(hashmap! { proc.id => vec![var_a.id, var_a.id + 1, var_a.id + 2] }, env.locals_symbols);
}

#[test]
fn sym_generate_proc_return_type() {
    let code = r#"
            TO MYPROC_INT(): INT
            END

            TO MYPROC_BOOL(): BOOL
            END

            TO MYPROC_STR(): STR
            END
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(0, "MYPROC_INT", &SymbolKind::Proc);
    let proc_int = symbol.unwrap().as_proc();

    assert_eq!(proc_int.name, "MYPROC_INT");
    assert_eq!(proc_int.return_type, ExpressionType::Int);
}

#[test]
fn sym_generate_proc_local_var_int() {
    let code = r#"
            TO MYPROC()
                MAKELOCAL A = 10
            END
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(1, "A", &SymbolKind::Var);
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

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(2, "A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, false);
    assert_eq!(var.name, "A".to_string());
    assert_eq!(var.var_type, None);
}

#[test]
fn sym_generate_initializing_a_local_var_with_proc_call_expr() {
    let code = r#"
            TO MYPROC(X: BOOL): INT
            END

            TO MYPROC_2(Y: STR)
                MAKELOCAL A = MYPROC()
            END
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup(2, "A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, false);
    assert_eq!(var.name, "A".to_string());
    assert_eq!(var.var_type, None);
}

#[test]
fn sym_generate_proc_call_inject_proc_id() {
    let code = r#"
            TO FOO(A: INT)
            END

            FOO(10)
        "#;

    gen_symbols!(code, env, ast);

    let symbol = env.symbol_table.lookup(0, "FOO", &SymbolKind::Proc);
    let proc = symbol.unwrap().as_proc();

    let proc_call_stmt = &ast.statements[1];

    let mut called = false;

    if let Statement::Expression(proc_call_expr) = proc_call_stmt {
        let (_, _, proc_id_option) = proc_call_expr.as_proc_call_expr();

        assert_eq!(proc.id, proc_id_option.unwrap());

        called = true;
    }

    assert!(called);
}

#[test]
fn sym_generate_error_global_use_before_declare() {
    let code = r#"
            MAKE A = 20
        "#;

    let expected = AstWalkError::MissingVarDeclaration("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_local_use_before_declare() {
    let code = r#"
            TO MYPROC()
                MAKE A = 20
            END
        "#;

    let expected = AstWalkError::MissingVarDeclaration("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_duplicate_global_variable_declaration() {
    let code = r#"
            MAKEGLOBAL A = 10
            MAKEGLOBAL A = 20
        "#;

    let expected = AstWalkError::DuplicateGlobalVar("A".to_string());

    assert_symbol_err!(expected, code);
}

#[test]
fn sym_generate_error_duplicate_local_variable_declaration() {
    let code = r#"
        TO MYPROC()
            MAKELOCAL A = 10
            MAKELOCAL A = 20
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
fn sym_generate_lookup_global_var_from_within_an_inner_scope() {
    let code = r#"
            MAKEGLOBAL A = 10
            TO MYPROC()
                IF 1 + 2 [MAKELOCAL B = 1]
            END
        "#;

    gen_symbols!(code, env);

    let symbol = env.symbol_table.lookup_recur(2, "A", &SymbolKind::Var);
    let var = symbol.unwrap().as_var();

    assert_eq!(var.global, true);
    assert_eq!(var.name, "A".to_string());
}

#[test]
fn sym_generate_error_locals_not_allowed_under_root_scope() {
    let code = r#"
            MAKELOCAL A = 10
        "#;

    let expected = AstWalkError::LocalsNotAllowedUnderRootScope("A".to_string());

    assert_symbol_err!(expected, code);
}
