use crate::ast::expression::Expression;
use crate::ast::program::Program;

use crate::ast::statement::{
    block_stmt::BlockStatement,
    command_stmt::CommandStmt,
    direction::{Direction, DirectionStmt},
    if_stmt::IfStmt,
    make_stmt::MakeStmt,
    procedure_stmt::ProcedureStmt,
    repeat_stmt::RepeatStmt,
    Statement,
};

use crate::lexer::{location::Location, simple_lexer::SimpleLexer, token::Token, Lexer};
use crate::parser::{Parser, ParserResult};

pub struct ProgramParser;

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
            _ => panic!("Syntax error, unexpected token: {:?}", token),
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
                self.skip_token(lexer); // skipping the block `ending token`
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

        let stmt = CommandStmt::from(val);

        Statement::Command(stmt)
    }

    fn parse_make(&self, lexer: &mut impl Lexer) -> Statement {
        self.skip_token(lexer); // skipping the `MAKE` token

        let mut symbol = self.expect_ident(lexer);

        if symbol.starts_with("\"") {
            symbol = symbol[1..].to_string();
        } else {
            panic!("Invalid `MAKE` exression: {}", symbol);
        }

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
            match v.parse::<usize>() {
                Ok(num) => Expression::Int(num),
                Err(_) => {
                    if v.starts_with(":") {
                        Expression::Var(v[1..].to_string())
                    }
                    else if v.starts_with("\"") {
                        Expression::Str(v[1..].to_string())
                    }
                    else {
                        panic!("Invalid Literal: {}", v);
                    }
                }
            }
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
