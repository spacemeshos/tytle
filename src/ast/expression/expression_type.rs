#[derive(Debug, PartialEq, Clone)]
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
            "" => ExpressionType::Unit,
            _ => panic!(format!(
                "can't convert string `{}` to an expression type",
                type_str
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_expr_type() {
        assert_eq!(ExpressionType::from("STR"), ExpressionType::Str);
    }

    #[test]
    fn int_expr_type() {
        assert_eq!(ExpressionType::from("INT"), ExpressionType::Int);
    }

    #[test]
    fn bool_expr_type() {
        assert_eq!(ExpressionType::from("BOOL"), ExpressionType::Bool);
    }

    #[test]
    fn unit_expr_type() {
        assert_eq!(ExpressionType::from(""), ExpressionType::Unit);
    }

    #[test]
    #[should_panic(expected = "can't convert string `str` to an expression type")]
    fn invalid_expr_type() {
        ExpressionType::from("str");
    }
}
