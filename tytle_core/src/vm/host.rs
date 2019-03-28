use crate::ast::statement::{Command, Direction};
use crate::ir::CfgNodeId;

pub trait Host {
    fn exec_cmd(&mut self, cmd: &Command);
    fn exec_direct(&mut self, direct: &Direction, count: isize);
    fn exec_trap(&mut self, node: usize, ip: usize);
    fn exec_print(&mut self, value: isize);
    fn compilation_error(&mut self, error: &str);
}
