use crate::ir::instruction::Instruction;
use crate::lexer::location::Location;
use std::default::Default;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub instructions: Vec<Instruction>,
}

impl Default for Ast {
    fn default() -> Self {
        Ast {
            instructions: Vec::new(),
        }
    }
}

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

type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(code: &str) -> ParserResult;
}

mod simple_parser;
