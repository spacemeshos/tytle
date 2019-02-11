use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::ast::Ast;

pub trait AstWalker<'a> {
    fn walk_ast(&mut self, ast: &Ast) {}

    fn walk_proc_stmt(&mut self, proc_stmt: &ProcedureStmt) {
        self.on_proc_start(proc_stmt);

        self.walk_proc_params(proc_stmt);
        self.walk_block_stmt(&proc_stmt.block);

        self.on_proc_end(proc_stmt);
    }

    fn walk_proc_params(&mut self, proc_stmt: &ProcedureStmt) {
        for param in &proc_stmt.params {
            self.on_proc_param(proc_stmt, param);
        }
    }

    fn walk_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Nop => {}
            Statement::Command(ref cmd_stmt) => self.walk_command_stmt(cmd_stmt),
            Statement::Direction(ref direct_stmt) => self.walk_direct_stmt(direct_stmt),
            Statement::If(ref if_stmt) => self.walk_if_stmt(if_stmt),
            Statement::Make(ref make_stmt) => self.walk_make_stmt(make_stmt),
            Statement::Repeat(ref repeat_stmt) => self.walk_repeat_stmt(repeat_stmt),
            Statement::Procedure(ref proc_stmt) => self.walk_proc_stmt(proc_stmt),
        }
    }

    fn walk_if_stmt(&mut self, if_stmt: &IfStmt) {
        self.walk_expr(&if_stmt.cond_expr);

        self.walk_block_stmt(&if_stmt.true_block);

        if if_stmt.false_block.is_some() {
            self.walk_block_stmt(if_stmt.false_block.as_ref().unwrap());
        }
    }

    fn walk_block_stmt(&mut self, block_stmt: &BlockStatement) {
        self.on_block_stmt_start(&block_stmt);

        for stmt in &block_stmt.stmts {
            self.walk_stmt(stmt);
        }

        self.on_block_stmt_end(&block_stmt);
    }

    fn walk_expr(&mut self, expr: &Expression) -> Option<ExpressionType> {
        match expr {
            Expression::Literal(ref lexpr) => self.on_literal_expr(lexpr),
            Expression::ProcCall(ref proc_name, ref proc_params) => {
                self.walk_proc_call_expr(proc_name, proc_params)
            }
            Expression::Binary(binary_op, lexpr, rexpr) => {
                let lexpr_type = self.walk_expr(lexpr);
                let rexpr_type = self.walk_expr(rexpr);

                self.resolve_binary_expr(binary_op, lexpr_type, rexpr_type)
            }
        }
    }

    fn resolve_proc_call_expr(&self, proc_name: &str) -> Option<ExpressionType> {
        None
    }

    fn walk_proc_call_expr(
        &mut self,
        proc_name: &str,
        params_exprs: &Vec<Box<Expression>>,
    ) -> Option<ExpressionType> {
        self.on_proc_call_expr_start(proc_name);

        for param_expr in params_exprs {
            self.on_proc_param_expr_start(param_expr);
            self.walk_expr(param_expr);
            self.on_proc_param_expr_end(param_expr);
        }

        self.on_proc_call_expr_end(proc_name);
        self.resolve_proc_call_expr(proc_name)
    }

    fn walk_command_stmt(&mut self, cmd: &CommandStmt) {
        self.on_command_stmt(cmd);
    }

    fn walk_direct_stmt(&mut self, direct_stmt: &DirectionStmt) {
        self.walk_expr(&direct_stmt.expr);
        self.on_direct_stmt(direct_stmt);
    }

    fn walk_make_stmt(&mut self, make_stmt: &MakeStmt) {
        self.walk_expr(&make_stmt.expr);
        self.on_make_stmt(make_stmt);
    }

    fn walk_repeat_stmt(&mut self, repeat_stmt: &RepeatStmt) {
        self.walk_expr(&repeat_stmt.count_expr);
        self.walk_block_stmt(&repeat_stmt.block);
    }

    // hooks
    // procedure declaration
    fn on_proc_start(&mut self, proc_stmt: &ProcedureStmt) {}
    fn on_proc_end(&mut self, proc_stmt: &ProcedureStmt) {}
    fn on_proc_param(&mut self, proc_stmt: &ProcedureStmt, param: &ProcParam) {}

    // block
    fn on_block_stmt_start(&mut self, block_stmt: &BlockStatement) {}
    fn on_block_stmt_end(&mut self, block_stmt: &BlockStatement) {}

    // expression
    fn on_literal_expr(&mut self, expr: &LiteralExpr) -> Option<ExpressionType> {
        None
    }

    fn resolve_binary_expr(
        &self,
        binary_op: &BinaryOp,
        lexpr_type: Option<ExpressionType>,
        rexpr_type: Option<ExpressionType>,
    ) -> Option<ExpressionType> {
        None
    }

    // procedure call
    fn on_proc_call_expr_start(&mut self, proc_name: &str) {}
    fn on_proc_call_expr_end(&mut self, proc_name: &str) {}
    fn on_proc_param_expr_start(&mut self, param_expr: &Expression) {}
    fn on_proc_param_expr_end(&mut self, param_expr: &Expression) {}

    // misc
    fn on_make_stmt(&mut self, make_stmt: &MakeStmt) {}
    fn on_command_stmt(&mut self, cmd: &CommandStmt) {}
    fn on_direct_stmt(&mut self, direct_stmt: &DirectionStmt) {}
}
