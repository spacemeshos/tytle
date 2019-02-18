use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingColon,
    NewLineExpected,
    IdentifierExpected,
    MissingProcReturnType,
    InvalidDataType(String),
    InvalidIdentifierDeclaration(String),
    UnexpectedToken { expected: Token, actual: Token },
    UnexpectedKeyword { keyword: String },
    Syntax { message: String },
}
