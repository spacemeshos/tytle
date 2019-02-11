use crate::ast::expression::ExpressionType;
use std::convert::Into;

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveType {
    Int,
    Str,
}
