use crate::ir::opcode::Opcode;
use crate::ir::operand::Operand;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}
