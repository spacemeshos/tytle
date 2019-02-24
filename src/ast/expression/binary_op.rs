use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
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
}
