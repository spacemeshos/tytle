use crate::ast::Ast;
use crate::lexer::location::Location;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    message: String,
    location: Location,
}

impl ParseError {
    fn new(message: &str, location: Location) -> Self {
        Self {
            location,
            message: message.to_string(),
        }
    }
}

pub type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}

pub mod simple_parser;
