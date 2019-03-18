pub enum PenState {
    Up,
    Down,
    Erase,
}

pub struct Pen {
    pub state: PenState,
    pub color: (u8, u8, u8),
}

impl Pen {
    pub fn new() -> Self {
        Self {
            state: PenState::Down,
            color: (0, 0, 0),
        }
    }
}
