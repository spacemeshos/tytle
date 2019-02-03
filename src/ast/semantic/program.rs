use crate::ast::semantic::{Scope, Variable};
use crate::ast::statement::ProcedureStmt;
use std::collections::HashMap;

pub struct Program {
    procedures: HashMap<String, ProcedureStmt>,
    globals: HashMap<String, Variable>,
    scopes: Vec<Scope>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            procedures: Default::default(),
            globals: Default::default(),
            scopes: Default::default(),
        }
    }
}
