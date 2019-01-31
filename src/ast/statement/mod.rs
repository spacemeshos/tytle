mod block_stmt;
mod command_stmt;
mod if_stmt;
mod make_stmt;
mod procedure_stmt;
mod repeat_stmt;

pub mod direction;

pub use block_stmt::BlockStatement;
pub use command_stmt::CommandStmt;
pub use direction::DirectionStmt;
pub use if_stmt::IfStmt;
pub use make_stmt::MakeStmt;
pub use procedure_stmt::ProcedureStmt;
pub use repeat_stmt::RepeatStmt;

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
