use crate::ast::statement::BlockStatement;
use crate::ast::variable::Variable;

use std::collections::HashSet;

pub struct Procedure {
    pub name: String,
    pub locals: HashSet<Variable>,
    pub params: HashSet<Variable>,
    pub block: BlockStatement,
}

impl Procedure {
    pub fn new(name: String) -> Self {
        Self {
            name,
            locals: Default::default(),
            params: Default::default(),
            block: Default::default(),
        }
    }
}
