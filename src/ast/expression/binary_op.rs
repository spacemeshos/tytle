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
