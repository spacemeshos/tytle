use crate::ast::expression::*;
use crate::ast::semantic::SymbolId;

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

    pub fn with_parentheses(expr_ast: ExpressionAst) -> Self {
        let expr = Self::new(expr_ast);

        let ast = ExpressionAst::Parentheses(Box::new(expr));
        Self::new(ast)
    }

    pub fn adjust_parentheses(expr_ast: ExpressionAst, wrap: bool) -> Self {
        match wrap {
            true => Self::with_parentheses(expr_ast),
            false => Self::new(expr_ast),
        }
    }
}

impl Expression {
    pub fn as_lit_expr_mut(&mut self) -> &mut LiteralExpr {
        match self.expr_ast {
            ExpressionAst::Literal(ref mut expr) => expr,
            _ => panic!("expected a literal expression. got: `{:?}`", self.expr_ast),
        }
    }

    pub fn as_lit_expr(&self) -> &LiteralExpr {
        match &self.expr_ast {
            ExpressionAst::Literal(expr) => expr,
            _ => panic!("expected a literal expression. got: `{:?}`", self.expr_ast),
        }
    }

    pub fn as_not_expr(&self) -> &Expression {
        match &self.expr_ast {
            ExpressionAst::Not(expr) => expr,
            _ => panic!("expected a *not* expression. got: `{:?}`", self.expr_ast),
        }
    }

    pub fn as_parentheses_expr(&self) -> &Expression {
        match &self.expr_ast {
            ExpressionAst::Parentheses(expr) => expr,
            _ => panic!(
                "expected an expression surrounded by parentheses. got: `{:?}`",
                self.expr_ast
            ),
        }
    }

    pub fn as_binary_expr(&self) -> (&BinaryOp, &Expression, &Expression) {
        match &self.expr_ast {
            ExpressionAst::Binary(bin_op, lexpr, rexpr) => (bin_op, lexpr, rexpr),
            _ => panic!("expected a binary expression. got: `{:?}`", self.expr_ast),
        }
    }

    pub fn as_proc_call_expr_mut(
        &mut self,
    ) -> (&mut String, &mut Vec<Expression>, &mut Option<SymbolId>) {
        match &mut self.expr_ast {
            ExpressionAst::ProcCall(proc_name, proc_args_exprs, proc_id) => {
                return (proc_name, proc_args_exprs, proc_id);
            }
            _ => panic!("expected a procedure call"),
        }
    }

    pub fn as_proc_call_expr(&self) -> (&String, &Vec<Expression>, Option<&SymbolId>) {
        match &self.expr_ast {
            ExpressionAst::ProcCall(proc_name, proc_args_exprs, proc_id) => {
                (proc_name, proc_args_exprs, proc_id.as_ref())
            }
            _ => panic!(
                "expected a procedure call expression. got: `{:?}`",
                self.expr_ast
            ),
        }
    }
}
