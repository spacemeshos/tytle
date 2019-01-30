use crate::ast::direction::Direction;
use crate::ast::expression::Expression;
use crate::ast::program::Program;
use crate::ast::statement::{
    BlockStatement, CommandStmt, DirectionStmt, IfStmt, LocalStmt, MakeStmt, ProcedureStmt,
    RepeatStmt, Statement, Symbol,
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
                "TO" => {
                    unimplemented!();
                }
                _ => self.parse_basic_stmt(val.clone().as_str(), lexer),
            },
            _ => panic!(),
        }
    }

    fn parse_repeat_stmt(&self, lexer: &mut impl Lexer) -> Option<Statement> {
        self.skip_token(lexer); // skipping the `REPEAT` token

        let count_expr = self.parse_expr(lexer);
        let block = self.parse_block_stmt(lexer);

        let repeat_stmt = RepeatStmt { count_expr, block };

        Some(Statement::Repeat(repeat_stmt))
    }

    fn parse_if_stmt(&self, lexer: &mut impl Lexer) -> Option<Statement> {
        self.skip_token(lexer); // skipping the `IF` token

        let cond_expr = self.parse_expr(lexer);
        let true_block = self.parse_block_stmt(lexer);
        let mut false_block = None;

        let tok_loc = self.peek_current_token(lexer);

        if tok_loc.is_some() {
            let (tok, loc) = tok_loc.unwrap();

            if *tok == Token::LBRACKET {
                false_block = Some(self.parse_block_stmt(lexer));
            }
        }

        let if_stmt = IfStmt {
            cond_expr,
            true_block,
            false_block,
        };

        Some(Statement::If(if_stmt))
    }

    fn parse_block_stmt(&self, lexer: &mut impl Lexer) -> BlockStatement {
        let mut block = BlockStatement::new();

        self.expect_token(lexer, Token::LBRACKET);

        let mut completed = false;

        while !completed {
            let stmt = self.parse_statement(lexer).unwrap();
            block.add_statement(stmt);

            let (tok, loc) = self.peek_current_token(lexer).unwrap();

            if *tok == Token::RBRACKET {
                self.skip_token(lexer); // skipping the `]` token
                completed = true;
            }
        }

        block
    }

    fn parse_basic_stmt(&self, val: &str, lexer: &mut impl Lexer) -> Option<Statement> {
        let stmt = match val {
            "MAKE" => self.parse_make(lexer),
            "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" => self.parse_direction(val, lexer),
            _ => {
                unimplemented!();
            }
        };

        Some(stmt)
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

        let distance_expr = self.parse_expr(lexer);

        self.expect_newline(lexer);

        let stmt = DirectionStmt {
            distance_expr,
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
    fn forward() {
        let actual = ProgramParser.parse("FORWARD 20").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Int(20),
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
                distance_expr: Expression::Int(20),
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
                distance_expr: Expression::Int(20),
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
                distance_expr: Expression::Int(20),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_and_then_backward_no_empty_lines() {
        let actual = ProgramParser.parse("FORWARD 10\nRIGHT 20").unwrap();

        let expected = Program {
            statements: vec![
                Statement::Direction(DirectionStmt {
                    direction: Direction::Forward,
                    distance_expr: Expression::Int(10),
                }),
                Statement::Direction(DirectionStmt {
                    direction: Direction::Right,
                    distance_expr: Expression::Int(20),
                }),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_and_then_backward_with_empty_lines() {
        let actual = ProgramParser
            .parse("\n\nFORWARD 10\n\nRIGHT 20\n\n")
            .unwrap();

        let expected = Program {
            statements: vec![
                Statement::Direction(DirectionStmt {
                    direction: Direction::Forward,
                    distance_expr: Expression::Int(10),
                }),
                Statement::Direction(DirectionStmt {
                    direction: Direction::Right,
                    distance_expr: Expression::Int(20),
                }),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_integer_expr_surrounded_by_parentheses() {
        let actual = ProgramParser.parse("FORWARD (10)").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Int(10),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_add_integers_expr_with_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Add(
                    Box::new(Expression::Int(1)),
                    Box::new(Expression::Int(2)),
                ),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_add_integers_expr_without_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 + 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Add(
                    Box::new(Expression::Int(1)),
                    Box::new(Expression::Int(2)),
                ),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_add_and_mul_integers_expr() {
        let actual = ProgramParser.parse("FORWARD 1 * 2 + 3 * 4").unwrap();

        let clause1 = Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));
        let clause2 = Expression::Mul(Box::new(Expression::Int(3)), Box::new(Expression::Int(4)));
        let distance_expr = Expression::Add(Box::new(clause1), Box::new(clause2));

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr,
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_only_mul_integers_expr_without_spaces() {
        let actual = ProgramParser.parse("FORWARD 1 * 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr: Expression::Mul(
                    Box::new(Expression::Int(1)),
                    Box::new(Expression::Int(2)),
                ),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn forward_mix_of_mul_add_ops_between_integers_and_parentheses_expr() {
        let actual = ProgramParser
            .parse("FORWARD (1*1 + 2) * (3*3 + 4)")
            .unwrap();

        let ones_mul = Expression::Mul(Box::new(Expression::Int(1)), Box::new(Expression::Int(1)));
        let three_mul = Expression::Mul(Box::new(Expression::Int(3)), Box::new(Expression::Int(3)));

        let add_1_2 = Expression::Add(Box::new(ones_mul), Box::new(Expression::Int(2)));
        let add_3_4 = Expression::Add(Box::new(three_mul), Box::new(Expression::Int(4)));

        let distance_expr = Expression::Mul(Box::new(add_1_2), Box::new(add_3_4));

        let expected = Program {
            statements: vec![Statement::Direction(DirectionStmt {
                direction: Direction::Forward,
                distance_expr,
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_variable_assign_an_integer() {
        let actual = ProgramParser.parse("MAKE MyVar = 2").unwrap();

        let expected = Program {
            statements: vec![Statement::Make(MakeStmt {
                symbol: Symbol {
                    name: "MyVar".to_string(),
                },
                expr: Expression::Int(2),
            })],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn make_variable_assign_an_expr() {
        let actual = ProgramParser.parse("MAKE MyVar = 1 + 2").unwrap();

        let expr = Expression::Add(Box::new(Expression::Int(1)), Box::new(Expression::Int(2)));

        let expected = Program {
            statements: vec![Statement::Make(MakeStmt {
                symbol: Symbol {
                    name: "MyVar".to_string(),
                },
                expr,
            })],
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
    fn if_else_stmt() {
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

        let expected_block_stmt = Statement::If(IfStmt {
            cond_expr,
            true_block,
            false_block: Some(false_block),
        });

        let expected = Program {
            statements: vec![expected_block_stmt],
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
}
