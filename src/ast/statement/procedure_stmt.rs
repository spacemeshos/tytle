use std::default::Default;
use std::fmt;

use crate::ast::statement::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct ProcParam {
    pub name: String,
    // TODO: add explicit procedure parameter type
    // for now we'll assume each procedure parameter if of type `INT`
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureStmt {
    pub name: String,
    pub params: Vec<ProcParam>,
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
