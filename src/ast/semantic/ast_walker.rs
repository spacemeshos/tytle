use crate::ast::expression::*;
use crate::ast::semantic::AstWalkError;
use crate::ast::statement::*;
use crate::ast::Ast;

pub type AstWalkResult = Result<(), AstWalkError>;

pub trait AstWalker<'a> {
    fn walk_ast(&mut self, ast: &mut Ast) -> AstWalkResult {
        for stmt in &mut ast.statements {
            self.walk_stmt("__main__", stmt)?;
        }

        Ok(())
    }

    fn walk_stmt(&mut self, ctx_proc: &str, stmt: &mut Statement) -> AstWalkResult {
        match stmt {
            Statement::NOP | Statement::EOF => {}
            Statement::Command(ref mut cmd_stmt) => self.on_command(ctx_proc, cmd_stmt)?,
            Statement::Direction(ref mut direct_stmt) => {
                self.walk_direct_stmt(ctx_proc, direct_stmt)?
            }
            Statement::If(ref mut if_stmt) => self.walk_if_stmt(ctx_proc, if_stmt)?,
            Statement::Make(ref mut make_stmt) => self.walk_make_stmt(ctx_proc, make_stmt)?,
            Statement::Repeat(ref mut repeat_stmt) => {
                self.walk_repeat_stmt(ctx_proc, repeat_stmt)?
            }
            Statement::Procedure(ref mut proc_stmt) => self.walk_proc_stmt(ctx_proc, proc_stmt)?,
            Statement::Return(ref mut return_stmt) => self.walk_ret_stmt(ctx_proc, return_stmt)?,
            Statement::Expression(ref mut expr) => self.walk_expr_stmt(ctx_proc, expr)?,
        }

        Ok(())
    }

    fn walk_proc_stmt(&mut self, ctx_proc: &str, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        self.on_proc_start(ctx_proc, proc_stmt)?;

        self.walk_proc_params(ctx_proc, proc_stmt)?;

        // we don't call `walk_proc_stmt` in order to avoid starting a new scope.
        // we want the procedure params and the procedure root-block to share the same scope
        for stmt in &mut proc_stmt.block.stmts {
            self.walk_stmt(proc_stmt.name.as_str(), stmt)?;
        }

        self.on_proc_end(ctx_proc, proc_stmt)?;

        Ok(())
    }

    fn walk_ret_stmt(&mut self, ctx_proc: &str, ret_stmt: &mut ReturnStmt) -> AstWalkResult {
        if ret_stmt.expr.is_some() {
            let expr = ret_stmt.expr.as_mut().unwrap();
            self.walk_expr(ctx_proc, expr)?;
        }

        self.on_ret_stmt(ctx_proc, ret_stmt)?;

        Ok(())
    }

    fn walk_expr_stmt(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        self.walk_expr(ctx_proc, expr)?;

        self.on_expr_stmt(ctx_proc, expr)?;

        Ok(())
    }

    fn walk_proc_params(
        &mut self,
        _ctx_proc: &str,
        proc_stmt: &mut ProcedureStmt,
    ) -> AstWalkResult {
        for param in &mut proc_stmt.params {
            self.on_proc_param(&proc_stmt.name, param)?;
        }

        Ok(())
    }

    fn walk_if_stmt(&mut self, ctx_proc: &str, if_stmt: &mut IfStmt) -> AstWalkResult {
        self.walk_expr(ctx_proc, &mut if_stmt.cond_expr)?;

        self.walk_block_stmt(ctx_proc, &mut if_stmt.true_block)?;

        if if_stmt.false_block.is_some() {
            self.walk_block_stmt(ctx_proc, if_stmt.false_block.as_mut().unwrap())?;
        }

        self.on_if_stmt(ctx_proc, if_stmt)
    }

    fn walk_block_stmt(
        &mut self,
        ctx_proc: &str,
        block_stmt: &mut BlockStatement,
    ) -> AstWalkResult {
        self.on_block_stmt_start(ctx_proc, block_stmt)?;

        for stmt in &mut block_stmt.stmts {
            self.walk_stmt(ctx_proc, stmt)?;
        }

        self.on_block_stmt_end(ctx_proc, block_stmt)
    }

    fn walk_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        match expr.expr_ast {
            ExpressionAst::Literal(_) => self.on_literal_expr(ctx_proc, expr),
            ExpressionAst::ProcCall(ref call_name, ref mut call_params) => {
                self.walk_proc_call_expr(ctx_proc, call_name, call_params)?;

                self.on_proc_call_expr(ctx_proc, expr)
            }
            ExpressionAst::Binary(_, ref mut lexpr, ref mut rexpr) => {
                self.walk_expr(ctx_proc, lexpr)?;
                self.walk_expr(ctx_proc, rexpr)?;

                self.on_binary_expr(ctx_proc, expr)
            }
            ExpressionAst::Parentheses(ref mut inner_expr) => {
                self.walk_expr(ctx_proc, inner_expr)?;

                self.on_parentheses_expr(ctx_proc, expr)
            }
            ExpressionAst::Not(ref mut inner_expr) => {
                self.walk_expr(ctx_proc, inner_expr)?;

                self.on_not_expr(ctx_proc, expr)
            }
        }
    }

    fn walk_proc_call_expr(
        &mut self,
        ctx_proc: &str,
        call_name: &str,
        call_params: &mut Vec<Expression>,
    ) -> AstWalkResult {
        self.on_proc_call_expr_start(ctx_proc, call_name)?;

        for call_param in call_params {
            self.on_proc_call_param_expr_start(ctx_proc, call_param)?;
            self.walk_expr(ctx_proc, call_param)?;
            self.on_proc_call_param_expr_end(ctx_proc, call_param)?;
        }

        self.on_proc_call_expr_end(ctx_proc, call_name)?;

        Ok(())
    }

    fn walk_direct_stmt(
        &mut self,
        ctx_proc: &str,
        direct_stmt: &mut DirectionStmt,
    ) -> AstWalkResult {
        self.walk_expr(ctx_proc, &mut direct_stmt.expr)?;
        self.on_direct_stmt(ctx_proc, direct_stmt)
    }

    fn walk_make_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.walk_expr(ctx_proc, &mut make_stmt.expr)?;

        match make_stmt.kind {
            MakeStmtKind::Global => self.on_make_global_stmt(ctx_proc, make_stmt)?,
            MakeStmtKind::Local => self.on_make_local_stmt(ctx_proc, make_stmt)?,
            MakeStmtKind::Assign => self.on_make_assign_stmt(ctx_proc, make_stmt)?,
        }

        Ok(())
    }

    fn walk_repeat_stmt(&mut self, ctx_proc: &str, repeat_stmt: &mut RepeatStmt) -> AstWalkResult {
        self.walk_expr(ctx_proc, &mut repeat_stmt.count_expr)?;

        self.walk_block_stmt(ctx_proc, &mut repeat_stmt.block)?;

        self.on_repeat_stmt(ctx_proc, repeat_stmt)
    }

    // hooks
    fn on_proc_start(&mut self, ctx_proc: &str, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_end(&mut self, ctx_proc: &str, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_param(&mut self, ctx_proc: &str, proc_param: &mut ProcParam) -> AstWalkResult {
        Ok(())
    }

    // block
    fn on_block_stmt_start(
        &mut self,
        ctx_proc: &str,
        block_stmt: &mut BlockStatement,
    ) -> AstWalkResult {
        Ok(())
    }

    fn on_block_stmt_end(
        &mut self,
        ctx_proc: &str,
        block_stmt: &mut BlockStatement,
    ) -> AstWalkResult {
        Ok(())
    }

    // expression
    fn on_literal_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_call_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_binary_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_not_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_parentheses_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_expr_stmt(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    // procedure call
    fn on_proc_call_expr_start(&mut self, ctx_proc: &str, call_proc_name: &str) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_call_expr_end(&mut self, ctx_proc: &str, proc_name: &str) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_call_param_expr_start(
        &mut self,
        ctx_proc: &str,
        param_expr: &mut Expression,
    ) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_call_param_expr_end(
        &mut self,
        ctx_proc: &str,
        param_expr: &mut Expression,
    ) -> AstWalkResult {
        Ok(())
    }

    // `MAKE` statements
    fn on_make_global_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_make_local_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_make_assign_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        Ok(())
    }

    // misc
    fn on_if_stmt(&mut self, ctx_proc: &str, if_stmt: &mut IfStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_repeat_stmt(&mut self, ctx_proc: &str, repeat_stmt: &mut RepeatStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_ret_stmt(&mut self, ctx_proc: &str, return_stmt: &mut ReturnStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_command(&mut self, ctx_proc: &str, cmd: &mut Command) -> AstWalkResult {
        Ok(())
    }

    fn on_direct_stmt(&mut self, ctx_proc: &str, direct_stmt: &mut DirectionStmt) -> AstWalkResult {
        Ok(())
    }
}
