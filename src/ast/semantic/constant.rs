use crate::ast::semantic::PrimitiveType;

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub value: i64,
    pub reference: Option<u64>,
    pub resolved_type: Option<PrimitiveType>,
}

impl Constant {}
