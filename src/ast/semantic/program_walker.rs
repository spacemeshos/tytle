use crate::ast::semantic::{AstWalker, Program, Scope, Variable};
use crate::ast::Ast;

use std::collections::HashMap;

use crate::ast::statement::{BlockStatement, ProcedureStmt};

pub struct ProgramWalker {
    program: Option<Program>,
}

impl ProgramWalker {
    pub fn new() -> Self {
        Self { program: None }
    }
}

impl<'a> AstWalker<'a> for ProgramWalker {}
