use crate::ast::expression::{BinaryOp, ExpressionType};

#[derive(Debug, Clone, PartialEq)]
pub enum AstWalkError {
    DuplicateGlobalVar(String),
    DuplicateProc(String),
    DuplicateProcLocalVar(String),
    DuplicateProcParam(String, String),
    MissingVarDeclaration(String),
    ProcNotAllowedToDeclareGlobals(String),
    InvalidReturnType(ExpressionType, ExpressionType),
    LocalsNotAllowedUnderRootScope(String),
    TypeMismatch(ExpressionType, ExpressionType),
    InvalidBinaryOp(BinaryOp, ExpressionType, ExpressionType),
    InvalidProcCallArgsCount(String, usize, usize),
    InvalidProcCallArgType(usize, ExpressionType, ExpressionType),
    VariableTypeMissing(String),
    NotBooleanExpr(String),
    NotIntExpr(String),
}

impl ToString for AstWalkError {
    fn to_string(&self) -> String {
        match self {
            AstWalkError::DuplicateGlobalVar(var) => format!("Duplicate global var: `{}`", var),
            AstWalkError::DuplicateProc(proc) => format!("Duplicate procedure: `{}`", proc),
            AstWalkError::DuplicateProcLocalVar(local) => {
                format!("Duplicate procedure local: `{}`", local)
            }
            AstWalkError::DuplicateProcParam(proc, param) => format!(
                "Duplicate procedure param: `{}` (procedure: `{}`)",
                param, proc
            ),
            AstWalkError::MissingVarDeclaration(var) => {
                format!("Missing variable declaration for `{}`", var)
            }
            AstWalkError::ProcNotAllowedToDeclareGlobals(proc) => format!(
                "Procedure not allowed to declare globals (procedure `{}`)",
                proc
            ),
            AstWalkError::InvalidReturnType(expected, actual) => format!(
                "Invalid return type. expected: `{}`, actual: `{}`",
                expected.to_string(),
                actual.to_string()
            ),
            AstWalkError::LocalsNotAllowedUnderRootScope(var) => format!(
                "Local aren't allowed under the main procedure (variable: `{}`)",
                var
            ),
            AstWalkError::TypeMismatch(expected, actual) =>
                format!("Type mismatch. expected: `{}`, actual: `{}`", expected.to_string(), actual.to_string()),
            AstWalkError::InvalidBinaryOp(bin_op, ltype, rtype) =>
                format!("Invalid binary operator `{}`(left expression-type: `{}`, right expression-type: `{}`", bin_op.to_string(), ltype.to_string(), rtype.to_string()),
            AstWalkError::InvalidProcCallArgsCount(proc, expected, actual) => {
                format!("Prcedure call wrong number of arguments for `{}` (expected: {}, actual: {})", proc, expected, actual)
            },
            AstWalkError::VariableTypeMissing(var) => format!("Missing type for variable: `{}`", var),
            AstWalkError::NotBooleanExpr(expr) => format!("Expression `{}` isn't a Boolean expression", expr),
            AstWalkError::NotIntExpr(expr) => format!("Expression `{}` isn't an Integer expression", expr),
            AstWalkError::InvalidProcCallArgType(arg_index, expected, actual) =>
                format!("expected the {} argument to be `{}` (actual: `{}`)", self.indexify_arg(*arg_index), expected.to_string(), actual.to_string())
        }
    }
}

impl AstWalkError {
    fn indexify_arg(&self, index: usize) -> String {
        match index {
            1 => "first".to_string(),
            2 => "second".to_string(),
            3 => "third".to_string(),
            4 => "fourth".to_string(),
            _ => format!("{}-ith", index),
        }
    }
}
