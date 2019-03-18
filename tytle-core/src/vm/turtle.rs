use crate::vm::Position;

pub struct Turtle {
    pub color: (u8, u8, u8),
    pub position: Position,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            color: (255, 255, 255),
            position: Position::default(),
        }
    }
}
