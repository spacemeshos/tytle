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
        let mut parser = ProgramParser {};
        let mut lexer = SimpleLexer::new(code);

        let program = parser.parse_program(&mut lexer);
        Ok(program)
    }
}

impl ProgramParser {
    fn parse_program(&mut self, lexer: &mut impl Lexer) -> Program {
        let mut program = Program::default();

        while let Some(stmt) = Self::parse_statement(lexer) {
            program.statements.push(stmt);
        }

        program
    }

    fn parse_statement(lexer: &mut impl Lexer) -> Option<Statement> {
        lexer.buffer_more_tokens();

        let tok_loc = Self::peek_current_token(lexer);
        if tok_loc.is_none() {
            return None;
        }

        let (token, location) = tok_loc.unwrap();

        match token {
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
                _ => Self::parse_basic_statement(val.clone(), lexer),
            },
        }
    }

    fn parse_basic_statement(val: String, lexer: &mut impl Lexer) -> Option<Statement> {
        let val = val.as_str();

        let stmt = match val {
            "MAKE" => {
                unimplemented!();
            }
            "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" => Self::parse_direction(val, lexer),
            _ => {
                unimplemented!();
            }
        };

        Some(stmt)
    }

    fn parse_direction(direction: &str, lexer: &mut impl Lexer) -> Statement {
        Self::skip_token(lexer);

        let distance_expr = Self::parse_expr(lexer);

        let stmt = DirectionStmt {
            distance_expr,
            direction: Direction::from(direction),
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

    fn parse_expr(lexer: &mut impl Lexer) -> Expression {
        let value = Self::expect_number(lexer);

        Expression::Int(IntExpr { value })
    }

    fn expect_number(lexer: &mut impl Lexer) -> usize {
        let pair = Self::pop_current_token(lexer);

        let (tok, loc) = pair.unwrap();

        match tok {
            Token::EOF | Token::NEWLINE => panic!("unexpected..."),
            Token::VALUE(v) => v.parse::<usize>().unwrap()
        }
    }

    fn peek_current_token(lexer: &impl Lexer) -> Option<&(Token, Location)> {
        lexer.peek_current_token()
    }

    fn skip_token(lexer: &mut impl Lexer) {
        lexer.pop_current_token();
    }

    fn pop_current_token(lexer: &mut impl Lexer) -> Option<(Token, Location)> {
        lexer.pop_current_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward() {
        let actual = ProgramParser.parse("FORWARD 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Int(IntExpr { value: 20 }),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn backward() {
        let actual = ProgramParser.parse("BACKWARD 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Backward,
                distance_expr: Expression::Int(IntExpr { value: 20 }),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn left() {
        let actual = ProgramParser.parse("LEFT 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Left,
                distance_expr: Expression::Int(IntExpr { value: 20 }),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn right() {
        let actual = ProgramParser.parse("RIGHT 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Right,
                distance_expr: Expression::Int(IntExpr { value: 20 }),
            })],
        };

        assert_eq!(actual, expected);
    }
}
