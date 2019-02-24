use crate::ast::expression::{BinaryOp, ExpressionType};

#[derive(Debug, Clone, PartialEq)]
pub enum AstWalkError {
    DuplicateGlobalVar(String),
    DuplicateProc(String),
    DuplicateProcLocalVar(String),
    DuplicateProcParam(String, String),
    MissingVarDeclaration(String),
    ProcNotAllowedToDeclareGlobals(String),
    LocalsNotAllowedUnderRootScope(String),
    TypeMismatch(ExpressionType, ExpressionType),
    InvalidBinaryOp(BinaryOp, ExpressionType, ExpressionType),
    InvalidProcCallArgsCount(String, usize, usize),
    InvalidProcCallArgType(usize, ExpressionType, ExpressionType),
    VariableTypeMissing(String),
    Custom { message: String },
}

impl AstWalkError {
    pub fn new(message: &str) -> Self {
        AstWalkError::Custom {
            message: message.to_owned(),
        }
    }
}
