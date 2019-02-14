use std::default::Default;
use std::fmt;

use crate::ast::statement::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct ProcParam {
    pub param_name: String,
    pub param_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureStmt {
    pub name: String,
    pub params: Vec<ProcParam>,
    pub return_type: Option<String>,
    pub block: BlockStatement,
}

impl ProcedureStmt {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Default::default(),
            return_type: None,
            block: BlockStatement::new(),
        }
    }
}
