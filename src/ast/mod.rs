pub mod expression;
pub mod semantic;
pub mod statement;

use crate::ast::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Default for Ast {
    fn default() -> Self {
        Self {
            statements: Default::default(),
        }
    }
}
