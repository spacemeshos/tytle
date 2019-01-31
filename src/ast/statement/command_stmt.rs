#[derive(Debug, Clone, PartialEq)]
pub enum CommandStmt {
    XCor,
    YCor,
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    PenErase,
    Clean,
    ClearScreen,
    SetPenColor,
    SetBackgroundColor,
    Wait,
    Stop,
}

impl From<&str> for CommandStmt {
    fn from(s: &str) -> Self {
        match s {
            "XCOR" => CommandStmt::XCor,
            "YCOR" => CommandStmt::YCor,
            "PENUP" => CommandStmt::PenUp,
            "PENDOWN" => CommandStmt::PenDown,
            "SHOWTURTLE" => CommandStmt::ShowTurtle,
            "HIDETURTLE" => CommandStmt::HideTurtle,
            "PENERASE" => CommandStmt::PenErase,
            "CLEAN" => CommandStmt::Clean,
            "CLEARSCREEN" => CommandStmt::ClearScreen,
            "SETPENCOLOR" => CommandStmt::SetPenColor,
            "SETBACKGROUND" => CommandStmt::SetBackgroundColor,
            "WAIT" => CommandStmt::Wait,
            "STOP" => CommandStmt::Stop,
            _ => panic!("Invalid command: {}", s),
        }
    }
}
