use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

pub struct AstTypeCheck<'a, 'b: 'a> {
    sym_visitor: &'a mut SymbolTableVisitor<'b>,
}

impl<'a, 'b> AstTypeCheck<'a, 'b> {
    pub fn new(sym_visitor: &'a mut SymbolTableVisitor<'b>) -> Self {
        Self { sym_visitor }
    }

    pub fn check(&mut self, ast: &mut Ast) {
        self.walk_ast(ast);
    }
}

impl<'a, 'b> AstWalker<'a> for AstTypeCheck<'a, 'b> {
    fn on_literal_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        let lit_expr = expr.as_lit_expr();

        // let expr_type = match lit_expr {
        //     LiteralExpr::Bool(_) => ExpressionType::Bool,
        //     LiteralExpr::Int(_) => ExpressionType::Int,
        //     LiteralExpr::Str(_) => ExpressionType::Str,
        //     LiteralExpr::Var(v) => {
        //         let symbol = self.sym_visitor.lookup_symbol_recur(v, &SymbolKind::Var);
        //         let var: &Variable = symbol.unwrap().as_var();
        //
        //         if let Some(ref var_type) = var.resolved_type {
        //             var_type
        //         } else {
        //             panic!(format!("variable `{}`, type couldn't be inferred", v))
        //         }
        //     }
        // };

        // expr.expr_type = Some(expr_type);

        Ok(())
    }

    fn on_proc_call_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    fn on_binary_expr(&mut self, bin_expr: &mut Expression) -> AstWalkResult {
        Ok(())
    }

    // `MAKE` statements
    fn on_make_global_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let symbol = self
            .sym_visitor
            .lookup_symbol_recur_mut(make_stmt.var.as_str(), &SymbolKind::Var);

        let var: &mut Variable = symbol.unwrap().as_var_mut();

        let expr_type: Option<&ExpressionType> = make_stmt.expr.expr_type.as_ref();
        // var.resolved_type = expr_type;

        Ok(())
    }

    fn on_make_local_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        Ok(())
    }

    fn on_make_assign_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let symbol = self
            .sym_visitor
            .lookup_symbol_recur_mut(make_stmt.var.as_str(), &SymbolKind::Var);

        let var: &mut Variable = symbol.unwrap().as_var_mut();

        if var.resolved_type.is_none() {
            panic!()
        }

        let expr_type: Option<&ExpressionType> = make_stmt.expr.expr_type.as_ref();

        // if expr_type != var.resolved_type {
        //     panic!("type mismtach");
        // }

        Ok(())
    }

    // visiting scopes
    fn on_proc_start(&mut self, proc_stmt: &mut ProcedureStmt) -> AstWalkResult {
        // self.sym_visitor.next_scope();
        Ok(())
    }

    fn on_block_stmt_start(&mut self, _block_stmt: &mut BlockStatement) -> AstWalkResult {
        // self.sym_visitor.next_scope();
        Ok(())
    }
}
