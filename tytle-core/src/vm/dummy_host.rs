use crate::ast::statement::{Command, Direction};
use crate::vm::{Host, Pen, Turtle};

#[derive(Debug)]
pub struct DummyHost {
    pen: Pen,
    turtle: Turtle,
}

impl Host for DummyHost {
    fn exec_cmd(&mut self, cmd: &Command) {}

    fn exec_direct(&mut self, direct: &Direction, count: usize) {
        self.turtle.exec_direct(direct, count);
    }
}

impl DummyHost {
    pub fn new() -> Self {
        Self {
            pen: Pen::new(),
            turtle: Turtle::new(),
        }
    }

    pub fn xcor(&self) -> isize {
        self.turtle.xcor()
    }

    pub fn ycor(&self) -> isize {
        self.turtle.ycor()
    }

    pub fn xycors(&self) -> (isize, isize) {
        let x = self.xcor();
        let y = self.ycor();

        (x, y)
    }

    pub fn pen_up(&mut self) {
        self.pen.up()
    }

    pub fn pen_down(&mut self) {
        self.pen.down()
    }

    pub fn pen_erase(&mut self) {
        self.pen.erase()
    }

    pub fn show_turtle(&mut self) {
        self.turtle.show();
    }

    pub fn hide_turtle(&mut self) {
        self.turtle.hide();
    }

    pub fn clean(&mut self) {
        unimplemented!()
    }

    pub fn clear_screen(&mut self) {
        unimplemented!()
    }

    pub fn set_pen_color(&mut self, color: (u8, u8, u8)) {
        self.pen.set_color(color);
    }

    pub fn set_bg_color(&mut self) {
        unimplemented!()
    }

    pub fn wait(&mut self) {}

    pub fn stop(&mut self) {
        unimplemented!()
    }

    pub fn get_turtle(&self) -> Turtle {
        unimplemented!();
    }

    pub fn get_pen(&self) -> Pen {
        unimplemented!();
    }
}
