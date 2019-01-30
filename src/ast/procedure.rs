use crate::ast::statement::BlockStatement;
use crate::ast::variable::Variable;

use std::fmt;

pub struct Procedure {
    pub name: String,
    pub block: BlockStatement,
}

impl Procedure {
    pub fn new(name: String) -> Self {
        Self {
            name,
            block: Default::default(),
        }
    }
}

impl fmt::Debug for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let proc_str = r#"
        CODE:
        "#;

        write!(f, "{}", proc_str)
    }
}
