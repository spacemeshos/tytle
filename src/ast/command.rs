#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    PenErase,
    ClearScreen,
    SetPenColor,
    SetBackgroundColor,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "PENUP" => Command::PenUp,
            "PENDOWN" =>       Command::PenDown,
            "SHOWTURTLE" =>    Command::ShowTurtle,
            "HIDETURTLE" =>    Command::HideTurtle,
            "PENERASE" =>      Command::PenErase,
            "CLEARSCREEN" =>   Command::ClearScreen,
            "SETPENCOLOR" =>   Command::SetPenColor,
            "SETBACKGROUND" => Command::SetBackgroundColor,
            _ => panic!("Invalid command: {}", s),
        }
    }
}
