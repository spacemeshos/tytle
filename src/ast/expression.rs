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
    GT,
    LT,
    GTE,
    LTE,
    EQEQ,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(LiteralExpr),
    ProcCall(String, Vec<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Int,
    Str,
    Bool,
}

impl From<&str> for BinaryOp {
    fn from(s: &str) -> BinaryOp {
        match s {
            "+" => BinaryOp::Add,
            "*" => BinaryOp::Mul,
            _ => panic!("Invalid binary operator: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_expr_equal(expr: &Expression, expected: ExpressionType, actual: ExpressionType) {
        if expected != actual {
            panic!(format!(
                "expected expression `{:?}` to be of type `{:?}` (actual: `{:?}`",
                expr, expected, actual
            ));
        }
    }

    #[test]
    fn int_expr_sanity() {
        let expr1 = Expression::Literal(LiteralExpr::Int(10));
        let expr2 = Expression::Literal(LiteralExpr::Int(10));
        let expr3 = Expression::Literal(LiteralExpr::Int(20));

        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    #[should_panic(expected = "expected expression `Literal(Int(10))` to be of type `Str`")]
    fn expr_type_ensure_same_mismatch() {
        let expr = Expression::Literal(LiteralExpr::Int(10));

        assert_expr_equal(&expr, ExpressionType::Str, ExpressionType::Int);
    }
}
