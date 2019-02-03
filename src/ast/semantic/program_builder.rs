use crate::ast::semantic::{AstWalker, Program, ProgramWalker};
use crate::ast::statement::ProcedureStmt;
use crate::ast::Ast;

pub struct ProgramBuilder;

impl ProgramBuilder {
    fn build(&mut self, ast: Ast) -> Program {
        let root_proc: ProcedureStmt = ast.as_proc_stmt();

        let mut walker = ProgramWalker::new();
        walker.walk(&root_proc);

        walker.program
    }
}
