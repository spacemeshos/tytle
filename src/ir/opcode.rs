#[derive(Debug, PartialEq)]
pub enum Opcode {
    SOC, // START OF CODE
    EOC, // END OF CODE

    // builtins
    FD, // Forward
    BK, // Backward
    RT, // Right
    LT, // Left
    PU, // PenUp
    PD, // PenDown
    ST, // ShowTurtle
    HT, // HideTurtle

    CALL,
    RETURN,

    JMP,
    BEQ,
    BNE,

    STORE,
    LOAD,
}
