use crate::ir::CfgInstruction;
use crate::vm::*;

pub struct Interpreter {
    turtle: Turtle,
    bytecode: Vec<CfgInstruction>,
    call_stack: CallStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            turtle: Turtle::new(),
            call_stack: CallStack::new(),
        }
    }
}
