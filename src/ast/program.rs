use crate::ast::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            statements: Default::default(),
        }
    }
}
