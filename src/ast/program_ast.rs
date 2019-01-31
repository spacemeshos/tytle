use crate::ast::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramAst {
    pub statements: Vec<Statement>,
}

impl Default for ProgramAst {
    fn default() -> Self {
        Self {
            statements: Default::default(),
        }
    }
}
