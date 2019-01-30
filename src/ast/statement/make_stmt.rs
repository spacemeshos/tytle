use crate::ast::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct MakeStmt {
    pub symbol: String,
    pub expr: Expression,
}
