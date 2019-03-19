use crate::ast::statement::{Command, Direction};

pub trait Host {
    fn exec_cmd(&mut self, cmd: &Command);
    fn exec_direct(&mut self, direct: &Direction, count: usize);
}
