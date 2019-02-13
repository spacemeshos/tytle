use crate::ast::Ast;
use crate::lexer::{Location, Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NewLineExpected,
    IdentifierExpected,
    InvalidIdentifierDeclaration(String),
    UnexpectedToken { expected: Token, actual: Token },
    UnexpectedKeyword { keyword: String },
    InvalidProcParam { param: String },
    Syntax { message: String },
}

pub type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}
