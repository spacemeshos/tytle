use crate::ast::semantic::{Scope, Variable};
use crate::ast::statement::ProcedureStmt;
use std::collections::{HashMap, HashSet};

pub struct Program {
    procedures: HashSet<String>,
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
