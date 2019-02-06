#[derive(Debug, PartialEq)]
pub enum Opcode {
    SOC, // START OF CODE
    EOC, // END OF CODE

    FD, // Forward
    BK, // Backward
    RT, // Right
    LT, // Left
    PU, // PenUp
    PD, // PenDown

    ST, // ShowTurtle
    HT, // HideTurtle
}
