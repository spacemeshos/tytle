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
    pub expr: Expression,
}

pub struct ProcedureStmt {
    pub loction: Option<Location>,
    pub name: String,
    pub block: BlockStatement,
}

pub enum SymbolKind {
    Int,
    String,
}

pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
}

pub struct MakeStmt {
    pub symbol: Symbol,
    pub expr: Box<Expression>,
}

pub struct LocalStmt {
    pub symbol: SymbolKind,
}

pub struct RepeatStmt {
    pub count_expr: Expression,
    pub block: BlockStatement,
}

pub struct DirectionStmt {
    pub direction: Direction,
    pub distance_expr: Expression,
}

pub struct IfStmt {
    pub cond_expr: Box<Expression>,
    pub true_block: BlockStatement,
    pub false_block: Option<BlockStatement>,
}

pub enum CommandStmt {
    PenUp,
    PenDown,
    ShowTurtle,
    HideTurtle,
    PenErase,
}

pub struct BlockStatement {
    pub stmts: Vec<Statement>,
}

impl Default for BlockStatement {
    fn default() -> Self {
        Self {
            stmts: Default::default(),
        }
    }
}

impl BlockStatement {
    pub fn add_statement(&mut self, stmt: Statement) {
        self.stmts.push(stmt);
    }
}

pub enum Statement {
    Command(CommandStmt),
    Direction(DirectionStmt),
    Repeat(RepeatStmt),
    If(IfStmt),
    Make(MakeStmt),
    Procedure(ProcedureStmt),
}
