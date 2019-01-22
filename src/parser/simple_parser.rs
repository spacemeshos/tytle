use super::super::ir::instruction::Instruction;
use crate::parser::Ast;
use crate::parser::Parser;
use crate::parser::ParserResult;

pub struct SimpleParser;

impl SimpleParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parser for SimpleParser {
    fn parse(code: &str) -> ParserResult {
        let ast = Ast {
            instructions: Vec::new(),
        };
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::simple_parser::SimpleParser;
    use crate::parser::Ast;
    use crate::parser::Parser;
    use crate::parser::ParserResult;

    #[test]
    pub fn empty() {
        let empty_ast = Ast::default();

        assert_eq!(SimpleParser::parse("").unwrap(), empty_ast);
    }
}
