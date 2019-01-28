use crate::ir::instruction::Instruction;
use crate::vm::position::Position;
use crate::vm::screen::Screen;
use crate::vm::turtle::Turtle;

pub struct Interpreter {
    turtle: Turtle,
    bytecode: Vec<Instruction>,
    screen: Box<Screen>,
}

impl Interpreter {
    pub fn new(bytecode: Vec<Instruction>, screen: Box<Screen>) -> Self {
        let turtle = Turtle::default();

        Self {
            bytecode,
            turtle,
            screen,
        }
    }

    pub fn set_turtle_color(&mut self, color: (u8, u8, u8)) {
        self.turtle.color = color;
    }

    fn exec_next_inst(&mut self) {
        let inst = self.bytecode.pop();

        if inst.is_none() {
            return;
        }

        let inst: Instruction = inst.unwrap();

        match inst.opcode {
            _ => panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn turtle_starts_at_0_0() {

    }
}

// #[cfg(test)]
// mod tests {
//     use crate::ir::{instruction::Instruction, opcode::Opcode, operand::Operand};
//     use crate::lexer::location::Location;
//     use crate::parser::{simple_parser::SimpleParser, Ast, ParseError, Parser, ParserResult};
//
//     macro_rules! parse_err {
//         ($msg:expr, $line:expr, $col:expr) => {{
//             ParseError::new($msg, Location($line, $col))
//         }};
//     }
//
//     macro_rules! assert_parse_err {
//         ($actual: tt, $msg:expr, $line:expr, $col:expr) => {{
//             let expected = parse_err!($msg, $line, $col);
//             assert_eq!($actual, Err(expected));
//         }};
//     }
//
//     macro_rules! inst {
//         ($opcode:ident $($op_type:ident($op_value:expr)),*) => {{
//             let mut operands = Vec::new();
//
//             $(
//                 let operand =
//                     match stringify!($op_type) {
//                         "Int" => Operand::Int($op_value.to_string()),
//                         _ => unimplemented!()
//                     };
//
//                 operands.push(operand);
//             )*
//
//             Instruction {
//                 opcode: crate::ir::opcode::Opcode::$opcode,
//                 operands: operands,
//             }
//         }};
//     }
//
//     #[test]
//     pub fn empty() {
//         let empty_ast = Ast::default();
//
//         assert_eq!(SimpleParser::parse("").unwrap(), empty_ast);
//     }
//
//     #[test]
//     pub fn forward_with_number_operand() {
//         let ast = SimpleParser::parse("FORWARD 100").unwrap();
//         let insts = vec![inst!(FD Int(100))];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn forward_with_non_number_operand() {
//         let res = SimpleParser::parse("FORWARD ABC");
//         assert_parse_err!(res, "expected a number, received: ABC", 1, 9)
//     }
//
//     #[test]
//     pub fn forward_without_operands() {
//         let res = SimpleParser::parse("FORWARD");
//         assert_parse_err!(res, "missing number", 2, 1);
//     }
//
//     #[test]
//     pub fn forward_with_2_integer_operands() {
//         let res = SimpleParser::parse("FORWARD 100 200");
//         assert_parse_err!(res, "command is too long", 1, 13);
//     }
//
//     #[test]
//     pub fn backward_with_number_operand() {
//         let ast = SimpleParser::parse("BACKWARD 100").unwrap();
//         let insts = vec![inst!(BK Int(100))];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn backward_with_non_number_operand() {
//         let res = SimpleParser::parse("BACKWARD ABC");
//         assert_parse_err!(res, "expected a number, received: ABC", 1, 10);
//     }
//
//     #[test]
//     pub fn right_with_number_operand() {
//         let ast = SimpleParser::parse("RIGHT 100").unwrap();
//         let insts = vec![inst!(RT Int(100))];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn left_with_number_operand() {
//         let ast = SimpleParser::parse("LEFT 100").unwrap();
//         let insts = vec![inst!(LT Int(100))];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn multiple_directions_commands() {
//         let ast =
//             SimpleParser::parse("LEFT 100 \n RIGHT 200 \n FORWARD 300 \n BACKWARD 400 ").unwrap();
//
//         let insts = vec![
//             inst!(LT Int(100)),
//             inst!(RT Int(200)),
//             inst!(FD Int(300)),
//             inst!(BK Int(400)),
//         ];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn pen_up() {
//         let ast = SimpleParser::parse("PENUP").unwrap();
//         let insts = vec![inst!(PU)];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn pen_up_invalid() {
//         let res = SimpleParser::parse("PENUP 100");
//         assert_parse_err!(res, "command is too long", 1, 7);
//     }
//
//     #[test]
//     pub fn pen_down() {
//         let ast = SimpleParser::parse("PENDOWN").unwrap();
//         let insts = vec![inst!(PD)];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn pen_down_invalid() {
//         let res = SimpleParser::parse("PENDOWN 100");
//         assert_parse_err!(res, "command is too long", 1, 9);
//     }
//
//     #[test]
//     pub fn show_turtle() {
//         let ast = SimpleParser::parse("SHOWTURTLE").unwrap();
//         let insts = vec![inst!(ST)];
//
//         assert_eq!(ast.instructions, insts);
//     }
//
//     #[test]
//     pub fn hide_turtle() {
//         let ast = SimpleParser::parse("HIDETURTLE").unwrap();
//         let insts = vec![inst!(HT)];
//
//         assert_eq!(ast.instructions, insts);
//     }
// }
