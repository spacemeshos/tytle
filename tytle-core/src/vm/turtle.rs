use crate::ast::statement::Direction;

#[derive(Debug)]
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

    pub fn exec_direct(&mut self, direct: &Direction, count: usize) {
        match direct {
            Direction::Forward => self.position.1 += count,
            Direction::Backward => self.position.1 -= count,
            Direction::Right => self.position.0 += count,
            Direction::Left => self.position.0 -= count,
            Direction::SetY => self.position.1 = count,
            Direction::SetX => self.position.0 = count,
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
