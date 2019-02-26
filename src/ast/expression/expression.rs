use crate::ast::expression::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub expr_type: Option<ExpressionType>,
    pub expr_ast: ExpressionAst,
    pub surrounded_by_parens: bool,
}

impl Expression {
    pub fn new(expr_ast: ExpressionAst) -> Self {
        Self {
            expr_ast,
            expr_type: None,
            surrounded_by_parens: false,
        }
    }
}

impl Expression {
    pub fn as_lit_expr(&self) -> &LiteralExpr {
        match &self.expr_ast {
            ExpressionAst::Literal(lit_expr) => lit_expr,
            _ => panic!("expected a literal expression. got: `{:?}`", self.expr_ast),
        }
    }

    pub fn as_not_expr(&self) -> &Expression {
        match &self.expr_ast {
            ExpressionAst::Not(inner_expr) => inner_expr,
            _ => panic!("expected a not expression. got: `{:?}`", self.expr_ast),
        }
    }

    pub fn as_binary_expr(&self) -> (&BinaryOp, &Expression, &Expression) {
        match &self.expr_ast {
            ExpressionAst::Binary(bin_op, lexpr, rexpr) => (bin_op, lexpr, rexpr),
            _ => panic!("expected a binary expression. got: `{:?}`", self.expr_ast),
        }
    }

    pub fn as_proc_call_expr(&self) -> (&String, &Vec<Expression>) {
        match &self.expr_ast {
            ExpressionAst::ProcCall(proc_name, proc_args_exprs) => (proc_name, proc_args_exprs),
            _ => panic!(
                "expected a procedure call expression. got: `{:?}`",
                self.expr_ast
            ),
        }
    }
}
