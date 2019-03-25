#[derive(Debug, Clone, PartialEq)]
pub enum Command {
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
    Trap,
}

impl Command {
    pub fn parse(s: &str) -> Option<Command> {
        match s {
            "XCOR" => Some(Command::XCor),
            "YCOR" => Some(Command::YCor),
            "PENUP" => Some(Command::PenUp),
            "PENDOWN" => Some(Command::PenDown),
            "SHOWTURTLE" => Some(Command::ShowTurtle),
            "HIDETURTLE" => Some(Command::HideTurtle),
            "PENERASE" => Some(Command::PenErase),
            "CLEAN" => Some(Command::Clean),
            "CLEARSCREEN" => Some(Command::ClearScreen),
            "SETPENCOLOR" => Some(Command::SetPenColor),
            "SETBACKGROUND" => Some(Command::SetBackgroundColor),
            "WAIT" => Some(Command::Wait),
            "STOP" => Some(Command::Stop),
            _ => None,
        }
    }
}
