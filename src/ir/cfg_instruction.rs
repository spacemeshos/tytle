use crate::ast::expression::Expression;
use crate::ast::statement::{Command, Direction};
use crate::ir::{CfgNodeId, VarRef};

#[derive(Debug, Clone, PartialEq)]
pub enum CfgInstruction {
    Jump(CfgNodeId),
    Command(Command),
    Direction(Direction, Expression),
    Assign(VarRef, Expression),
}
