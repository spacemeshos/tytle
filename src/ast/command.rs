#[derive(Debug, Clone, PartialEq)]
pub enum CommandStmt {
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    PenErase,
    ClearScreen,
    SetPenColor,
    SetBackgroundColor,
}

impl From<&str> for CommandStmt {
    fn from(s: &str) -> Self {
        match s {
            "PENUP" => CommandStmt::PenUp,
            "PENDOWN" => CommandStmt::PenDown,
            "SHOWTURTLE" => CommandStmt::ShowTurtle,
            "HIDETURTLE" => CommandStmt::HideTurtle,
            "PENERASE" => CommandStmt::PenErase,
            "CLEARSCREEN" => CommandStmt::ClearScreen,
            "SETPENCOLOR" => CommandStmt::SetPenColor,
            "SETBACKGROUND" => CommandStmt::SetBackgroundColor,
            _ => panic!("Invalid command: {}", s),
        }
    }
}
