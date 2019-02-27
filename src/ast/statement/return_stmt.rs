use crate::ast::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expr: Option<Expression>,
}

impl ReturnStmt {
    pub fn new(expr: Option<Expression>) -> Self {
        Self { expr }
    }
}
