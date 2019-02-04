#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExpr {
    Int(usize),
    Var(String),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Mul,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(LiteralExpr),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Int,
    Str,
    Bool,
    NotSure,
}

impl ExpressionType {
    pub fn ensure_same(expr: &Expression, expected: ExpressionType, actual: ExpressionType) {
        if expected != actual {
            panic!(format!(
                "expected expression `{:?}` to be of type `{:?}` (actual: `{:?}`",
                expr, expected, actual
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        ExpressionType::ensure_same(&expr, ExpressionType::Str, ExpressionType::Int);
    }
}
