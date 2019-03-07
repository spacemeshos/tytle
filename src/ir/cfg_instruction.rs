use crate::ast::semantic::SymbolId;
use crate::ast::statement::{Command, Direction};
use crate::ir::CfgNodeId;

#[derive(Debug, Clone, PartialEq)]
pub enum CfgInstruction {
    Command(Command),
    Direction(Direction),
    Load(SymbolId),
    Store(SymbolId),
    Bool(bool),
    Int(usize),
    Str(String),
    Add,
    Mul,
    Not,
    And,
    Or,
    GT,
    LT,
}
