use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::ast::Ast;

pub trait AstWalker<'a> {
    fn walk(&mut self, root: &ProcedureStmt) {
        assert_eq!(root.name, "__main__");

        self.walk_proc(None, root);
    }

    fn walk_proc(&mut self, parent_proc: Option<&ProcedureStmt>, proc: &ProcedureStmt) {
        self.on_proc_start(parent_proc, proc);

        self.walk_proc_params(proc);
        self.walk_proc_body(proc);

        self.on_proc_end(parent_proc, proc);
    }

    fn walk_proc_params(&mut self, proc: &ProcedureStmt) {
        for param in &proc.params {
            self.on_proc_param(proc, param);
        }
    }

    fn walk_proc_body(&mut self, proc: &ProcedureStmt) {
        for stmt in &proc.block.stmts {
            self.walk_proc_stmt(proc, stmt);
        }
    }

    fn walk_proc_stmt(&mut self, proc: &ProcedureStmt, stmt: &Statement) {
        match stmt {
            Statement::Nop => {}
            Statement::Command(ref cmd_stmt) => self.walk_command_stmt(proc, cmd_stmt),
            Statement::Direction(ref direct_stmt) => self.walk_direct_stmt(proc, direct_stmt),
            Statement::If(ref if_stmt) => self.walk_if_stmt(proc, if_stmt),
            Statement::Make(ref make_stmt) => self.walk_make_stmt(proc, make_stmt),
            Statement::Repeat(ref repeat_stmt) => self.walk_repeat_stmt(proc, repeat_stmt),
            Statement::Procedure(ref proc_stmt) => self.walk_proc(Some(proc), proc_stmt),
        }
    }

    fn walk_if_stmt(&mut self, proc: &ProcedureStmt, if_stmt: &IfStmt) {
        let expr_type = self.walk_expr(proc, &if_stmt.cond_expr);

        self.walk_block_stmt(proc, &if_stmt.true_block);

        if if_stmt.false_block.is_some() {
            self.walk_block_stmt(proc, if_stmt.false_block.as_ref().unwrap());
        }
    }

    fn walk_block_stmt(&mut self, proc: &ProcedureStmt, block: &BlockStatement) {
        self.on_block_stmt_start(proc);

        for stmt in &block.stmts {
            self.walk_proc_stmt(proc, stmt);
        }

        self.on_block_stmt_end(proc);
    }

    fn walk_expr(&mut self, proc: &ProcedureStmt, expr: &Expression) {
        match expr {
            Expression::Literal(ref lexpr) => self.on_literal_expr(proc, lexpr),
            Expression::ProcCall(ref proc_name, ref params_exprs) => {
                self.walk_proc_call_expr(proc, proc_name, params_exprs);
            }
            Expression::Binary(binary_op, lexpr, rexpr) => {
                self.walk_expr(proc, lexpr);
                self.walk_expr(proc, rexpr);

                self.on_binary_expr_end(proc, binary_op);
            }
        }
    }

    fn walk_proc_call_expr(
        &mut self,
        proc: &ProcedureStmt,
        proc_name: &str,
        params_exprs: &Vec<Box<Expression>>,
    ) {
        self.on_proc_call_expr_start(proc, proc_name);

        for param_expr in params_exprs {
            self.walk_expr(proc, param_expr);
            self.on_proc_param_expr_end(proc, param_expr);
        }

        self.on_proc_call_expr_end(proc, proc_name);
    }

    fn walk_command_stmt(&mut self, proc: &ProcedureStmt, cmd: &CommandStmt) {
        self.on_command_stmt(proc, cmd);
    }

    fn walk_direct_stmt(&mut self, proc: &ProcedureStmt, direct_stmt: &DirectionStmt) {
        self.on_direct_stmt(proc, direct_stmt);
    }

    fn walk_make_stmt(&mut self, proc: &ProcedureStmt, make_stmt: &MakeStmt) {
        self.on_make_stmt(proc, make_stmt);
    }

    fn walk_repeat_stmt(&mut self, proc: &ProcedureStmt, repeat_stmt: &RepeatStmt) {
        let expr_type = self.walk_expr(proc, &repeat_stmt.count_expr);

        self.walk_block_stmt(proc, &repeat_stmt.block);
    }

    // hooks
    fn on_proc_start(&mut self, parent_proc: Option<&ProcedureStmt>, proc: &ProcedureStmt) {}
    fn on_proc_end(&mut self, parent_proc: Option<&ProcedureStmt>, proc: &ProcedureStmt) {}
    fn on_proc_param(&mut self, proc: &ProcedureStmt, param: &ProcParam) {}
    fn on_block_stmt_start(&mut self, proc: &ProcedureStmt) {}
    fn on_block_stmt_end(&mut self, proc: &ProcedureStmt) {}
    fn on_make_stmt(&mut self, proc: &ProcedureStmt, make_stmt: &MakeStmt) {}
    fn on_command_stmt(&mut self, proc: &ProcedureStmt, cmd: &CommandStmt) {}

    fn on_literal_expr(&mut self, proc: &ProcedureStmt, expr: &LiteralExpr) {}
    fn on_binary_expr_end(&mut self, proc: &ProcedureStmt, binary_op: &BinaryOp) {}

    fn on_proc_call_expr_start(&mut self, proc: &ProcedureStmt, proc_name: &str) {}
    fn on_proc_call_expr_end(&mut self, proc: &ProcedureStmt, proc_name: &str) {}
    fn on_proc_param_expr_end(&mut self, proc: &ProcedureStmt, param_expr: &Expression) {}

    fn on_direct_stmt(&mut self, proc: &ProcedureStmt, direct_stmt: &DirectionStmt) {
        self.walk_expr(proc, &direct_stmt.expr)
    }
}
