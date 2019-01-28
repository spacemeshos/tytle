use crate::ast::direction::Direction;
use crate::ast::program::Program;
use crate::ast::statement::{
    BlockStatement, CommandStmt, DirectionStmt, Expression, IfStmt, IntExpr, LocalStmt, MakeStmt,
    ProcedureStmt, RepeatStmt, ShowExpr, Statement,
};
use crate::lexer::{location::Location, simple_lexer::SimpleLexer, token::Token, Lexer};
use crate::parser::{Parser, ParserResult};

struct ProgramParser;

impl Parser for ProgramParser {
    fn parse(&mut self, code: &str) -> ParserResult {
        let program = Program::default();
        Ok(program)
    }
}

impl Parser {
    fn parse_program(&mut self, lexer: &mut impl Lexer) -> Program {
        let mut program = Program::default();

        while let Some(stmt) = self.parse_statement(lexer) {
            program.statements.push(stmt);
        }

        program
    }

    fn parse_statement(&mut self, lexer: &mut impl Lexer) -> Option<Statement> {
        lexer.buffer_more_tokens();

        if lexer.peek_current_token().is_none() {
            return None;
        }

        let (token, location) = lexer.pop_current_token().unwrap();

        let stmt = match token {
            Token::EOF => return None,
            Token::NEWLINE => panic!(),
            Token::VALUE(val) => match val.as_str() {
                "REPEAT" => {
                    unimplemented!();
                }
                "IF" => {
                    unimplemented!();
                }
                "TO" => {
                    unimplemented!();
                }
                _ => {
                    self.parse_basic_statement(val.as_str(), lexer).unwrap();
                }
            },
        };

        panic!()

        // Some(stmt)
    }

    fn parse_basic_statement(&mut self, val: &str, lexer: &mut impl Lexer) -> Option<Statement> {
        let stmt = match val {
            "MAKE" => {
                unimplemented!();
            }
            "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" => self.parse_direction(val, lexer),
            _ => {
                unimplemented!();
            }
        };

        Some(stmt)
    }

    fn parse_direction(&mut self, direction: &str, lexer: &mut impl Lexer) -> Statement {
        let distance_expr = self.parse_expr(lexer);
        let stmt = DirectionStmt {
            direction: Direction::from(direction),
            distance_expr: distance_expr,
        };

        Statement::Direction(stmt)
    }

    fn parse_command(&mut self, val: &str, lexer: &mut impl Lexer) -> CommandStmt {
        match val {
            "PENUP" => CommandStmt::PenUp,
            "PENDOWN" => CommandStmt::PenDown,
            "SHOWTURTLE" => CommandStmt::ShowTurtle,
            "HIDETURTLE" => CommandStmt::HideTurtle,
            _ => panic!(),
        }
    }

    fn parse_expr(&mut self, lexer: &mut Lexer) -> Expression {
        let value = self.expect_number(lexer);

        Expression::Int(IntExpr { value })
    }

    fn expect_number(&mut self, lexer: &mut Lexer) -> usize {
        10
    }
}
