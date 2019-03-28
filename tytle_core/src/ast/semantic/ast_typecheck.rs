use crate::ast::semantic::*;
use crate::ast::Ast;
use crate::ast::{expression::*, statement::*};

pub struct AstTypeCheck<'env> {
    env: &'env mut Environment,
}

impl<'env> AstTypeCheck<'env> {
    pub fn new(env: &'env mut Environment) -> Self {
        Self { env }
    }

    pub fn check(&mut self, ast: &mut Ast) -> AstWalkResult {
        self.walk_ast(ast)
    }
}

impl<'env> AstWalker for AstTypeCheck<'env> {
    fn on_literal_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let lit_expr: &LiteralExpr = expr.as_lit_expr();

        let expr_type = match lit_expr {
            LiteralExpr::Bool(_) => ExpressionType::Bool,
            LiteralExpr::Int(_) => ExpressionType::Int,
            LiteralExpr::Str(_) => ExpressionType::Str,
            LiteralExpr::Var(var_name, var_id_wrapped) => {
                let var_id = var_id_wrapped.unwrap();
                let var: &Variable = self.env.symbol_table.get_var_by_id(var_id);

                if let Some(ref var_type) = var.var_type {
                    var_type.to_owned()
                } else {
                    panic!(format!(
                        "variable `{}`, type couldn't be inferred",
                        var_name
                    ))
                }
            }
        };

        expr.expr_type = Some(expr_type);

        Ok(())
    }

    fn on_parentheses_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let inner_expr = expr.as_parentheses_expr();

        // we copy the inner expresison to the outer parentheses expression
        expr.expr_type = inner_expr.expr_type.clone();

        Ok(())
    }

    fn on_not_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let inner_expr = expr.as_not_expr();

        if inner_expr.expr_type != Some(ExpressionType::Bool) {
            let expr_str = PrettyPrintAst::pprint_expr(inner_expr);
            let err = AstWalkError::NotBooleanExpr(expr_str);
            return Err(err);
        }

        expr.expr_type = Some(ExpressionType::Bool);

        Ok(())
    }

    fn on_proc_call_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let (proc_name, proc_args_exprs, _proc_id) = expr.as_proc_call_expr();

        let root_scope_id = 0;
        let symbol =
            self.env
                .symbol_table
                .lookup_recur(root_scope_id, proc_name, &SymbolKind::Proc);

        let proc: &Procedure = symbol.unwrap().as_proc();

        let expected_params_types = proc.params_types.clone();
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

        let mut arg_pos = 1;
        let mut actual_iter = proc_args_exprs.iter();
        let mut expected_iter = expected_params_types.iter();

        while arg_pos <= expected_args_count {
            let arg_expr: &Expression = actual_iter.next().unwrap();

            let actual_type: ExpressionType = arg_expr.expr_type.clone().unwrap();
            let expected_type: &ExpressionType = expected_iter.next().unwrap();

            if *expected_type != actual_type {
                let err = AstWalkError::InvalidProcCallArgType(
                    arg_pos,
                    expected_type.clone(),
                    actual_type.clone(),
                );
                return Err(err);
            }

            arg_pos += 1;
        }

        expr.expr_type = Some(proc.return_type.clone());

        Ok(())
    }

    fn on_binary_expr(&mut self, ctx_proc: &str, expr: &mut Expression) -> AstWalkResult {
        let (bin_op, lexpr, rexpr) = expr.as_binary_expr();

        self.do_binary_expr_typecheck(bin_op, lexpr, rexpr)?;

        expr.expr_type = Some(ExpressionType::from(bin_op));

        Ok(())
    }

    // `MAKE` statements
    fn on_make_global_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.typecheck_var_declare(make_stmt)
    }

    fn on_make_local_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        self.typecheck_var_declare(make_stmt)
    }

    fn on_make_assign_stmt(&mut self, ctx_proc: &str, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let var_id = make_stmt.var_id.unwrap();
        let var: &mut Variable = self.env.symbol_table.get_var_by_id_mut(var_id);

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

    fn on_direct_stmt(&mut self, ctx_proc: &str, direct_stmt: &mut DirectionStmt) -> AstWalkResult {
        let expr_type = &direct_stmt.expr.expr_type;

        if *expr_type != Some(ExpressionType::Int) {
            let expr_str = PrettyPrintAst::pprint_expr(&direct_stmt.expr);
            let err = AstWalkError::NotIntExpr(expr_str);
            return Err(err);
        }

        Ok(())
    }

    fn on_if_stmt(&mut self, ctx_proc: &str, if_stmt: &mut IfStmt) -> AstWalkResult {
        let cond_expr = &if_stmt.cond_expr;

        if cond_expr.expr_type != Some(ExpressionType::Bool) {
            let expr_str = PrettyPrintAst::pprint_expr(cond_expr);
            let err = AstWalkError::NotBooleanExpr(expr_str);
            return Err(err);
        }

        Ok(())
    }

    fn on_repeat_stmt(&mut self, ctx_proc: &str, repeat_stmt: &mut RepeatStmt) -> AstWalkResult {
        let count_expr = &repeat_stmt.count_expr;

        if count_expr.expr_type != Some(ExpressionType::Int) {
            let expr_str = PrettyPrintAst::pprint_expr(count_expr);
            let err = AstWalkError::NotIntExpr(expr_str);
            return Err(err);
        }

        Ok(())
    }

    fn on_ret_stmt(&mut self, ctx_proc: &str, ret_stmt: &mut ReturnStmt) -> AstWalkResult {
        let root_scope_id = 0;

        let symbol = self
            .env
            .symbol_table
            .lookup_recur(root_scope_id, ctx_proc, &SymbolKind::Proc);

        let proc: &Procedure = symbol.unwrap().as_proc();

        let actual_ret_type = if ret_stmt.expr.is_some() {
            let ret_expr = ret_stmt.expr.as_ref().unwrap();
            let ret_expr_type = ret_expr.expr_type.as_ref().unwrap();

            ret_expr_type.clone()
        } else {
            ExpressionType::Unit
        };

        if proc.return_type != actual_ret_type {
            let err = AstWalkError::InvalidReturnType(proc.return_type.clone(), actual_ret_type);
            return Err(err);
        }

        Ok(())
    }
}

impl<'env> AstTypeCheck<'env> {
    fn typecheck_var_declare(&mut self, make_stmt: &mut MakeStmt) -> AstWalkResult {
        let var_id = make_stmt.var_id.unwrap();
        let var: &mut Variable = self.env.symbol_table.get_var_by_id_mut(var_id);

        let expr_type: &ExpressionType = make_stmt.expr.expr_type.as_ref().unwrap();

        if *expr_type == ExpressionType::Unit {
            let err = AstWalkError::VariableTypeMissing(var.name.to_string());
            return Err(err);
        }

        var.var_type = Some(expr_type.to_owned());

        Ok(())
    }

    fn do_binary_expr_typecheck(
        &self,
        bin_op: &BinaryOp,
        lexpr: &Expression,
        rexpr: &Expression,
    ) -> AstWalkResult {
        let ltype = lexpr.expr_type.clone().unwrap();
        let rtype = rexpr.expr_type.clone().unwrap();

        if ltype != rtype {
            let err = AstWalkError::InvalidBinaryOp(bin_op.clone(), ltype, rtype);
            return Err(err);
        }

        assert!(ltype == rtype);

        // if we're here we know that `left expression type == right expression type`
        let expr_type: ExpressionType = ltype;

        match bin_op {
            BinaryOp::Add | BinaryOp::Mul => {
                if expr_type != ExpressionType::Int {
                    let err = AstWalkError::InvalidBinaryOp(
                        bin_op.clone(),
                        expr_type.clone(),
                        expr_type.clone(),
                    );

                    Err(err)
                } else {
                    Ok(())
                }
            }
            BinaryOp::GT | BinaryOp::LT => {
                if expr_type != ExpressionType::Int {
                    let err = AstWalkError::InvalidBinaryOp(
                        bin_op.clone(),
                        expr_type.clone(),
                        expr_type.clone(),
                    );
                    Err(err)
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}
