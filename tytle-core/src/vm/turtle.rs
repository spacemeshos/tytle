pub struct Turtle {
    pub color: (u8, u8, u8),
    pub position: (usize, usize),
    pub visible: bool,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            color: (255, 255, 255),
            visible: true,
            position: (0, 0),
        }
    }
}
