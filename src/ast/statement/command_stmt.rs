#[derive(Debug, Clone, PartialEq)]
pub enum CommandStmt {
    XCor,
    YCor,
    PenUp,
    PenDown,
    PenErase,
    ShowTurtle,
    HideTurtle,
    Clean,
    ClearScreen,
    SetPenColor,
    SetBackgroundColor,
    Wait,
    Stop,
}

impl CommandStmt {
    pub fn parse(s: &str) -> Option<CommandStmt> {
        match s {
            "XCOR" => Some(CommandStmt::XCor),
            "YCOR" => Some(CommandStmt::YCor),
            "PENUP" => Some(CommandStmt::PenUp),
            "PENDOWN" => Some(CommandStmt::PenDown),
            "SHOWTURTLE" => Some(CommandStmt::ShowTurtle),
            "HIDETURTLE" => Some(CommandStmt::HideTurtle),
            "PENERASE" => Some(CommandStmt::PenErase),
            "CLEAN" => Some(CommandStmt::Clean),
            "CLEARSCREEN" => Some(CommandStmt::ClearScreen),
            "SETPENCOLOR" => Some(CommandStmt::SetPenColor),
            "SETBACKGROUND" => Some(CommandStmt::SetBackgroundColor),
            "WAIT" => Some(CommandStmt::Wait),
            "STOP" => Some(CommandStmt::Stop),
            _ => None,
        }
    }
}
