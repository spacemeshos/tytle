pub struct Turtle {
    position: (usize, usize),
    visible: bool,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            visible: true,
            position: (0, 0),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn xcor(&self) -> usize {
        self.position.0
    }

    pub fn ycor(&self) -> usize {
        self.position.1
    }
}
