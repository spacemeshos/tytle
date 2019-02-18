use crate::ast::Ast;
use crate::parser::ParseError;
use crate::lexer::{Location, Token};

pub type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}
