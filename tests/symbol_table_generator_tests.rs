#[macro_use]
extern crate tytle;

use tytle::parser::{Parser, TytleParser};
use tytle::ast::semantic::{SymbolTableGenerator, AstWalkError};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_global_assign_before_declare() {
        let code = r#"
            MAKE "A=20
        "#;

        let expected = AstWalkError::MissingVarDeclaration("A".to_string());

        let ast = TytleParser.parse(code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        let actual = generator.generate(&ast).err().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_duplicate_global_variable_declaration() {
        let code = r#"
            MAKEGLOBAL "A=10
            MAKEGLOBAL "A=20
        "#;

        let expected = AstWalkError::DuplicateGlobalVar("A".to_string());

        let ast = TytleParser.parse(code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        let actual = generator.generate(&ast).err().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn error_duplicate_proc_declaration() {
        let code = r#"
            TO MYPROC
            END

            TO MYPROC
            END
        "#;

        let expected = AstWalkError::DuplicateProc("MYPROC".to_string());

        let ast = TytleParser.parse(code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        let actual = generator.generate(&ast).err().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn prewalk_make_and_procs() {
        let code = r#"
            MAKE "A=20

            TO MOVE_FORWARD
                FORWARD 10
            END

            TO MOVE_BACKWARD
                BACKWARD 10
            END

            MAKE "B=30
            MAKE "C=40

            "#;

        let ast = TytleParser.parse(code).unwrap();

        let mut generator = SymbolTableGenerator::new();
        generator.generate(&ast);
    }
}
