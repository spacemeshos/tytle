use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::ast::Ast;

use crate::lexer::{Lexer, Location, Token, TytleLexer};
use crate::parser::{ParseError, Parser, ParserResult};

pub type StatementResult = Result<Statement, ParseError>;
pub type ExpressionResult = Result<Expression, ParseError>;

pub struct TytleParser;

impl TytleParser {
    fn new() -> Self {
        Self {}
    }
}

impl Parser for TytleParser {
    fn parse(&mut self, code: &str) -> ParserResult {
        let mut lexer = TytleLexer::new(code);

        self.parse(&mut lexer)
    }
}

impl TytleParser {
    fn parse(&mut self, lexer: &mut impl Lexer) -> ParserResult {
        let mut ast = Ast::default();

        loop {
            let stmt = self.parse_statement(lexer)?;

            match stmt {
                Statement::NOP => continue,
                Statement::EOF => break,
                _ => ast.statements.push(stmt),
            }
        }

        if ast.statements.len() == 0 {
            ast.statements.push(Statement::EOF);
        }

        Ok(ast)
    }

    fn parse_statement(&self, lexer: &mut impl Lexer) -> StatementResult {
        let tok_loc = self.peek_current_token(lexer);
        if tok_loc.is_none() {
            return Ok(Statement::EOF);
        }

        let (token, location) = tok_loc.unwrap();

        match token {
            Token::EOF => Ok(Statement::EOF),
            Token::NEWLINE => {
                self.skip_token(lexer);
                Ok(Statement::NOP)
            }
            Token::VALUE(val) => match val.as_str() {
                "REPEAT" => self.parse_repeat_stmt(lexer),
                "IF" => self.parse_if_stmt(lexer),
                "TO" => self.parse_proc_stmt(lexer),
                _ => self.parse_basic_stmt(val.clone().as_str(), lexer),
            },
            _ => unimplemented!(),
        }
    }

    fn parse_proc_stmt(&self, lexer: &mut impl Lexer) -> StatementResult {
        self.skip_token(lexer); // skipping the `TO` token

        let name = self.expect_ident(lexer)?;
        let borders = (None, Token::VALUE("END".to_string()));
        let (params, return_type) = self.parse_proc_signature(lexer)?;
        let block = self.parse_block_stmt(lexer, borders)?;

        let proc_stmt = ProcedureStmt {
            name,
            block,
            params,
            return_type,
        };

        Ok(Statement::Procedure(proc_stmt))
    }

    fn parse_proc_signature(
        &self,
        lexer: &mut impl Lexer,
    ) -> Result<(Vec<ProcParam>, Option<String>), ParseError> {
        let mut params = Vec::new();
        let mut completed = false;

        // expecting signature to start with `(`
        self.expect_token(lexer, Token::LPAREN)?;

        while !completed {
            let (tok, loc) = self.peek_current_token(lexer).unwrap();

            if *tok == Token::RPAREN {
                self.skip_token(lexer); // skipping the `)`
                completed = true
            } else {
                let param_name = self.expect_ident(lexer)?;

                self.validate_var_name(param_name.as_str())?;

                self.expect_token(lexer, Token::COLON)?;

                let param_type = self.expect_ident(lexer)?;

                let param = ProcParam {
                    param_name: param_name.to_string(),
                    param_type: param_type.to_string(),
                };

                params.push(param);

                if self.peek_current_token_clone(lexer) == Token::COMMA {
                    self.skip_token(lexer);
                }
            }
        }

        let (tok, loc) = self.peek_current_token(lexer).unwrap();

        let return_type = if *tok == Token::COLON {
            self.skip_token(lexer); // skipping the `:`

            Some(self.expect_ident(lexer)?)
        } else {
            None
        };

        Ok((params, return_type))
    }

    fn parse_repeat_stmt(&self, lexer: &mut impl Lexer) -> StatementResult {
        self.skip_token(lexer); // skipping the `REPEAT` token

        let count_expr = self.parse_expr(lexer)?;
        let borders = (Some(Token::LBRACKET), Token::RBRACKET);
        let block = self.parse_block_stmt(lexer, borders)?;
        let repeat_stmt = RepeatStmt { count_expr, block };

        Ok(Statement::Repeat(repeat_stmt))
    }

    fn parse_if_stmt(&self, lexer: &mut impl Lexer) -> StatementResult {
        self.skip_token(lexer); // skipping the `IF` token

        let borders = (Some(Token::LBRACKET), Token::RBRACKET);

        let cond_expr = self.parse_expr(lexer)?;
        let true_block = self.parse_block_stmt(lexer, borders.clone())?;
        let mut false_block = None;

        let tok_loc = self.peek_current_token(lexer);

        if tok_loc.is_some() {
            let (tok, loc) = tok_loc.unwrap();

            if *tok == Token::LBRACKET {
                let block_stmt = self.parse_block_stmt(lexer, borders.clone())?;
                false_block = Some(block_stmt);
            }
        }

        let if_stmt = IfStmt {
            cond_expr,
            true_block,
            false_block,
        };

        Ok(Statement::If(if_stmt))
    }

    fn parse_block_stmt(
        &self,
        lexer: &mut impl Lexer,
        block_borders: (Option<Token>, Token),
    ) -> Result<BlockStatement, ParseError> {
        let mut block = BlockStatement::new();

        let (start_tok, end_tok) = block_borders;

        if start_tok.is_some() {
            self.expect_token(lexer, start_tok.unwrap())?;
        }

        let mut completed = false;

        while !completed {
            let stmt = self.parse_statement(lexer)?;

            block.add_statement(stmt);

            let (tok, loc) = self.peek_current_token(lexer).unwrap();

            if *tok == end_tok {
                self.skip_token(lexer); // skipping the block `ending token`
                completed = true;
            }
        }

        Ok(block)
    }

    fn parse_basic_stmt(&self, val: &str, lexer: &mut impl Lexer) -> StatementResult {
        match val {
            "MAKE" => self.parse_make_assign(lexer),
            "MAKEGLOBAL" => self.parse_make_global(lexer),
            "MAKELOCAL" => self.parse_make_local(lexer),
            "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" | "SETX" | "SETY" => {
                self.parse_direction(val, lexer)
            }
            _ => self.parse_command(val, lexer),
        }
    }

    fn parse_command(&self, val: &str, lexer: &mut impl Lexer) -> StatementResult {
        self.skip_token(lexer); // skipping the `command` token

        let stmt = CommandStmt::from(val);

        Ok(Statement::Command(stmt))
    }

    fn parse_make_global(&self, lexer: &mut impl Lexer) -> StatementResult {
        self.build_make_stmt(lexer, MakeStmtKind::Global)
    }

    fn parse_make_local(&self, lexer: &mut impl Lexer) -> StatementResult {
        self.build_make_stmt(lexer, MakeStmtKind::Local)
    }

    fn parse_make_assign(&self, lexer: &mut impl Lexer) -> StatementResult {
        self.build_make_stmt(lexer, MakeStmtKind::Assign)
    }

    fn build_make_stmt(&self, lexer: &mut impl Lexer, kind: MakeStmtKind) -> StatementResult {
        self.skip_token(lexer); // skipping the `MAKE/MAKEGLOBAL/MAKELOCAL` token

        let mut var = self.expect_ident(lexer)?;

        self.validate_var_name(var.as_str())?;

        self.expect_token(lexer, Token::ASSIGN)?;

        let expr = self.parse_expr(lexer)?;
        let stmt = MakeStmt { var, expr, kind };

        Ok(Statement::Make(stmt))
    }

    fn parse_direction(&self, direction: &str, lexer: &mut impl Lexer) -> StatementResult {
        // skipping the direction token
        // we already have the value under `direction`
        self.skip_token(lexer);

        let expr = self.parse_expr(lexer)?;

        self.expect_newline(lexer)?;

        let stmt = DirectionStmt {
            expr,
            direction: Direction::from(direction),
        };

        Ok(Statement::Direction(stmt))
    }

    fn parse_expr(&self, lexer: &mut impl Lexer) -> ExpressionResult {
        let left_expr = self.parse_mul_expr(lexer)?;

        let (tok, loc) = self.peek_current_token(lexer).unwrap();

        let expr = if *tok == Token::ADD {
            self.skip_token(lexer); // we skip the `+` token
            let right_expr = self.parse_expr(lexer)?;
            Expression::Binary(BinaryOp::Add, Box::new(left_expr), Box::new(right_expr))
        } else {
            left_expr
        };

        Ok(expr)
    }

    fn parse_mul_expr(&self, lexer: &mut impl Lexer) -> ExpressionResult {
        let lparen_expr = self.parse_parens_expr(lexer)?;

        let (tok, loc) = self.peek_current_token(lexer).unwrap();

        let mul_expr = if *tok == Token::MUL {
            self.skip_token(lexer); // skip the `*`

            let rparen_expr = self.parse_parens_expr(lexer)?;
            Expression::Binary(BinaryOp::Mul, Box::new(lparen_expr), Box::new(rparen_expr))
        } else {
            lparen_expr
        };

        Ok(mul_expr)
    }

    fn parse_parens_expr(&self, lexer: &mut impl Lexer) -> ExpressionResult {
        let (tok, loc) = self.peek_current_token(lexer).unwrap();

        if *tok == Token::LPAREN {
            self.skip_token(lexer); // skip the `(`

            let inner_expr = self.parse_expr(lexer)?;

            self.expect_token(lexer, Token::RPAREN)?;

            Ok(inner_expr)
        } else {
            self.parse_basic_expr(lexer)
        }
    }

    fn parse_basic_expr(&self, lexer: &mut impl Lexer) -> ExpressionResult {
        let (token, _location) = self.peek_next_token(lexer).unwrap();

        let basic_expr = if *token == Token::LPAREN {
            let (proc_name, proc_params) = self.parse_proc_call_expr(lexer)?;
            Expression::ProcCall(proc_name, proc_params)
        } else {
            let expr = self.parse_literal_expr(lexer)?;
            Expression::Literal(expr)
        };

        Ok(basic_expr)
    }

    fn parse_proc_call_expr(
        &self,
        lexer: &mut impl Lexer,
    ) -> Result<(String, Vec<Expression>), ParseError> {
        let (token, _) = self.pop_current_token(lexer).unwrap();

        if let Token::VALUE(proc_name) = token {
            self.expect_token(lexer, Token::LPAREN)?;

            let proc_params = self.parse_proc_call_params_expr(lexer)?;

            self.expect_token(lexer, Token::RPAREN)?;

            Ok((proc_name, proc_params))
        } else {
            Err(ParseError::Syntax {
                message: "Invalid Call Expression".to_string(),
            })
        }
    }

    fn parse_proc_call_params_expr(
        &self,
        lexer: &mut impl Lexer,
    ) -> Result<Vec<Expression>, ParseError> {
        let mut params = Vec::new();

        while self.peek_current_token_clone(lexer) != Token::RPAREN {
            let param_expr = self.parse_call_param_expr(lexer)?;

            if param_expr.is_some() {
                params.push(param_expr.unwrap());
            }
        }

        Ok(params)
    }

    fn parse_call_param_expr(
        &self,
        lexer: &mut impl Lexer,
    ) -> Result<Option<Expression>, ParseError> {
        let expr = self.parse_expr(lexer)?;

        if self.peek_current_token_clone(lexer) == Token::COMMA {
            self.skip_token(lexer);
        }

        Ok(Some(expr))
    }

    fn parse_literal_expr(&self, lexer: &mut impl Lexer) -> Result<LiteralExpr, ParseError> {
        let pair = self.pop_current_token(lexer);

        let (tok, loc) = pair.unwrap();

        if let Token::VALUE(v) = tok {
            match v.parse::<usize>() {
                Ok(num) => Ok(LiteralExpr::Int(num)),
                Err(_) => {
                    if v.starts_with("\"") {
                        let s = v[1..v.len() - 1].to_string();
                        Ok(LiteralExpr::Str(s))
                    } else {
                        Ok(LiteralExpr::Var(v.to_string()))
                    }
                }
            }
        } else {
            panic!();
        }
    }

    fn expect_newline(&self, lexer: &mut impl Lexer) -> Result<(), ParseError> {
        let tok_loc = self.pop_current_token(lexer);

        if tok_loc.is_some() {
            let (tok, loc) = tok_loc.unwrap();

            match tok {
                Token::EOF | Token::NEWLINE => return Ok(()),
                _ => return Err(ParseError::NewLineExpected),
            }
        }

        Ok(())
    }

    fn expect_ident(&self, lexer: &mut impl Lexer) -> Result<String, ParseError> {
        let (token, loc) = self.pop_current_token(lexer).unwrap();

        if let Token::VALUE(v) = token {
            Ok(v)
        } else {
            Err(ParseError::IdentifierExpected)
        }
    }

    fn expect_token(&self, lexer: &mut impl Lexer, expected: Token) -> Result<(), ParseError> {
        let (actual, loc) = self.pop_current_token(lexer).unwrap();

        if actual == expected {
            Ok(())
        } else {
            let err = match expected {
                Token::COLON => ParseError::MissingColon,
                _ => ParseError::UnexpectedToken { expected, actual },
            };

            Err(err)
        }
    }

    fn peek_current_token<'a>(&self, lexer: &'a impl Lexer) -> Option<&'a (Token, Location)> {
        lexer.peek_current_token()
    }

    fn peek_next_token<'a>(&self, lexer: &'a impl Lexer) -> Option<&'a (Token, Location)> {
        lexer.peek_next_token()
    }

    fn peek_current_token_clone<'a>(&self, lexer: &'a impl Lexer) -> Token {
        let (token, _) = lexer.peek_current_token().unwrap();

        token.clone()
    }

    fn peek_next_token_clone<'a>(&self, lexer: &'a impl Lexer) -> Token {
        let (token, _) = lexer.peek_next_token().unwrap();

        token.clone()
    }

    fn skip_token(&self, lexer: &mut impl Lexer) {
        self.pop_current_token(lexer);
    }

    fn pop_current_token(&self, lexer: &mut impl Lexer) -> Option<(Token, Location)> {
        lexer.pop_current_token()
    }

    pub fn validate_var_name(&self, name: &str) -> Result<(), ParseError> {
        let upper = name
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_digit(10) || c == '_');

        if !upper {
            let err = ParseError::InvalidIdentifierDeclaration(format!(
                "All characters must be capital, digit or `_` (got `{}`)",
                name
            ));
            return Err(err);
        };

        let starts_with_digit = name.chars().next().unwrap().is_digit(10);

        if starts_with_digit {
            let err = ParseError::InvalidIdentifierDeclaration(format!(
                "Variable name isn't allowed to begin with a digit (got `{}`)",
                name
            ));
            return Err(err);
        }

        Ok(())
    }
}
