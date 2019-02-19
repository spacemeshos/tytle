use crate::ast::expression::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub expr_type: Option<ExpressionType>,
    pub expr_ast: ExpressionAst,
}

impl Expression {
    pub fn new(expr_ast: ExpressionAst) -> Self {
        Self {
            expr_ast,
            expr_type: None,
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
