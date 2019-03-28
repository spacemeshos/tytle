use crate::ast::semantic::SymbolId;
use crate::ast::statement::BlockStatement;
use std::default::Default;

#[derive(Debug, Clone, PartialEq)]
pub struct ProcParam {
    pub param_name: String,
    pub param_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureStmt {
    pub id: Option<SymbolId>,
    pub name: String,
    pub params: Vec<ProcParam>,
    pub return_type: String,
    pub block: BlockStatement,
}

impl ProcedureStmt {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: None,
            params: Default::default(),
            return_type: "".to_string(),
            block: BlockStatement::new(),
        }
    }
}
