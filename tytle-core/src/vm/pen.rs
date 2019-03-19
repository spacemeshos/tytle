pub enum PenState {
    Up,
    Down,
    Erase,
}

pub struct Pen {
    state: PenState,
    color: (u8, u8, u8),
}

impl Pen {
    pub fn new() -> Self {
        Self {
            state: PenState::Down,
            color: (0, 0, 0),
        }
    }

    pub fn down(&mut self) {
        self.state = PenState::Down;
    }

    pub fn up(&mut self) {
        self.state = PenState::Up;
    }

    pub fn erase(&mut self) {
        self.state = PenState::Erase;
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color
    }

    pub fn get_state(&self) -> &PenState {
        &self.state
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }
}
