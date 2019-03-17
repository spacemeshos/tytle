use crate::ast::expression::Expression;
use crate::ast::statement::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub cond_expr: Expression,
    pub true_block: BlockStatement,
    pub false_block: Option<BlockStatement>,
}
