use crate::ast::semantic::SymbolId;
use crate::ast::statement::{Command, Direction};
use crate::ir::CfgNodeId;

#[derive(Debug, Clone, PartialEq)]
pub enum CfgInstruction {
    Command(Command),
    Direction(Direction),
    Load(SymbolId),
    Store(SymbolId),
    Call(CfgNodeId),
    Bool(bool),
    Int(isize),
    Str(String),
    Return,
    Trap,
    Print,
    EOC,
    Add,
    Mul,
    Not,
    And,
    Or,
    GT,
    LT,
}
