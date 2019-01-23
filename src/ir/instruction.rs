use crate::ir::opcode::Opcode;
use crate::ir::operand::Operand;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

impl Instruction {
    pub fn build_opcode_instruction(opcode: Opcode) -> Self {
        Self {
            opcode: opcode,
            operands: Vec::new(),
        }
    }
}
