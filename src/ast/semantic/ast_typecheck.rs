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

    pub fn check(&mut self, ast: &mut Ast) -> AstWalkResult {
        self.walk_ast(ast)
    }
}

impl<'a, 'b> AstWalker<'a> for AstTypeCheck<'a, 'b> {
    fn on_literal_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        let lit_expr = expr.as_lit_expr();

        let expr_type = match lit_expr {
            LiteralExpr::Bool(_) => ExpressionType::Bool,
            LiteralExpr::Int(_) => ExpressionType::Int,
            LiteralExpr::Str(_) => ExpressionType::Str,
            LiteralExpr::Var(v) => {
                let symbol = self.sym_visitor.lookup_recur(v, &SymbolKind::Var);
                let var: &Variable = symbol.unwrap().as_var();

                if let Some(ref var_type) = var.var_type {
                    var_type.to_owned()
                } else {
                    panic!(format!("variable `{}`, type couldn't be inferred", v))
                }
            }
        };

        expr.expr_type = Some(expr_type);

        Ok(())
    }

    fn on_proc_call_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        let (proc_name, proc_args_exprs) = expr.as_proc_call_expr();

        // TODO: ask directly the root scope
        let symbol = self.sym_visitor.lookup_recur(proc_name, &SymbolKind::Proc);

        let proc: &Procedure = symbol.unwrap().as_proc();

        let expected_params_types = proc.params_types.clone().unwrap();
        let expected_args_count = expected_params_types.len();
        let actual_args_count = proc_args_exprs.len();

        if expected_args_count != actual_args_count {
            let err = AstWalkError::InvalidProcCallArgsCount(
                proc_name.clone(),
                expected_args_count,
                actual_args_count,
            );
            return Err(err);
        }


        let mut i = 1;
        let mut actual_iter = proc_args_exprs.iter();
        let mut expected_iter = expected_params_types.iter();

        while i <= expected_args_count {
            let arg_expr: &Expression = actual_iter.next().unwrap();

            let actual_type: ExpressionType = arg_expr.expr_type.clone().unwrap();
            let expected_type: &ExpressionType = expected_iter.next().unwrap();

            if *expected_type != actual_type {
                let err = AstWalkError::InvalidProcCallArgType(i, expected_type.clone(), actual_type.clone());
                return Err(err);
            }

            i += 1;
        }

        expr.expr_type = proc.return_type.clone();

        Ok(())
    }

    fn on_binary_expr(&mut self, expr: &mut Expression) -> AstWalkResult {
        let (bin_op, lexpr, rexpr) = expr.as_binary_expr();

        if lexpr.expr_type != rexpr.expr_type {
            let ltype = lexpr.expr_type.clone().unwrap();
            let rtype = rexpr.expr_type.clone().unwrap();

            let err = AstWalkError::InvalidBinaryOp(bin_op.clone(), ltype, rtype);
            return Err(err);
        }

        expr.expr_type = lexpr.expr_type.clone();

        Ok(())
    }

    // `MAKE` statements
    fn on_make_global_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.typecheck_var_declare(make_stmt)
    }

    fn on_make_local_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.typecheck_var_declare(make_stmt)
    }

    fn on_make_assign_stmt(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let symbol = self
            .sym_visitor
            .lookup_recur_mut(make_stmt.var.as_str(), &SymbolKind::Var);

        let var: &mut Variable = symbol.unwrap().as_var_mut();

        if var.var_type.is_none() {
            panic!()
        }

        let expr_type: ExpressionType = make_stmt.expr.expr_type.as_ref().unwrap().to_owned();
        let var_type: ExpressionType = var.var_type.clone().unwrap();

        if expr_type != var_type {
            let err = AstWalkError::TypeMismatch(var_type, expr_type);
            return Err(err);
        }

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

impl<'a, 'b> AstTypeCheck<'a, 'b> {
    fn typecheck_var_declare(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let symbol = self
            .sym_visitor
            .lookup_recur_mut(make_stmt.var.as_str(), &SymbolKind::Var);

        let var: &mut Variable = symbol.unwrap().as_var_mut();

        let expr_type: &ExpressionType = make_stmt.expr.expr_type.as_ref().unwrap();
        var.var_type = Some(expr_type.to_owned());

        Ok(())
    }
}
