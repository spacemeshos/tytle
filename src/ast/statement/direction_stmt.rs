use crate::ast::expression::Expression;
use crate::ast::statement::Direction;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionStmt {
    pub direction: Direction,
    pub expr: Expression,
}
