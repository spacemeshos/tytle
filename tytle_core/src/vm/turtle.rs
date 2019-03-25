use crate::ast::statement::Direction;
use std::cmp;

#[derive(Debug)]
pub struct Turtle {
    position: (isize, isize),
    visible: bool,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            visible: true,
            position: (0, 0),
        }
    }

    pub fn exec_direct(&mut self, direct: &Direction, count: isize) {
        match direct {
            Direction::Forward => self.position.1 += count,
            Direction::Backward => self.position.1 = cmp::max(self.position.1 - count, 0),
            Direction::Right => self.position.0 += count,
            Direction::Left => self.position.0 = cmp::max(self.position.0 - count, 0),
            Direction::SetX => self.position.0 = cmp::max(count, 0),
            Direction::SetY => self.position.1 = cmp::max(count, 0),
            _ => unimplemented!(),
        };
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

    pub fn xcor(&self) -> isize {
        self.position.0
    }

    pub fn ycor(&self) -> isize {
        self.position.1
    }
}
