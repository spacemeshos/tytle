use super::super::ir::instruction::Instruction;
use super::super::ir::opcode::Opcode;
use super::super::ir::operand::Operand;
use crate::lexer::simple_lexer::SimpleLexer;
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::parser::Ast;
use crate::parser::ParseError;
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
        let mut lexer = SimpleLexer::new(code);
        let mut eof = false;

        let mut insts = Vec::<Instruction>::new();

        while !eof {
            match lexer.next_token() {
                Token::EOF => eof = true,
                Token::VALUE(value) => match value.as_str() {
                    "FORWARD" => {
                        let fd_inst = Self::parse_forward(&mut lexer)?;
                        insts.push(fd_inst);
                    }
                    _ => unimplemented!(),
                },
            }
        }

        let ast = Ast {
            instructions: insts,
        };

        Ok(ast)
    }
}

impl SimpleParser {
    fn parse_forward(lexer: &mut SimpleLexer) -> Result<Instruction, ParseError> {
        let num_as_str = Self::expect_number(lexer)?;

        let inst = Instruction {
            opcode: Opcode::FD,
            operands: vec![Operand::Int(num_as_str)],
        };

        Ok(inst)
    }

    fn expect_number(lexer: &mut SimpleLexer) -> Result<String, ParseError> {
        let token = lexer.next_token();

        match token {
            Token::EOF => Err(ParseError {
                message: "missing number".to_string(),
            }),
            Token::VALUE(string) => match string.parse::<isize>() {
                Ok(d) => Ok(string),
                Err(_) => Err(ParseError {
                    message: format!("expected a number, received: {}", string),
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::instruction::Instruction;
    use crate::ir::opcode::Opcode;
    use crate::ir::operand::Operand;
    use crate::parser::simple_parser::SimpleParser;
    use crate::parser::Ast;
    use crate::parser::ParseError;
    use crate::parser::Parser;
    use crate::parser::ParserResult;

    macro_rules! inst {
        ($opcode:ident $($op_type:ident[$op_value:expr]),*) => {{
            let mut operands = Vec::new();

            $(
                let operand =
                    match stringify!($op_type) {
                        "Int" => Operand::Int($op_value.to_string()),
                        _ => unimplemented!()
                    };

                operands.push(operand);
            )*

            Instruction {
                opcode: crate::ir::opcode::Opcode::$opcode,
                operands: operands,
            }
        }};
    }

    #[test]
    pub fn empty() {
        let empty_ast = Ast::default();

        assert_eq!(SimpleParser::parse("").unwrap(), empty_ast);
    }

    #[test]
    pub fn valid_forward() {
        let ast = SimpleParser::parse("FORWARD 100").unwrap();

        let insts = vec![inst!(FD Int[100])];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn forward_without_operands() {
        let res = SimpleParser::parse("FORWARD");

        assert_eq!(
            res,
            Err(ParseError {
                message: "missing number".to_string()
            })
        );
    }
}
