use crate::ast::Ast;
use crate::lexer::{Location, Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NewLineExpected,
    IdentifierExpected,
    UnexpectedToken { expected: Token, actual: Token },
    UnknownToken(Token),
    InvalidProcParam { param: String },
    Custom { message: String },
}

pub type ParserResult = Result<Ast, ParseError>;

impl ParseError {
    fn new(message: &str, _location: Location) -> Self {
        ParseError::Custom {
            message: message.to_string(),
        }
    }
}

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}
