use crate::ast::expression::*;
use crate::ast::semantic::AstWalkError;
use crate::ast::statement::*;
use crate::ast::Ast;

pub type AstWalkResult = Result<(), AstWalkError>;

pub trait AstWalker<'a> {
    fn walk_ast(&mut self, ast: &mut Ast) -> AstWalkResult {
        for stmt in &mut ast.statements {
            self.walk_stmt(stmt)?;
        }

        Ok(())
    }

    fn walk_stmt(&mut self, stmt: &mut Statement) -> AstWalkResult {
        match stmt {
            Statement::NOP | Statement::EOF => {}
            Statement::Command(ref mut cmd_stmt) => self.walk_command_stmt(cmd_stmt)?,
            Statement::Direction(ref mut direct_stmt) => self.walk_direct_stmt(direct_stmt)?,
            Statement::If(ref mut if_stmt) => self.walk_if_stmt(if_stmt)?,
            Statement::Make(ref mut make_stmt) => self.walk_make_stmt(make_stmt)?,
            Statement::Repeat(ref mut repeat_stmt) => self.walk_repeat_stmt(repeat_stmt)?,
            Statement::Procedure(ref mut proc_stmt) => self.walk_proc_stmt(proc_stmt)?,
        }

        Ok(())
    }

    fn walk_proc_stmt(&mut self, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        self.on_proc_start(proc_stmt)?;

        self.walk_proc_params(proc_stmt)?;

        // we don't call `walk_proc_stmt` in order to avoid starting a new scope.
        // we want the procedure params and the procedure root-block to share the same scope
        for stmt in &mut proc_stmt.block.stmts {
            self.walk_stmt(stmt)?;
        }

        self.on_proc_end(proc_stmt)?;

        Ok(())
    }

    fn walk_proc_params(&mut self, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        for param in &mut proc_stmt.params {
            self.on_proc_param(&proc_stmt.name, param)?;
        }

        Ok(())
    }

    fn walk_if_stmt(&mut self, if_stmt: &mut IfStmt) -> AstWalkResult {
        self.walk_expr(&mut if_stmt.cond_expr)?;

        self.walk_block_stmt(&mut if_stmt.true_block)?;

        if if_stmt.false_block.is_some() {
            self.walk_block_stmt(if_stmt.false_block.as_mut().unwrap())?;
        }

        self.on_if_stmt(if_stmt)
    }

    fn walk_block_stmt(&mut self, block_stmt: &mut BlockStatement) -> AstWalkResult {
        self.on_block_stmt_start(block_stmt)?;

        for stmt in &mut block_stmt.stmts {
            self.walk_stmt(stmt)?;
        }

        self.on_block_stmt_end(block_stmt)
    }

    fn walk_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        match expr.expr_ast {
            ExpressionAst::Literal(_) => self.on_literal_expr(expr),
            ExpressionAst::ProcCall(ref proc_name, ref mut proc_params) => {
                self.walk_proc_call_expr(proc_name, proc_params)?;

                self.on_proc_call_expr(expr)
            }
            ExpressionAst::Binary(_, ref mut lexpr, ref mut rexpr) => {
                self.walk_expr(lexpr)?;
                self.walk_expr(rexpr)?;

                self.on_binary_expr(expr)
            }
            ExpressionAst::Not(ref mut inner_expr) => {
                self.walk_expr(inner_expr)?;

                self.on_not_expr(expr)
            }
        }
    }

    fn walk_proc_call_expr(
        &mut self,
        proc_name: &str,
        params_exprs: &mut Vec<Expression>,
    ) -> AstWalkResult {
        self.on_proc_call_expr_start(proc_name)?;

        for param_expr in params_exprs {
            self.on_proc_param_expr_start(param_expr)?;
            self.walk_expr(param_expr)?;
            self.on_proc_param_expr_end(param_expr)?;
        }

        Ok(())
    }

    fn walk_command_stmt(&mut self, cmd: &mut CommandStmt) -> AstWalkResult {
        self.on_command_stmt(cmd)
    }

    fn walk_direct_stmt(&mut self, direct_stmt: &mut DirectionStmt) -> AstWalkResult {
        self.walk_expr(&mut direct_stmt.expr)?;
        self.on_direct_stmt(direct_stmt)
    }

    fn walk_make_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.walk_expr(&mut make_stmt.expr)?;

        match make_stmt.kind {
            MakeStmtKind::Global => self.on_make_global_stmt(make_stmt)?,
            MakeStmtKind::Local => self.on_make_local_stmt(make_stmt)?,
            MakeStmtKind::Assign => self.on_make_assign_stmt(make_stmt)?,
        }

        Ok(())
    }

    fn walk_repeat_stmt(&mut self, repeat_stmt: &mut RepeatStmt) -> AstWalkResult {
        self.walk_expr(&mut repeat_stmt.count_expr)?;

        self.walk_block_stmt(&mut repeat_stmt.block)?;

        self.on_repeat_stmt(repeat_stmt)
    }

    // hooks
    fn on_proc_start(&mut self, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_end(&mut self, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_param(&mut self, proc_name: &str, proc_param: &mut ProcParam) -> AstWalkResult {
        Ok(())
    }

    // block
    fn on_block_stmt_start(&mut self, block_stmt: &mut BlockStatement) -> AstWalkResult {
        Ok(())
    }

    fn on_block_stmt_end(&mut self, block_stmt: &mut BlockStatement) -> AstWalkResult {
        Ok(())
    }

    // expression
    fn on_literal_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_call_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_binary_expr(&mut self, bin_expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_not_expr(&mut self, inner_expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    // procedure call
    fn on_proc_call_expr_start(&mut self, proc_name: &str) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_call_expr_end(&mut self, proc_name: &str) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_param_expr_start(&mut self, param_expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_param_expr_end(&mut self, param_expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    // `MAKE` statements
    fn on_make_global_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_make_local_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_make_assign_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        Ok(())
    }

    // misc
    fn on_if_stmt(&mut self, if_stmt: &mut IfStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_repeat_stmt(&mut self, repeat_stmt: &mut RepeatStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_command_stmt(&mut self, cmd: &mut CommandStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_direct_stmt(&mut self, direct_stmt: &mut DirectionStmt) -> AstWalkResult {
        Ok(())
    }
}
