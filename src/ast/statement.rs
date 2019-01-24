use crate::ast::direction::Direction;
use crate::ir::instruction::Instruction;
use crate::lexer::location::Location;

pub struct Expression {
    //
}

pub struct ShowExpr {
    varname: String,
}

pub struct OutputExpr {
    expr: Expression,
}

pub struct ProcedureStmt {
    loction: Option<Location>,
    name: String,
    block: BlockStatement,
}

pub enum SymbolKind {
    Int,
    String,
}

pub struct Symbol {
    name: String,
    kind: SymbolKind,
}

pub struct MakeStmt {
    symbol: Symbol,
    expr: Box<Expression>,
}

pub struct LocalStmt {
    symbol: SymbolKind,
}

pub struct RepeatStmt {
    count: Expression,
    block: BlockStatement,
}

pub struct DirectionStmt {
    direction: Direction,
    distance_expr: Expression,
}

pub struct IfStmt {
    condition: Box<Expression>,
    true_block: BlockStatement,
    false_block: Option<BlockStatement>,
}

pub enum CommandStmt {
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    PenErase,
}

pub struct BlockStatement {
    stmts: Vec<Statement>,
}

pub enum Statement {
    Command(CommandStmt),
    Direction(DirectionStmt),
    Repeat(RepeatStmt),
    If(IfStmt),
    Make(MakeStmt),
    Procedure(ProcedureStmt),
}
