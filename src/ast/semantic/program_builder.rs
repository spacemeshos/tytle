use crate::ast::semantic::AstWalker;
use crate::ast::Ast;
use std::collections::HashMap;

use crate::ast::statement::{Statement, ProcedureStmt};


pub struct ProgramBuilder<'a> {
    procedures: HashMap<String, &'a ProcedureStmt>
}

impl<'a> ProgramBuilder<'a> {
    pub fn new() -> Self {
        Self {
            procedures: Default::default()
        }
    }
}

// impl<'a> AstWalker<'a> for ProgramBuilder<'a> {
// }
