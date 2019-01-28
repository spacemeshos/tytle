use crate::ast::direction::Direction;
use crate::ast::program::Program;
use crate::ast::statement::Expression;
use crate::ast::statement::Statement;
use crate::ast::statement::{
    BlockStatement, CommandStmt, DirectionStmt, IfStmt, LocalStmt, MakeStmt, ProcedureStmt,
    RepeatStmt,
};
use crate::lexer::simple_lexer::SimpleLexer;
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::parser::{Parser, ParserResult};

struct ProgramParser;

// impl Parser for ProgramParser {
//     fn parse(&mut self, code: &str) -> ParserResult {
//         let program = Program::default();
//         Ok(program)
//     }
// }
//
// impl Parser {
//     fn parse_program(&mut self, lexer: &mut impl Lexer) -> Program {
//         let mut program = Program::default();
//
//         while let Some(stmt) = self.parse_statement(lexer) {
//
//         }
//
//         program
//     }
//
//     fn parse_statement(&mut self, lexer: &mut impl Lexer) -> Option<Statement> {
//         let (token, location) = lexer.next_token();
//
//         let stmt = match token {
//             Token::EOF => return None,
//             Token::NEWLINE => panic!(),
//             Token::VALUE(val) => {
//                 match val.as_str() {
//                     "REPEAT" => {
//                         panic!()
//                         // self.parse_repeat_stmt(lexer)
//                     }
//                     "IF" => {
//                         panic!()
//                         // self.parse_if
//                     }
//                     "TO" => {
//                         panic!()
//                         // parse_procedure
//                     }
//                     _ => {
//                         self.parse_basic_statement(val.as_str(), lexer).unwrap();
//                     }
//                 }
//             }
//         };
//
//         panic!()
//
//         // Some(stmt)
//     }
//
//     fn parse_basic_statement(&mut self, val: &str, lexer: &mut impl Lexer) -> Option<Statement> {
//         match val {
//             "MAKE" => {
//                 // self.parse_make(lexer)
//             }
//             "FORWARD" | "BACKWARD" | "RIGHT" | "LEFT" => {
//                 // self.parse_direction(lexer)
//             }
//             _ => {
//                 // self.parse_command(val);
//             }
//         }
//
//         None
//     }
//
//     fn parse_repeat_stmt(&mut self, lexer: &mut impl Lexer) -> Statement {
//         let count_expr = self.parse_expr(lexer);
//         let block = self.parse_block_stmt(lexer);
//         Statement::Repeat(RepeatStmt { count_expr, block })
//     }
//
//     fn parse_if(&mut self, lexer: &mut impl Lexer) {
//         // let cond_expr = self.parse_expr(lexer);
//         // let true_block = parse_block_stmt(lexer);
//         // let false_block = parse_optional_block(lexer);
//         // IfStmt { cond_expr, true_block, false_block }
//     }
//
//     fn parse_block_stmt(&mut self, lexer: &mut impl Lexer) -> BlockStatement {
//         // lexer.expect_block_start() ]
//         // let mut block = BlockStatement::default();
//
//         // while let Some(stmt) = self.parse_basic_statement(lexer) {
//         //     block.add_statement(stmt);
//         // }
//
//         // lexer.expect_end() ]
//         BlockStatement::default()
//     }
//
//     fn parse_direction(&mut self, lexer: &mut impl Lexer) {
//         // let distance_expr = self.parse_expr(lexer);
//         // DirectionStmt { direction: Direction::Forward, distance_expr: distance_expr }
//     }
//
//     fn parse_make(&mut self, lexer: &mut impl Lexer) {
//         // let symbol = self.parse_symbol(lexer);
//         // let expr = self.parse_expr(lexer);
//     }
//
//     fn parse_command(&mut self, val: &str) -> CommandStmt {
//         match val {
//             "PENUP" => CommandStmt::PenUp,
//             "PENDOWN" => CommandStmt::PenDown,
//             "SHOWTURTLE" => CommandStmt::ShowTurtle,
//             "HIDETURTLE" => CommandStmt::HideTurtle,
//             _ => panic!(),
//         }
//     }
//
//     fn parse_expr(&mut self, lexer: &mut Lexer) -> Expression {
//         Expression {}
//     }
// }
