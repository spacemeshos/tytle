use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

pub struct AstTypeCheck<'a> {
    sym_table: &'a SymbolTable,
}

impl<'a> AstTypeCheck<'a> {
    pub fn new(sym_table: &'a SymbolTable) -> Self {
        Self { sym_table }
    }
}

impl<'a> AstWalker<'a> for AstTypeCheck<'a> {
    fn on_literal_expr(&mut self, expr: &LiteralExpr) -> AstWalkResult {
        Ok(())
    }

    fn on_binary_expr(
        &mut self,
        binary_op: &BinaryOp,
        lexpr: &Expression,
        rexpr: &Expression,
    ) -> AstWalkResult {
        Ok(())
    }

    fn on_proc_call_expr_start(&mut self, proc_name: &str) -> AstWalkResult {
        Ok(())
    }

    fn on_make_assign_stmt(&mut self, make_stmt: &MakeStmt) -> AstWalkResult {
        Ok(())
    }
}
