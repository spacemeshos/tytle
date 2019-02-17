extern crate tytle;

use tytle::ast::semantic::*;
use tytle::parser::{Parser, TytleParser};

#[test]
fn sym_generate_error_global_use_before_declare() {
    let code = r#"
            MAKE A=20
        "#;

    let expected = AstWalkError::MissingVarDeclaration("A".to_string());

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();
    let actual = generator.generate(&ast).err().unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn sym_generate_error_local_use_before_declare() {
    let code = r#"
            TO MYPROC()
                MAKE A=20
            END
        "#;

    let expected = AstWalkError::MissingVarDeclaration("A".to_string());

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();
    let actual = generator.generate(&ast).err().unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn sym_generate_error_duplicate_global_variable_declaration() {
    let code = r#"
            MAKEGLOBAL A=10
            MAKEGLOBAL A=20
        "#;

    let expected = AstWalkError::DuplicateGlobalVar("A".to_string());

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();
    let actual = generator.generate(&ast).err().unwrap();

    assert_eq!(expected, actual);
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

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();
    let actual = generator.generate(&ast).err().unwrap();

    assert_eq!(expected, actual);
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

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();
    let actual = generator.generate(&ast).err().unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn sym_generate_error_proc_cannot_declare_global_variables() {
    let code = r#"
            TO MYPROC()
                MAKEGLOBAL A = 10
            END
        "#;

    let expected = AstWalkError::ProcNotAllowedToDeclareGlobals("A".to_string());

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();
    let actual = generator.generate(&ast).err().unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn sym_generate_error_duplicate_proc_param() {
    let code = r#"
            TO MYPROC(A: INT, A: STR)
            END
        "#;

    let expected = AstWalkError::DuplicateProcParam("MYPROC".to_string(), "A".to_string());

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();

    let actual = generator.generate(&ast).err().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn sym_generate_error_proc_param_is_considered_a_local_variable() {
    let code = r#"
            TO MYPROC(A: INT)
                MAKELOCAL A = 10
            END
        "#;

    let expected = AstWalkError::DuplicateProcLocalVar("A".to_string());

    let ast = TytleParser.parse(code).unwrap();

    let mut generator = SymbolTableGenerator::new();

    let actual = generator.generate(&ast).err().unwrap();

    assert_eq!(expected, actual);
}
