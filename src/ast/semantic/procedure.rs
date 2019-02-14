use crate::ast::semantic::PrimitiveType;

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    pub name: String,
    pub reference: Option<u64>,
    pub params_types: Option<Vec<PrimitiveType>>,
    pub return_type: Option<PrimitiveType>,
}

impl Procedure {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            reference: None,
            params_types: None,
            return_type: None,
        }
    }
}
