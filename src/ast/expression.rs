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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(LiteralExpr),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
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
}
