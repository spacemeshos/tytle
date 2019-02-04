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
            self.walk_proc_stmt(proc, stmt);
        }
    }

    fn walk_proc_stmt(&mut self, proc: &ProcedureStmt, stmt: &Statement) {
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

    fn walk_if_stmt(&mut self, proc: &ProcedureStmt, if_stmt: &IfStmt) {
        let expr = self.walk_expr(proc, &if_stmt.cond_expr);
        // TODO: expect `expr` to return a bool

        self.walk_block_stmt(proc, &if_stmt.true_block);

        if if_stmt.false_block.is_some() {
            self.walk_block_stmt(proc, if_stmt.false_block.as_ref().unwrap());
        }
    }

    fn walk_block_stmt(&mut self, proc: &ProcedureStmt, block: &BlockStatement) {
        for stmt in &block.stmts {
            self.walk_proc_stmt(proc, stmt);
        }
    }

    fn walk_expr(&mut self, proc: &ProcedureStmt, expr: &Expression) -> ExpressionType {
        match expr {
            Expression::Literal(ref literal) => ExpressionType::NotSure,
            Expression::Binary(binary_op, ref lexpr, ref rexpr) => {
                let lexpr_type = self.walk_expr(proc, lexpr);
                let rexpr_type = self.walk_expr(proc, rexpr);

                ExpressionType::NotSure
            }
        }
    }

    fn walk_command_stmt(&mut self, proc: &ProcedureStmt, cmd: &CommandStmt) {}

    fn walk_proc_param(&mut self, proc: &ProcedureStmt, param: &str) {
        // TODO: make sure the param name is unique
        // TODO: enforce each param to have a type
    }

    fn walk_direct_stmt(&mut self, proc: &ProcedureStmt, direct_stmt: &DirectionStmt) {
        let expr = self.walk_expr(proc, &direct_stmt.expr);
        // TODO: expect `expr` to return an int
    }

    fn walk_make_stmt(&mut self, proc: &ProcedureStmt, make_stmt: &MakeStmt) {
        // TODO: assert the current scope didn't declare the variable yet
    }

    fn walk_repeat_stmt(&mut self, proc: &ProcedureStmt, repeat_stmt: &RepeatStmt) {
        let expr = self.walk_expr(proc, &repeat_stmt.count_expr);
        // TODO: expect `expr` to return an int
    }
}
