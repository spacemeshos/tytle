use crate::ast::Ast;
use crate::lexer::{Location, Token};

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

pub type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}
