use crate::ast::expression::{BinaryOp, Expression, ExpressionType, LiteralExpr};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionAst {
    Literal(LiteralExpr),
    ProcCall(String, Vec<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
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
