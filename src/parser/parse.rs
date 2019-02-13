use crate::ast::Ast;
use crate::lexer::Location;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    message: String,
    location: Location,
}

pub type ParserResult = Result<Ast, ParseError>;

impl ParseError {
    fn new(message: &str, location: Location) -> Self {
        Self {
            location,
            message: message.to_string(),
        }
    }
}

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}
