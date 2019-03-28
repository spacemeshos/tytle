use crate::ast::expression::Expression;
use crate::ast::semantic::SymbolId;

#[derive(Debug, Clone, PartialEq)]
pub enum MakeStmtKind {
    Global,
    Local,
    Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MakeStmt {
    pub kind: MakeStmtKind,
    pub var_name: String,
    pub var_id: Option<SymbolId>,
    pub expr: Expression,
}
