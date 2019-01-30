use crate::ast::expression::Expression;
use crate::ast::statement::block_stmt::BlockStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatStmt {
    pub count_expr: Expression,
    pub block: BlockStatement,
}
