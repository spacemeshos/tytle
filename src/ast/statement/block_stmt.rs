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
        if stmt != Statement::Nop {
            self.stmts.push(stmt);
        }
    }
}

// impl<'a> IntoIterator for &'a BlockStatement {
//     type Item = &'a Statement;
//     type IntoIter = std::vec::IntoIter<Statement>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.stmts.into_iter()
//     }
// }
