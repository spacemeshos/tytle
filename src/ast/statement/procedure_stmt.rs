use std::default::Default;
use std::fmt;

use crate::ast::statement::block_stmt::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureStmt {
    pub name: String,
    pub params: Vec<String>,
    pub block: BlockStatement,
}

impl ProcedureStmt {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Default::default(),
            block: BlockStatement::new(),
        }
    }
}
