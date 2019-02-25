use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    And,
    Or,
    Not,

    Add,
    Mul,

    GT,
    LT,
    // GTE,
    // LTE,
    // EQEQ,
}

impl From<&str> for BinaryOp {
    fn from(tok: &str) -> BinaryOp {
        match tok {
            "AND" => BinaryOp::And,
            "OR" => BinaryOp::Or,
            "NOT" => BinaryOp::Not,
            "+" => BinaryOp::Add,
            "*" => BinaryOp::Mul,
            ">" => BinaryOp::GT,
            "<" => BinaryOp::LT,
            _ => panic!("Invalid binary operator: `{:?}`", tok),
        }
    }
}

impl From<&Token> for BinaryOp {
    fn from(tok: &Token) -> BinaryOp {
        match *tok {
            Token::AND => BinaryOp::And,
            Token::OR => BinaryOp::Or,
            Token::NOT => BinaryOp::Not,
            Token::ADD => BinaryOp::Add,
            Token::MUL => BinaryOp::Mul,
            Token::GT => BinaryOp::GT,
            Token::LT => BinaryOp::LT,
            _ => panic!("Invalid binary operator: `{:?}`", tok),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_op_add() {
        assert_eq!(BinaryOp::from("+"), BinaryOp::Add);
        assert_eq!(BinaryOp::from(&Token::ADD), BinaryOp::Add);
    }

    #[test]
    fn binary_op_mul() {
        assert_eq!(BinaryOp::from("*"), BinaryOp::Mul);
        assert_eq!(BinaryOp::from(&Token::MUL), BinaryOp::Mul);
    }

    #[test]
    fn binary_op_gt() {
        assert_eq!(BinaryOp::from(">"), BinaryOp::GT);
        assert_eq!(BinaryOp::from(&Token::GT), BinaryOp::GT);
    }

    #[test]
    fn binary_op_lt() {
        assert_eq!(BinaryOp::from("<"), BinaryOp::LT);
        assert_eq!(BinaryOp::from(&Token::LT), BinaryOp::LT);
    }

    #[test]
    fn binary_op_and() {
        assert_eq!(BinaryOp::from("AND"), BinaryOp::And);
        assert_eq!(BinaryOp::from(&Token::AND), BinaryOp::And);
    }

    #[test]
    fn binary_op_or() {
        assert_eq!(BinaryOp::from("OR"), BinaryOp::Or);
        assert_eq!(BinaryOp::from(&Token::OR), BinaryOp::Or);
    }

    #[test]
    fn binary_op_not() {
        assert_eq!(BinaryOp::from("NOT"), BinaryOp::Not);
        assert_eq!(BinaryOp::from(&Token::NOT), BinaryOp::Not);
    }
}
