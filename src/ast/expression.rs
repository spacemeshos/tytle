#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(usize),
    Var(String),
    Str(String),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShowExpr {
    pub varname: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputExpr {}

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
