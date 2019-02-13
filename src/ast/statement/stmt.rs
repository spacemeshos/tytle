use crate::ast::statement::{
    CommandStmt, DirectionStmt, IfStmt, MakeStmt, ProcedureStmt, RepeatStmt,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    NOP,
    EOF,
    Command(CommandStmt),
    Direction(DirectionStmt),
    Make(MakeStmt),
    If(IfStmt),
    Repeat(RepeatStmt),
    Procedure(ProcedureStmt),
}
