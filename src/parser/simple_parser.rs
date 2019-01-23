use super::super::ir::{instruction::Instruction, opcode::Opcode, operand::Operand};
use crate::lexer::{simple_lexer::SimpleLexer, token::Token, Lexer};
use crate::parser::{Ast, ParseError, Parser, ParserResult};

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

        let mut insts = Vec::new();

        while !eof {
            match lexer.next_token() {
                Token::EOF => eof = true,
                Token::NEWLINE => continue,
                Token::VALUE(value) => Self::parse_token_value(&value, &mut lexer, &mut insts)?,
                _ => unreachable!(),
            }
        }

        let ast = Ast {
            instructions: insts,
        };

        Ok(ast)
    }
}

impl SimpleParser {
    fn parse_token_value(
        value: &str,
        lexer: &mut impl Lexer,
        insts: &mut Vec<Instruction>,
    ) -> Result<(), ParseError> {
        let inst = if Self::is_operandless_cmd(&value) {
            let opcode = Self::translate_opcode(&value);
            Self::expect_end_of_cmd(lexer)?;
            Instruction::build_opcode_instruction(opcode)
        } else if Self::is_direction_cmd(&value) {
            let opcode = Self::translate_opcode(&value);
            Self::parse_direction(lexer, opcode)?
        } else {
            unreachable!("should never get here")
        };

        insts.push(inst);

        Ok(())
    }

    fn parse_direction(lexer: &mut impl Lexer, opcode: Opcode) -> Result<Instruction, ParseError> {
        let num_as_str = Self::expect_number(lexer)?;

        Self::expect_end_of_cmd(lexer)?;

        let inst = Instruction {
            opcode: opcode,
            operands: vec![Operand::Int(num_as_str)],
        };

        Ok(inst)
    }

    fn is_operandless_cmd(val: &str) -> bool {
        match val {
            "PENUP" | "PENDOWN" => true,
            _ => false,
        }
    }

    fn is_direction_cmd(val: &str) -> bool {
        match val {
            "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" => true,
            _ => false,
        }
    }

    fn translate_opcode(val: &str) -> Opcode {
        match val {
            "FORWARD" => Opcode::FD,
            "BACKWARD" => Opcode::BK,
            "RIGHT" => Opcode::RT,
            "LEFT" => Opcode::LT,
            "PENUP" => Opcode::PU,
            "PENDOWN" => Opcode::PD,
            _ => unimplemented!(),
        }
    }

    fn expect_end_of_cmd(lexer: &mut impl Lexer) -> Result<(), ParseError> {
        let token = lexer.next_token();

        match token {
            Token::NEWLINE | Token::EOF => Ok(()),
            _ => Err(ParseError::new("command is too long")),
        }
    }

    fn expect_number(lexer: &mut impl Lexer) -> Result<String, ParseError> {
        let token = lexer.next_token();

        match token {
            Token::VALUE(string) => match string.parse::<isize>() {
                Ok(d) => Ok(string),
                Err(_) => Err(ParseError {
                    message: format!("expected a number, received: {}", string),
                }),
            },
            _ => Err(ParseError::new("missing number")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::{instruction::Instruction, opcode::Opcode, operand::Operand};
    use crate::parser::{simple_parser::SimpleParser, Ast, ParseError, Parser, ParserResult};

    fn parse_err(msg: &str) -> ParseError {
        ParseError::new(msg)
    }

    macro_rules! inst {
        ($opcode:ident $($op_type:ident($op_value:expr)),*) => {{
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
    pub fn forward_with_number_operand() {
        let ast = SimpleParser::parse("FORWARD 100").unwrap();

        let insts = vec![inst!(FD Int(100))];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn forward_with_non_number_operand() {
        let res = SimpleParser::parse("FORWARD ABC");

        assert_eq!(res, Err(parse_err("expected a number, received: ABC")))
    }

    #[test]
    pub fn forward_without_operands() {
        let res = SimpleParser::parse("FORWARD");

        assert_eq!(res, Err(parse_err("missing number")));
    }

    #[test]
    pub fn forward_with_2_integer_operands() {
        let res = SimpleParser::parse("FORWARD 100 200");

        assert_eq!(res, Err(parse_err("command is too long")));
    }

    #[test]
    pub fn backward_with_number_operand() {
        let ast = SimpleParser::parse("BACKWARD 100").unwrap();

        let insts = vec![inst!(BK Int(100))];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn backward_with_non_number_operand() {
        let res = SimpleParser::parse("BACKWARD ABC");

        assert_eq!(res, Err(parse_err("expected a number, received: ABC")));
    }

    #[test]
    pub fn right_with_number_operand() {
        let ast = SimpleParser::parse("RIGHT 100").unwrap();

        let insts = vec![inst!(RT Int(100))];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn left_with_number_operand() {
        let ast = SimpleParser::parse("LEFT 100").unwrap();

        let insts = vec![inst!(LT Int(100))];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn multiple_directions_commands() {
        let ast =
            SimpleParser::parse("LEFT 100 \n RIGHT 200 \n FORWARD 300 \n BACKWARD 400 ").unwrap();

        let insts = vec![
            inst!(LT Int(100)),
            inst!(RT Int(200)),
            inst!(FD Int(300)),
            inst!(BK Int(400)),
        ];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn pen_up() {
        let ast = SimpleParser::parse("PENUP").unwrap();
        let insts = vec![inst!(PU)];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn pen_up_invalid() {
        let res = SimpleParser::parse("PENUP 100");

        assert_eq!(res, Err(ParseError::new("command is too long")))
    }

    #[test]
    pub fn pen_down() {
        let ast = SimpleParser::parse("PENDOWN").unwrap();
        let insts = vec![inst!(PD)];

        assert_eq!(ast.instructions, insts);
    }

    #[test]
    pub fn pen_down_invalid() {
        let res = SimpleParser::parse("PENDOWN 100");
        assert_eq!(res, Err(ParseError::new("command is too long")))
    }
}
