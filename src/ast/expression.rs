#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(usize),
    Var(String),
    Str(String),
    Bool(Box<BoolExpression>),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolExpression {
    True,
    False,
    And(Box<BoolExpression>, Box<BoolExpression>),
    Or(Box<BoolExpression>, Box<Expression>),
    EQ(Expression, Expression),
    GT(Expression, Expression),
    LT(Expression, Expression),
    GTE(Expression, Expression),
    LTE(Expression, Expression),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_expr_sanity() {
        let expr1 = Expression::Int(10);
        let expr2 = Expression::Int(10);
        let expr3 = Expression::Int(20);

        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }
}
