use crate::ast::statement::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    stmts: Vec<Statement>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self {
            stmts: Default::default(),
        }
    }

    pub fn add_statement(&mut self, stmt: Statement) {
        if stmt != Statement::Nop {
            self.stmts.push(stmt);
        }
    }
}
