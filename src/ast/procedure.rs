use crate::ast::statement::BlockStatement;
use crate::ast::variable::Variable;

use std::collections::HashSet;
use std::fmt;

pub struct Procedure {
    pub name: String,
    pub locals: HashSet<Variable>,
    pub params: HashSet<Variable>,
    pub code: BlockStatement,
}

impl Procedure {
    pub fn new(name: String) -> Self {
        Self {
            name,
            locals: Default::default(),
            params: Default::default(),
            code: Default::default(),
        }
    }
}

impl fmt::Debug for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let proc_str = r#"
        PROC: []
        PARAMS: []
        LOCALS: []
        CODE:
        "#;

        write!(f, "{}", proc_str)
    }
}
