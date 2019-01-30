pub mod block_stmt;
pub mod command_stmt;
pub mod direction;
pub mod if_stmt;
pub mod make_stmt;
pub mod procedure_stmt;
pub mod repeat_stmt;

use block_stmt::BlockStatement;
use command_stmt::CommandStmt;
use direction::DirectionStmt;
use if_stmt::IfStmt;
use make_stmt::MakeStmt;
use procedure_stmt::ProcedureStmt;
use repeat_stmt::RepeatStmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Nop,
    Command(CommandStmt),
    Direction(DirectionStmt),
    Make(MakeStmt),
    If(IfStmt),
    Repeat(RepeatStmt),
    Procedure(ProcedureStmt),
}
