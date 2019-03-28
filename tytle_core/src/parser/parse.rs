use crate::ast::Ast;
use crate::parser::ParseError;

pub type ParserResult = Result<Ast, ParseError>;

pub trait Parser {
    fn parse(&mut self, code: &str) -> ParserResult;
}
