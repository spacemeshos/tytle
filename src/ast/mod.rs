pub mod expression;
pub mod macros;
pub mod semantic;
pub mod statement;

use crate::ast::statement::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn as_program_stmt(stmts: Vec<Statement>) -> ProcedureStmt {
        let mut root = ProcedureStmt::new("__main__".to_string());

        for stmt in stmts {
            root.block.add_statement(stmt);
        }

        root
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self {
            statements: Default::default(),
        }
    }
}
