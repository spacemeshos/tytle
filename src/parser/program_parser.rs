use crate::parser::{Parser, ParserResult};
use crate::ast::program::Program;

struct ProgramParser {

}

impl Parser for ProgramParser {
    fn parse(&mut self, code: &str) -> ParserResult {
        let program = Program { stmts: Vec::new() };
        Ok(program)
    }
}
