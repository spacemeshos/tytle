use crate::ir::opcode::Opcode;
use crate::ir::operand::Operand;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

impl From<Opcode> for Instruction {
    fn from(opcode: Opcode) -> Self {
        Self {
            opcode: opcode,
            operands: Vec::new(),
        }
    }
}
