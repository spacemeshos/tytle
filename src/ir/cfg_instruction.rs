use crate::ast::semantic::SymbolId;
use crate::ast::statement::{Command, Direction};

#[derive(Debug, Clone, PartialEq)]
pub enum CfgInstruction {
    Command(Command),
    Direction(Direction),
    Add,
    Mul,
    Not,
    And,
    Or,
    GT,
    LT,
    Bool(bool),
    Int(usize),
    Str(String),
    Load(SymbolId),
    Store(SymbolId),
}
