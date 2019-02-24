use crate::ast::expression::ExpressionType;

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    pub name: String,
    pub reference: Option<u64>,
    pub params_types: Vec<ExpressionType>,
    pub return_type: ExpressionType,
}

impl Procedure {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            reference: None,
            params_types: Vec::new(),
            return_type: ExpressionType::Unit,
        }
    }
}
