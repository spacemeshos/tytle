use crate::ir::instruction::Instruction;
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
}

impl ParseError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(code: &str) -> ParserResult;
}

mod simple_parser;
