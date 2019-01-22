use crate::ir::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    operands: Vec<String>,
}
