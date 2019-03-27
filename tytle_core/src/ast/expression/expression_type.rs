use crate::ast::expression::BinaryOp;

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Int,
    Str,
    Bool,
    Unit,
}

impl From<&str> for ExpressionType {
    fn from(type_str: &str) -> ExpressionType {
        match type_str {
            "INT" => ExpressionType::Int,
            "STR" => ExpressionType::Str,
            "BOOL" => ExpressionType::Bool,
            "" | "UNIT" => ExpressionType::Unit,
            _ => panic!(format!(
                "can't convert string `{}` to an expression type",
                type_str
            )),
        }
    }
}

impl From<&BinaryOp> for ExpressionType {
    fn from(bin_op: &BinaryOp) -> ExpressionType {
        match bin_op {
            BinaryOp::Add | BinaryOp::Mul | BinaryOp::Div => ExpressionType::Int,
            BinaryOp::GT | BinaryOp::LT => ExpressionType::Bool,
            _ => panic!(format!(
                "can't convert binary operator `{:?}` to an expression type",
                bin_op
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_expr_type() {
        assert_eq!(ExpressionType::from("STR"), ExpressionType::Str);
    }

    #[test]
    fn int_to_expr_type() {
        assert_eq!(ExpressionType::from("INT"), ExpressionType::Int);
    }

    #[test]
    fn bool_to_expr_type() {
        assert_eq!(ExpressionType::from("BOOL"), ExpressionType::Bool);
    }

    #[test]
    fn unit_to_expr_type() {
        assert_eq!(ExpressionType::from(""), ExpressionType::Unit);
        assert_eq!(ExpressionType::from("UNIT"), ExpressionType::Unit);
    }

    #[test]
    #[should_panic(expected = "can't convert string `str` to an expression type")]
    fn invalid_str_to_expr_type_should_panic() {
        ExpressionType::from("str");
    }

    #[test]
    fn binary_op_add_to_expr_type_int() {
        assert_eq!(ExpressionType::from(&BinaryOp::Add), ExpressionType::Int);
    }

    #[test]
    fn binary_op_mul_to_expr_type_int() {
        assert_eq!(ExpressionType::from(&BinaryOp::Mul), ExpressionType::Int);
    }

    #[test]
    fn binary_op_div_to_expr_type_int() {
        assert_eq!(ExpressionType::from(&BinaryOp::Div), ExpressionType::Int);
    }

    #[test]
    fn binary_op_gt_to_expr_type_bool() {
        assert_eq!(ExpressionType::from(&BinaryOp::GT), ExpressionType::Bool);
    }

    #[test]
    fn binary_op_lt_to_expr_type_bool() {
        assert_eq!(ExpressionType::from(&BinaryOp::LT), ExpressionType::Bool);
    }
}
