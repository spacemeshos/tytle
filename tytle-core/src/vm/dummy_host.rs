use crate::ast::statement::{Command, Direction};
use crate::vm::{Host, Pen, Turtle};
use std::cell::RefCell;

#[derive(Debug)]
pub struct DummyHost {
    pen: Pen,
    turtle: Turtle,
    log: RefCell<Vec<String>>,
}

impl Host for DummyHost {
    fn exec_cmd(&mut self, cmd: &Command) {
        match cmd {
            Command::XCor => self.xcor(),
            Command::YCor => self.ycor(),
            Command::PenUp => self.pen_up(),
            Command::PenErase => self.pen_erase(),
            Command::Clean => self.clean(),
            Command::ClearScreen => self.clear_screen(),
            Command::ShowTurtle => self.show_turtle(),
            Command::HideTurtle => self.hide_turtle(),
            _ => unimplemented!()
        };
    }

    fn exec_direct(&mut self, direct: &Direction, count: isize) {
        self.turtle.exec_direct(direct, count);
    }
}

impl DummyHost {
    pub fn new() -> Self {
        Self {
            pen: Pen::new(),
            turtle: Turtle::new(),
            log: RefCell::new(Vec::new()),
        }
    }

    pub fn xcor(&self)  {
        let x = self.turtle.xcor();

        let line = format!("XCOR = {}", x);
        self.append_log(line);
    }

    pub fn ycor(&self) {
        let y = self.turtle.ycor();

        let line = format!("YCOR = {}", y);
        self.append_log(line);
    }

    pub fn xycors(&self) -> (isize, isize) {
        let x = self.turtle.xcor();
        let y = self.turtle.ycor();

        let line = format!("XYCORS = ({}, {})", x, y);
        self.append_log(line);

        (x, y)
    }

    pub fn pen_up(&mut self) {
        self.append_log("PENUP".to_string());
        self.pen.up()
    }

    pub fn pen_down(&mut self) {
        self.append_log("PENDOWN".to_string());
        self.pen.down()
    }

    pub fn pen_erase(&mut self) {
        self.append_log("PENERASE".to_string());
        self.pen.erase()
    }

    pub fn show_turtle(&mut self) {
        self.append_log("SHOWTURTLE".to_string());
        self.turtle.show();
    }

    pub fn hide_turtle(&mut self) {
        self.append_log("HIDETURTLE".to_string());
        self.turtle.hide();
    }

    pub fn clean(&mut self) {
        self.append_log("CLEAN".to_string());
    }

    pub fn clear_screen(&mut self) {
        self.append_log("CLEARSCREEN".to_string());
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

    pub fn get_turtle(&self) -> &Turtle {
        &self.turtle
    }

    pub fn get_pen(&self) -> &Pen {
        &self.pen
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }

    fn append_log(&self, line: String) {
        self.log.borrow_mut().push(line);
    }
}
