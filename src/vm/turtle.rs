use crate::vm::position::Position;
use std::default::Default;

pub struct Turtle {
    pub color: (u8, u8, u8),
    pub position: Position,
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            color: (255, 255, 255),
            position: Position::default(),
        }
    }
}
