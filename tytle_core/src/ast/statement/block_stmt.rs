use crate::ast::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub stmts: Vec<Statement>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self {
            stmts: Default::default(),
        }
    }

    pub fn add_statement(&mut self, stmt: Statement) {
        if stmt != Statement::NOP {
            self.stmts.push(stmt);
        }
    }
}
