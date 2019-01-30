use crate::ast::direction::Direction;
use crate::ast::expression::Expression;
use crate::ir::instruction::Instruction;
use crate::lexer::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    stmts: Vec<Statement>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self {
            stmts: Default::default(),
        }
    }

    pub fn add_statement(&mut self, stmt: Statement) {
        if stmt != Statement::Nop {
            self.stmts.push(stmt);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureStmt {
    pub name: String,
    pub block: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Int,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MakeStmt {
    pub symbol: Symbol,
    pub expr: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalStmt {
    pub symbol: SymbolKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatStmt {
    pub count_expr: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionStmt {
    pub direction: Direction,
    pub distance_expr: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub cond_expr: Expression,
    pub true_block: BlockStatement,
    pub false_block: Option<BlockStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandStmt {
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    PenErase,
}

impl Default for BlockStatement {
    fn default() -> Self {
        Self {
            stmts: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Command(CommandStmt),
    Direction(DirectionStmt),
    Repeat(RepeatStmt),
    If(IfStmt),
    Make(MakeStmt),
    Procedure(ProcedureStmt),
    Nop,
}
