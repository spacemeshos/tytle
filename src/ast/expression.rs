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

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionAst {
    Literal(LiteralExpr),
    ProcCall(String, Vec<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
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

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionType {
    Int,
    Str,
    Bool,
}

impl From<&str> for ExpressionType {
    fn from(type_str: &str) -> ExpressionType {
        match type_str {
            "INT" => ExpressionType::Int,
            "STR" => ExpressionType::Str,
            "BOOL" => ExpressionType::Bool,
            _ => panic!(format!(
                "can't convert string `{}` to an expression type",
                type_str
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExpr {
    Bool(bool),
    Int(usize),
    Var(String),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Mul,
    // GT,
    // LT,
    // GTE,
    // LTE,
    // EQEQ,
}

impl From<&str> for BinaryOp {
    fn from(s: &str) -> BinaryOp {
        match s {
            "+" => BinaryOp::Add,
            "*" => BinaryOp::Mul,
            _ => panic!("Invalid binary operator: `{}`", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_expr_equal(expr: &Expression, expected: ExpressionType, actual: ExpressionType) {
        if expected != actual {
            panic!(
                "expected expression `{:?}` to be of type `{:?}` (actual: `{:?}`",
                expr, expected, actual
            );
        }
    }

    #[test]
    fn expr_literal_sanity() {
        let expr1 = ExpressionAst::Literal(LiteralExpr::Int(10));
        let expr2 = ExpressionAst::Literal(LiteralExpr::Int(10));
        let expr3 = ExpressionAst::Literal(LiteralExpr::Int(20));

        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    #[should_panic]
    fn expr_type_equality_sanity() {
        let ast = ExpressionAst::Literal(LiteralExpr::Int(10));

        assert_expr_equal(
            &Expression::new(ast),
            ExpressionType::Str,
            ExpressionType::Int,
        );
    }
}
