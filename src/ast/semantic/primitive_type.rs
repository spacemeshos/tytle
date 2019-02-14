use crate::ast::expression::ExpressionType;

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveType {
    Int,
    Str,
    Bool,
}
