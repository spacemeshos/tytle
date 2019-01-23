use crate::ir::instruction::Instruction;
use crate::vm::position::Position;
use crate::vm::screen::Screen;
use crate::vm::turtle::Turtle;

pub struct Interpreter {
    turtle: Turtle,
    bytecode: Vec<Instruction>,
    screen: Box<Screen>,
}

impl Interpreter {
    pub fn new(bytecode: Vec<Instruction>, screen: Box<Screen>) -> Self {
        let turtle = Turtle::default();

        Self {
            bytecode,
            turtle,
            screen,
        }
    }

    pub fn set_turtle_color(&mut self, color: (u8, u8, u8)) {
        self.turtle.color = color;
    }

    fn exec_next_inst(&mut self) {
        let inst = self.bytecode.pop();

        if inst.is_none() {
            return;
        }

        let inst = inst.unwrap();
    }
}
