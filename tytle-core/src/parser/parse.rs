use crate::ast::Ast;
use crate::lexer::{Location, Token};
use crate::parser::ParseError;

pub type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}
