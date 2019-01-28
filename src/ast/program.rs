use crate::ast::{procedure::Procedure, scope::Scope, statement::Statement, variable::Variable};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    // globals: Vec<Variable>,
    // procs: HashMap<String, Procedure>,
    pub statements: Vec<Statement>,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            // globals: Default::default(),
            // procs: Default::default(),
            statements: Default::default(),
        }
    }
}
