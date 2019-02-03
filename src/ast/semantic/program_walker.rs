use std::collections::HashMap;

use crate::ast::semantic::{AstWalker, Program, Scope, Variable};
use crate::ast::statement::{BlockStatement, ProcedureStmt};
use crate::ast::Ast;

pub struct ProgramWalker {
    pub program: Program,
}

impl ProgramWalker {
    pub fn new() -> Self {
        Self { program: Program::new() }
    }
}

impl<'a> AstWalker<'a> for ProgramWalker {}
