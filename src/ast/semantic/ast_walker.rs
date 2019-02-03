use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::ast::Ast;

pub trait AstWalker<'a> {
    fn walk(&mut self, root: &ProcedureStmt) {
        assert_eq!(root.name, "__main__");

        self.walk_proc(None, root);
    }

    fn walk_proc(&mut self, parent_proc: Option<&ProcedureStmt>, proc: &ProcedureStmt) {
        self.walk_proc_params(proc);
        self.walk_proc_body(proc);
    }

    fn walk_proc_params(&mut self, proc: &ProcedureStmt) {
        for param in &proc.params {
            self.walk_proc_param(proc, param);
        }
    }

    fn walk_proc_body(&mut self, proc: &ProcedureStmt) {
        for stmt in &proc.block.stmts {
            match stmt {
                Statement::Command(ref cmd_stmt) => self.walk_command_stmt(proc, cmd_stmt),
                Statement::Direction(ref direct_stmt) => self.walk_direct_stmt(proc, direct_stmt),
                Statement::If(ref if_stmt) => self.walk_if_stmt(proc, if_stmt),
                Statement::Make(ref make_stmt) => self.walk_make_stmt(proc, make_stmt),
                Statement::Repeat(ref repeat_stmt) => self.walk_repeat_stmt(proc, repeat_stmt),
                Statement::Procedure(ref proc_stmt) => self.walk_proc(Some(proc), proc_stmt),
                _ => panic!(),
            }
        }
    }

    fn walk_if_stmt(&mut self, proc: &ProcedureStmt, if_stmt: &IfStmt) {
        self.walk_expr(&if_stmt.cond_expr);
        self.walk_block(&if_stmt.true_block);

        if if_stmt.false_block.is_some() {
            self.walk_block(if_stmt.false_block.as_ref().unwrap());
        }
    }

    fn walk_block(&mut self, block: &BlockStatement) {}

    fn walk_expr(&mut self, expr: &Expression) {}

    fn walk_command_stmt(&mut self, proc: &ProcedureStmt, cmd: &CommandStmt) {
        //
    }

    fn walk_proc_param(&mut self, proc: &ProcedureStmt, param: &str) {
        //
    }

    fn walk_direct_stmt(&mut self, proc: &ProcedureStmt, direct: &DirectionStmt) {
        //
    }

    fn walk_make_stmt(&mut self, proc: &ProcedureStmt, make_stmt: &MakeStmt) {
        //
    }

    fn walk_repeat_stmt(&mut self, proc: &ProcedureStmt, repeat_stmt: &RepeatStmt) {
        //
    }
}
