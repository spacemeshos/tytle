use crate::ast::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct MakeStmt {
    pub var: String,
    pub expr: Expression,
}
