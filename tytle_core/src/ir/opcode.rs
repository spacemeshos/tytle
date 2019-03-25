use crate::ast::expression::BinaryOp;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    SOC, // START OF CODE
    EOC, // END OF CODE

    FD,   // Forward
    BK,   // Backward
    RT,   // Right
    LT,   // Left
    SX,   // SetX
    SY,   // SetY
    GETX, // XCOR
    GETY, // YCOR

    PU, // PenUp
    PD, // PenDown
    PE, // PenErase
    ST, // ShowTurtle
    HT, // HideTurtle

    CLR, // Clean
    CLS, // ClearScreen

    WT,
    STP,

    JMP,
    RET,
    BEQ,
    BNE,

    STORE,
    LOAD,

    PUSH,
    POP,
    DEC,

    ADD,
    MUL,

    AND,
    OR,
    NOT,
    GT,
}

impl From<&BinaryOp> for Opcode {
    fn from(bin_op: &BinaryOp) -> Opcode {
        match bin_op {
            BinaryOp::And => Opcode::AND,
            BinaryOp::Or => Opcode::OR,
            BinaryOp::GT => Opcode::GT,
            BinaryOp::Add => Opcode::ADD,
            BinaryOp::Mul => Opcode::MUL,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_from_bin_op_and() {
        assert_eq!(Opcode::AND, Opcode::from(&BinaryOp::And));
    }

    #[test]
    fn opcode_from_bin_op_or() {
        assert_eq!(Opcode::OR, Opcode::from(&BinaryOp::Or));
    }

    #[test]
    fn opcode_from_bin_op_gt() {
        assert_eq!(Opcode::GT, Opcode::from(&BinaryOp::GT));
    }

    #[test]
    fn opcode_from_bin_op_add() {
        assert_eq!(Opcode::ADD, Opcode::from(&BinaryOp::Add));
    }

    #[test]
    fn opcode_from_bin_op_mul() {
        assert_eq!(Opcode::MUL, Opcode::from(&BinaryOp::Mul));
    }
}
