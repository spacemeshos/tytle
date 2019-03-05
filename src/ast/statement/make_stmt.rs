use crate::ast::expression::Expression;

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
    pub var_id: Option<u64>,
    pub expr: Expression,
}
