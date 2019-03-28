use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    And,
    Or,
    Add,
    Mul,
    Div,
    GreaterThan,
    LessThan,
}

impl From<&str> for BinaryOp {
    fn from(tok: &str) -> BinaryOp {
        match tok {
            "AND" => BinaryOp::And,
            "OR" => BinaryOp::Or,
            "+" => BinaryOp::Add,
            "*" => BinaryOp::Mul,
            "/" => BinaryOp::Div,
            ">" => BinaryOp::GreaterThan,
            "<" => BinaryOp::LessThan,
            _ => panic!("Invalid binary operator: `{:?}`", tok),
        }
    }
}

impl From<&Token> for BinaryOp {
    fn from(tok: &Token) -> BinaryOp {
        match *tok {
            Token::AND => BinaryOp::And,
            Token::OR => BinaryOp::Or,
            Token::ADD => BinaryOp::Add,
            Token::MUL => BinaryOp::Mul,
            Token::DIV => BinaryOp::Div,
            Token::GT => BinaryOp::GreaterThan,
            Token::LT => BinaryOp::LessThan,
            _ => panic!("Invalid binary operator: `{:?}`", tok),
        }
    }
}

impl ToString for BinaryOp {
    fn to_string(&self) -> String {
        let s = match *self {
            BinaryOp::And => "AND",
            BinaryOp::Or => "OR",
            BinaryOp::Add => "+",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::GreaterThan => ">",
            BinaryOp::LessThan => "<",
        };

        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_op_add() {
        assert_eq!(BinaryOp::from("+"), BinaryOp::Add);
        assert_eq!(BinaryOp::from(&Token::ADD), BinaryOp::Add);
        assert_eq!("+", BinaryOp::Add.to_string());
    }

    #[test]
    fn binary_op_mul() {
        assert_eq!(BinaryOp::from("*"), BinaryOp::Mul);
        assert_eq!(BinaryOp::from(&Token::MUL), BinaryOp::Mul);
        assert_eq!("*", BinaryOp::Mul.to_string());
    }

    #[test]
    fn binary_op_div() {
        assert_eq!(BinaryOp::from("/"), BinaryOp::Div);
        assert_eq!(BinaryOp::from(&Token::DIV), BinaryOp::Div);
        assert_eq!("/", BinaryOp::Div.to_string());
    }

    #[test]
    fn binary_op_gt() {
        assert_eq!(BinaryOp::from(">"), BinaryOp::GreaterThan);
        assert_eq!(BinaryOp::from(&Token::GT), BinaryOp::GreaterThan);
        assert_eq!(">", BinaryOp::GreaterThan.to_string());
    }

    #[test]
    fn binary_op_lt() {
        assert_eq!(BinaryOp::from("<"), BinaryOp::LessThan);
        assert_eq!(BinaryOp::from(&Token::LT), BinaryOp::LessThan);
        assert_eq!("<", BinaryOp::LessThan.to_string());
    }

    #[test]
    fn binary_op_and() {
        assert_eq!(BinaryOp::from("AND"), BinaryOp::And);
        assert_eq!(BinaryOp::from(&Token::AND), BinaryOp::And);
        assert_eq!("AND", BinaryOp::And.to_string());
    }

    #[test]
    fn binary_op_or() {
        assert_eq!(BinaryOp::from("OR"), BinaryOp::Or);
        assert_eq!(BinaryOp::from(&Token::OR), BinaryOp::Or);
        assert_eq!("OR", BinaryOp::Or.to_string());
    }
}
