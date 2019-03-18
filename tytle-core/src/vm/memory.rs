use crate::vm::{Globals, Pen, Turtle};

pub struct Memory {
    pub globals: Globals,
    pub turtle: Turtle,
    pub pen: Pen,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            globals: Globals::new(),
            turtle: Turtle::new(),
            pen: Pen::new(),
        }
    }
}
