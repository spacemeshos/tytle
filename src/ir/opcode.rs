#[derive(Debug, PartialEq)]
pub enum Opcode {
    FD,  // Forward
    BK,  // Backward
    RT,  // Right
    LT,  // Left
    PU,  // PenUp
    PD,  // PenDown
    HT,  // HideTurtle
    ST,  // ShowTurtle
    PE,  // PenErase
    RPT, // Repeat
    CS,  // ClearScreen

    XCOR,
    YCOR,
    SETPOS,
    DISTANCE,

    SETPEN, // SetPen
}
