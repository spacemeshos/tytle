use crate::ast::expression::ExpressionType;

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    pub name: String,
    pub params_types: Vec<ExpressionType>,
    pub return_type: ExpressionType,
}

impl Procedure {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            params_types: Vec::new(),
            return_type: ExpressionType::Unit,
        }
    }
}
