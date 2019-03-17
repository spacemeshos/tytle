use crate::ast::expression::Expression;

use crate::ast::statement::{
    Command, DirectionStmt, IfStmt, MakeStmt, ProcedureStmt, RepeatStmt, ReturnStmt,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    NOP,
    EOF,
    Expression(Expression),
    Command(Command),
    Direction(DirectionStmt),
    Make(MakeStmt),
    If(IfStmt),
    Repeat(RepeatStmt),
    Procedure(ProcedureStmt),
    Return(ReturnStmt),
}

impl Statement {
    pub fn as_expr(&self) -> &Expression {
        match self {
            Statement::Expression(expr) => expr,
            _ => panic!("expected statement to be an expression-statement"),
        }
    }
}
