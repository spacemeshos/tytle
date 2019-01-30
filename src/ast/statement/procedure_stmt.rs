use std::default::Default;
use std::fmt;

use crate::ast::statement::block_stmt::BlockStatement;

#[derive(Clone, PartialEq)]
pub struct ProcedureStmt {
    pub name: String,
    pub block: BlockStatement,
}

impl ProcedureStmt {
    pub fn new(name: String) -> Self {
        Self {
            name,
            block: BlockStatement::new()
        }
    }
}

impl fmt::Debug for ProcedureStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let proc_str = r#"
        CODE:
        "#;

        write!(f, "{}", proc_str)
    }
}
