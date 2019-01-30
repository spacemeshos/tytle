use crate::ast::command::Command;
use crate::ast::direction::Direction;
use crate::ast::expression::Expression;
use crate::ast::program::Program;
use crate::ast::statement::{
    BlockStatement, DirectionStmt, IfStmt, LocalStmt, MakeStmt, ProcedureStmt, RepeatStmt,
    Statement, Symbol,
};
use crate::lexer::{location::Location, simple_lexer::SimpleLexer, token::Token, Lexer};
use crate::parser::{Parser, ParserResult};

struct ProgramParser;

impl ProgramParser {
    fn new() -> Self {
        Self {}
    }
}

impl Parser for ProgramParser {
    fn parse(&mut self, code: &str) -> ParserResult {
        let mut parser = Self::new();
        let mut lexer = SimpleLexer::new(code);

        let program = parser.parse_program(&mut lexer);
        Ok(program)
    }
}

impl ProgramParser {
    fn parse_program(&mut self, lexer: &mut impl Lexer) -> Program {
        let mut program = Program::default();

        while let Some(stmt) = self.parse_statement(lexer) {
            if stmt != Statement::Nop {
                program.statements.push(stmt);
            }
        }

        program
    }

    fn parse_statement(&self, lexer: &mut impl Lexer) -> Option<Statement> {
        let tok_loc = self.peek_current_token(lexer);
        if tok_loc.is_none() {
            return None;
        }

        let (token, location) = tok_loc.unwrap();

        match token {
            Token::EOF => return None,
            Token::NEWLINE => {
                self.skip_token(lexer);
                Some(Statement::Nop)
            }
            Token::VALUE(val) => match val.as_str() {
                "REPEAT" => self.parse_repeat_stmt(lexer),
                "IF" => self.parse_if_stmt(lexer),
                "TO" => self.parse_proc_stmt(lexer),
                _ => self.parse_basic_stmt(val.clone().as_str(), lexer),
            },
            _ => panic!(),
        }
    }

    fn parse_proc_stmt(&self, lexer: &mut impl Lexer) -> Option<Statement> {
        self.skip_token(lexer); // skipping the `TO` token

        let name = self.expect_ident(lexer);
        let borders = (None, Token::VALUE("END".to_string()));

        let block = self.parse_block_stmt(lexer, borders);

        let proc_stmt = ProcedureStmt { name, block };

        Some(Statement::Procedure(proc_stmt))
    }

    fn parse_repeat_stmt(&self, lexer: &mut impl Lexer) -> Option<Statement> {
        self.skip_token(lexer); // skipping the `REPEAT` token

        let count_expr = self.parse_expr(lexer);

        let borders = (Some(Token::LBRACKET), Token::RBRACKET);

        let block = self.parse_block_stmt(lexer, borders);

        let repeat_stmt = RepeatStmt { count_expr, block };

        Some(Statement::Repeat(repeat_stmt))
    }

    fn parse_if_stmt(&self, lexer: &mut impl Lexer) -> Option<Statement> {
        self.skip_token(lexer); // skipping the `IF` token

        let borders = (Some(Token::LBRACKET), Token::RBRACKET);

        let cond_expr = self.parse_expr(lexer);
        let true_block = self.parse_block_stmt(lexer, borders.clone());
        let mut false_block = None;

        let tok_loc = self.peek_current_token(lexer);

        if tok_loc.is_some() {
            let (tok, loc) = tok_loc.unwrap();

            if *tok == Token::LBRACKET {
                false_block = Some(self.parse_block_stmt(lexer, borders.clone()));
            }
        }

        let if_stmt = IfStmt {
            cond_expr,
            true_block,
            false_block,
        };

        Some(Statement::If(if_stmt))
    }

    fn parse_block_stmt(
        &self,
        lexer: &mut impl Lexer,
        block_borders: (Option<Token>, Token),
    ) -> BlockStatement {
        let mut block = BlockStatement::new();

        let (block_start_token, block_end_token) = block_borders;

        if block_start_token.is_some() {
            self.expect_token(lexer, block_start_token.unwrap());
        }

        let mut completed = false;

        while !completed {
            let stmt = self.parse_statement(lexer).unwrap();
            block.add_statement(stmt);

            let (tok, loc) = self.peek_current_token(lexer).unwrap();

            if *tok == block_end_token {
                self.skip_token(lexer); // skipping the block ending token`
                completed = true;
            }
        }

        block
    }

    fn parse_basic_stmt(&self, val: &str, lexer: &mut impl Lexer) -> Option<Statement> {
        let stmt = match val {
            "MAKE" => self.parse_make(lexer),
            "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" => self.parse_direction(val, lexer),
            _ => self.parse_command(val, lexer),
        };

        Some(stmt)
    }

    fn parse_command(&self, val: &str, lexer: &mut impl Lexer) -> Statement {
        self.skip_token(lexer); // skipping the `command` token

        let cmd = Command::from(val);

        Statement::Command(cmd)
    }

    fn parse_make(&self, lexer: &mut impl Lexer) -> Statement {
        self.skip_token(lexer); // skipping the `MAKE` token

        let name = self.expect_ident(lexer);
        let symbol = Symbol { name };

        self.expect_token(lexer, Token::ASSIGN);

        let expr = self.parse_expr(lexer);

        let stmt = MakeStmt { symbol, expr };

        Statement::Make(stmt)
    }

    fn parse_direction(&self, direction: &str, lexer: &mut impl Lexer) -> Statement {
        // skipping the direction token
        // we already have the value under `direction`
        self.skip_token(lexer);

        let expr = self.parse_expr(lexer);

        self.expect_newline(lexer);

        let stmt = DirectionStmt {
            expr,
            direction: Direction::from(direction),
        };

        Statement::Direction(stmt)
    }

    fn parse_expr(&self, lexer: &mut impl Lexer) -> Expression {
        let left_expr = self.parse_mul_expr(lexer);

        let (tok, loc) = self.peek_current_token(lexer).unwrap();

        if *tok == Token::ADD {
            self.skip_token(lexer); // we skip the `+` token
            let right_expr = self.parse_expr(lexer);
            Expression::Add(Box::new(left_expr), Box::new(right_expr))
        } else {
            left_expr
        }
    }

    fn parse_mul_expr(&self, lexer: &mut impl Lexer) -> Expression {
        let lparen_expr = self.parse_parens_expr(lexer);

        let (tok, loc) = self.peek_current_token(lexer).unwrap();

        if *tok == Token::MUL {
            self.skip_token(lexer); // skip the `*`

            let rparen_expr = self.parse_parens_expr(lexer);
            Expression::Mul(Box::new(lparen_expr), Box::new(rparen_expr))
        } else {
            lparen_expr
        }
    }

    fn parse_parens_expr(&self, lexer: &mut impl Lexer) -> Expression {
        let (tok, loc) = self.peek_current_token(lexer).unwrap();

        if *tok == Token::LPAREN {
            self.skip_token(lexer); // skip the `(`

            let inner_expr = self.parse_expr(lexer);

            self.expect_token(lexer, Token::RPAREN);

            inner_expr
        } else {
            self.parse_literal_expr(lexer)
        }
    }

    fn parse_literal_expr(&self, lexer: &mut impl Lexer) -> Expression {
        let pair = self.pop_current_token(lexer);

        let (tok, loc) = pair.unwrap();

        if let Token::VALUE(v) = tok {
            let num = v.parse::<usize>().unwrap();

            Expression::Int(num)
        } else {
            panic!();
        }
    }

    fn expect_newline(&self, lexer: &mut impl Lexer) {
        let tok_loc = self.pop_current_token(lexer);

        if tok_loc.is_some() {
            let (tok, loc) = tok_loc.unwrap();

            match tok {
                Token::EOF | Token::NEWLINE => return,
                _ => panic!("invalid input"),
            }
        }
    }

    fn expect_ident(&self, lexer: &mut impl Lexer) -> String {
        let (token, loc) = self.pop_current_token(lexer).unwrap();

        if let Token::VALUE(v) = token {
            return v;
        } else {
            panic!("Expected an identifier");
        }
    }

    fn expect_token(&self, lexer: &mut impl Lexer, expected: Token) {
        let (actual, loc) = self.pop_current_token(lexer).unwrap();

        assert_eq!(actual, expected);
    }

    fn peek_current_token<'a>(&self, lexer: &'a impl Lexer) -> Option<&'a (Token, Location)> {
        lexer.peek_current_token()
    }

    fn skip_token(&self, lexer: &mut impl Lexer) {
        self.pop_current_token(lexer);
    }

    fn pop_current_token(&self, lexer: &mut impl Lexer) -> Option<(Token, Location)> {
        lexer.pop_current_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_forward() {
        let actual = ProgramParser.parse("FORWARD 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn direction_backward() {
        let actual = ProgramParser.parse("BACKWARD 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Backward,
                expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn direction_left() {
        let actual = ProgramParser.parse("LEFT 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Left,
                expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn direction_right() {
        let actual = ProgramParser.parse("RIGHT 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Right,
                expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn direction_forward_and_then_backward_no_empty_lines() {
        let actual = ProgramParser.parse("FORWARD 10\nRIGHT 20").unwrap();

        let expected = Program {
            statements: vec![
                Statement::Direction(DirectionStmt {
                    direction: Direction::Forward,
                    expr: Expression::Int(10),
                }),
                Statement::Direction(DirectionStmt {
                    direction: Direction::Right,
                    expr: Expression::Int(20),
                }),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn direction_forward_and_then_backward_with_empty_lines() {
        let actual = ProgramParser
            .parse("\n\nFORWARD 10\n\nRIGHT 20\n\n")
            .unwrap();

        let expected = Program {
            statements: vec![
                Statement::Direction(DirectionStmt {
                    direction: Direction::Forward,
                    expr: Expression::Int(10),
                }),
                Statement::Direction(DirectionStmt {
                    direction: Direction::Right,
                    expr: Expression::Int(20),
                }),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn expr_integer_surrounded_by_parentheses() {
        let actual = ProgramParser.parse("FORWARD (10)").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Int(10),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn expr_add_integers_with_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2))),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn expr_add_integers_without_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2))),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn expr_add_and_mul_integers() {
        let actual = ProgramParser.parse("FORWARD 1 * 2 + 3 * 4").unwrap();

        let clause1 = Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));
        let clause2 = Expression::Mul(Box::new(Expression::Int(3)), Box::new(Expression::Int(4)));
        let expr = Expression::Add(Box::new(clause1), Box::new(clause2));

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr,
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn expr_mul_integers_without_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 * 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr: Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(2))),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn expr_mix_of_mul_add_ops_between_integers_and_parentheses() {
        let actual = ProgramParser
            .parse("FORWARD (1*1 + 2) * (3*3 + 4)")
            .unwrap();

        let ones_mul = Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(1)));
        let three_mul = Expression::Mul(Box::new(Expression::Int(3)), Box::new(Expression::Int(3)));

        let add_1_2 = Expression::Add(Box::new(ones_mul), Box::new(Expression::Int(2)));
        let add_3_4 = Expression::Add(Box::new(three_mul), Box::new(Expression::Int(4)));

        let expr = Expression::Mul(Box::new(add_1_2), Box::new(add_3_4));

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                expr,
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_variable_assign_an_integer() {
        let actual = ProgramParser.parse("MAKE MyVar = 2").unwrap();

        let make_stmt = Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "MyVar".to_string(),
            },
            expr: Expression::Int(2),
        });

        let expected = Program {
            statements: vec![make_stmt],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_variable_assign_an_expr() {
        let actual = ProgramParser.parse("MAKE MyVar = 1 + 2").unwrap();

        let expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

        let make_stmt = Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "MyVar".to_string(),
            },
            expr,
        });

        let expected = Program {
            statements: vec![make_stmt],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn if_stmt_without_else() {
        let actual = ProgramParser
            .parse("IF 1 + 2 [MAKE A = 3 \n MAKE B = 4]")
            .unwrap();

        let cond_expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

        let mut true_block = BlockStatement::new();
        true_block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "A".to_string(),
            },
            expr: Expression::Int(3),
        }));

        true_block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "B".to_string(),
            },
            expr: Expression::Int(4),
        }));

        let if_stmt = Statement::If(IfStmt {
            cond_expr,
            true_block,
            false_block: None,
        });

        let expected = Program {
            statements: vec![if_stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn if_stmt_with_else() {
        let actual = ProgramParser
            .parse("IF 1 + 2 [MAKE A = 1] [MAKE B = 2]")
            .unwrap();

        let cond_expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

        let mut true_block = BlockStatement::new();
        true_block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "A".to_string(),
            },
            expr: Expression::Int(1),
        }));

        let mut false_block = BlockStatement::new();
        false_block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "B".to_string(),
            },
            expr: Expression::Int(2),
        }));

        let if_stmt = Statement::If(IfStmt {
            cond_expr,
            true_block,
            false_block: Some(false_block),
        });

        let expected = Program {
            statements: vec![if_stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn repeat_stmt() {
        let actual = ProgramParser
            .parse("REPEAT 1 + 2 [MAKE A = 3 \n MAKE B = 4]")
            .unwrap();

        let count_expr =
            Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

        let mut block = BlockStatement::new();
        block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "A".to_string(),
            },
            expr: Expression::Int(3),
        }));

        block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "B".to_string(),
            },
            expr: Expression::Int(4),
        }));

        let repeat_stmt = Statement::Repeat(RepeatStmt { count_expr, block });

        let expected = Program {
            statements: vec![repeat_stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn to_stmt() {
        let actual = ProgramParser
            .parse("TO MyProc \n MAKE A = 3 \n MAKE B = 4 \n END")
            .unwrap();

        let mut block = BlockStatement::new();
        block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "A".to_string(),
            },
            expr: Expression::Int(3),
        }));

        block.add_statement(Statement::Make(MakeStmt {
            symbol: Symbol {
                name: "B".to_string(),
            },
            expr: Expression::Int(4),
        }));

        let proc_stmt = Statement::Procedure(ProcedureStmt {
            name: "MyProc".to_string(),
            block,
        });

        let expected = Program {
            statements: vec![proc_stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_pen_up() {
        let actual = ProgramParser.parse("PENUP").unwrap();

        let stmt = Statement::Command(Command::PenUp);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_pen_down() {
        let actual = ProgramParser.parse("PENDOWN").unwrap();

        let stmt = Statement::Command(Command::PenDown);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_show_turtle() {
        let actual = ProgramParser.parse("SHOWTURTLE").unwrap();

        let stmt = Statement::Command(Command::ShowTurtle);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_hide_turtle() {
        let actual = ProgramParser.parse("HIDETURTLE").unwrap();

        let stmt = Statement::Command(Command::HideTurtle);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_pen_erase() {
        let actual = ProgramParser.parse("PENERASE").unwrap();

        let stmt = Statement::Command(Command::PenErase);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_clear_screen() {
        let actual = ProgramParser.parse("CLEARSCREEN").unwrap();

        let stmt = Statement::Command(Command::ClearScreen);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_set_pen_color() {
        let actual = ProgramParser.parse("SETPENCOLOR").unwrap();

        let stmt = Statement::Command(Command::SetPenColor);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn command_set_background_color() {
        let actual = ProgramParser.parse("SETBACKGROUND").unwrap();

        let stmt = Statement::Command(Command::SetBackgroundColor);

        let expected = Program {
            statements: vec![stmt],
        };

        assert_eq!(expected, actual);
    }
}
