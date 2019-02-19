#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionType {
    Int,
    Str,
    Bool,
}

impl From<&str> for ExpressionType {
    fn from(type_str: &str) -> ExpressionType {
        match type_str {
            "INT" => ExpressionType::Int,
            "STR" => ExpressionType::Str,
            "BOOL" => ExpressionType::Bool,
            _ => panic!(format!(
                "can't convert string `{}` to an expression type",
                type_str
            )),
        }
    }
}
