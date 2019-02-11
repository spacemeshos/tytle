use crate::ast::semantic::PrimitiveType;

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    pub name: String,
    pub reference: Option<u64>,
    pub params_types: Option<Vec<PrimitiveType>>,
    pub return_type: Option<PrimitiveType>,
}
