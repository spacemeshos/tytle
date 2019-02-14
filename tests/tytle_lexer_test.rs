#[macro_use]
extern crate tytle;

use tytle::lexer::{Lexer, Location, Token, TytleLexer};

macro_rules! assert_current_token {
    ($lexer:ident, $expected_tok:expr, $expected_loc:expr) => {{
        assert_current_token!($lexer, $expected_tok);

        let (_, actual_loc) = $lexer.peek_current_token().unwrap();
        assert_eq!($expected_loc, *actual_loc);
    }};

    ($lexer:ident, $expected_tok:expr) => {{
        let (actual_tok, _) = $lexer.peek_current_token().unwrap();
        assert_eq!($expected_tok, *actual_tok);
    }};
}

macro_rules! assert_next_token {
    ($lexer:ident, $expected_tok:expr, $expected_loc:expr) => {{
        assert_next_token!($lexer, $expected_tok);

        let (_, actual_loc) = $lexer.peek_next_token().unwrap();
        assert_eq!($expected_loc, *actual_loc);
    }};

    ($lexer:ident, $expected_tok:expr) => {{
        let (actual_tok, _) = $lexer.peek_next_token().unwrap();
        assert_eq!($expected_tok, *actual_tok);
    }};
}

#[test]
fn empty() {
    let lexer = TytleLexer::new("");
    assert_current_token!(lexer, Token::EOF, Location(2, 1));
}

#[test]
fn just_spaces() {
    let mut lexer = TytleLexer::new("   ");

    // peek
    assert_current_token!(lexer, Token::EOF, Location(2, 1));

    // pop
    let (tok, loc) = lexer.pop_current_token().unwrap();
    assert_eq!(loc, Location(2, 1));
    assert_eq!(tok, Token::EOF);
}

#[test]
fn one_line_1_token() {
    let mut lexer = TytleLexer::new("111");

    // peek
    assert_current_token!(lexer, Token::VALUE("111".to_string()), Location(1, 1));
    assert_next_token!(lexer, Token::EOF, Location(2, 1));

    // pop
    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("111".to_string()));
    assert_eq!(loc2, Location(2, 1));
    assert_eq!(tok2, Token::EOF)
}

#[test]
fn one_line_1_token_with_spaces() {
    let mut lexer = TytleLexer::new(" 1  ");

    // peek
    assert_current_token!(lexer, Token::VALUE("1".to_string()), Location(1, 2));
    assert_next_token!(lexer, Token::EOF, Location(2, 1));

    // pop
    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 2));
    assert_eq!(tok1, Token::VALUE("1".to_string()));
    assert_eq!(loc2, Location(2, 1));
    assert_eq!(tok2, Token::EOF)
}

#[test]
fn one_line_2_tokens() {
    let mut lexer = TytleLexer::new("111    222");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();;
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("111".to_string()));
    assert_eq!(loc2, Location(1, 8));
    assert_eq!(tok2, Token::VALUE("222".to_string()));
    assert_eq!(loc3, Location(2, 1));
    assert_eq!(tok3, Token::EOF);
}

#[test]
fn one_line_2_tokens_many_spaces() {
    let mut lexer = TytleLexer::new("  1   2  ");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 3));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 7));
    assert_eq!(tok2, Token::VALUE("2".to_string()));

    assert_eq!(loc3, Location(2, 1));
    assert_eq!(tok3, Token::EOF)
}

#[test]
fn one_line_3_tokens() {
    let mut lexer = TytleLexer::new("1 2 3");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();
    let (tok4, loc4) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 3));
    assert_eq!(tok2, Token::VALUE("2".to_string()));

    assert_eq!(loc3, Location(1, 5));
    assert_eq!(tok3, Token::VALUE("3".to_string()));

    assert_eq!(loc4, Location(2, 1));
    assert_eq!(tok4, Token::EOF);
}

#[test]
fn two_lines() {
    let mut lexer = TytleLexer::new("1 22 \n 333 4444");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();
    let (tok4, loc4) = lexer.pop_current_token().unwrap();
    let (tok5, loc5) = lexer.pop_current_token().unwrap();
    let (tok6, loc6) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 3));
    assert_eq!(tok2, Token::VALUE("22".to_string()));

    assert_eq!(loc3, Location(1, 6));
    assert_eq!(tok3, Token::NEWLINE);

    assert_eq!(loc4, Location(2, 2));
    assert_eq!(tok4, Token::VALUE("333".to_string()));

    assert_eq!(loc5, Location(2, 6));
    assert_eq!(tok5, Token::VALUE("4444".to_string()));

    assert_eq!(loc6, Location(3, 1));
    assert_eq!(tok6, Token::EOF)
}

#[test]
fn add_op() {
    let mut lexer = TytleLexer::new("1+2");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::ADD);

    assert_eq!(loc3, Location(1, 3));
    assert_eq!(tok3, Token::VALUE("2".to_string()));
}

#[test]
fn add_op_surrounded_by_spaces() {
    let mut lexer = TytleLexer::new("1 + 2");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 3));
    assert_eq!(tok2, Token::ADD);

    assert_eq!(loc3, Location(1, 5));
    assert_eq!(tok3, Token::VALUE("2".to_string()));
}

#[test]
fn mul_op() {
    let mut lexer = TytleLexer::new("1*2");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::MUL);

    assert_eq!(loc3, Location(1, 3));
    assert_eq!(tok3, Token::VALUE("2".to_string()));
}

#[test]
fn parentheses() {
    let mut lexer = TytleLexer::new("(111)");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::LPAREN);

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::VALUE("111".to_string()));

    assert_eq!(loc3, Location(1, 5));
    assert_eq!(tok3, Token::RPAREN);
}

#[test]
fn brackets() {
    let mut lexer = TytleLexer::new("[111]");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::LBRACKET);

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::VALUE("111".to_string()));

    assert_eq!(loc3, Location(1, 5));
    assert_eq!(tok3, Token::RBRACKET);
}

#[test]
fn brackets_surrounded_by_parentheses() {
    let mut lexer = TytleLexer::new("([])");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();
    let (tok4, loc4) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::LPAREN);

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::LBRACKET);

    assert_eq!(loc3, Location(1, 3));
    assert_eq!(tok3, Token::RBRACKET);

    assert_eq!(loc4, Location(1, 4));
    assert_eq!(tok4, Token::RPAREN);
}

#[test]
fn parentheses_surrounded_by_brackets() {
    let mut lexer = TytleLexer::new("[()]");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();
    let (tok4, loc4) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::LBRACKET);

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::LPAREN);

    assert_eq!(loc3, Location(1, 3));
    assert_eq!(tok3, Token::RPAREN);

    assert_eq!(loc4, Location(1, 4));
    assert_eq!(tok4, Token::RBRACKET);
}

#[test]
fn assign_an_int_expr() {
    let mut lexer = TytleLexer::new("MYVAR=10");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("MYVAR".to_string()));

    assert_eq!(loc2, Location(1, 6));
    assert_eq!(tok2, Token::ASSIGN);

    assert_eq!(loc3, Location(1, 7));
    assert_eq!(tok3, Token::VALUE("10".to_string()));
}

#[test]
fn assign_a_composite_expr() {
    let mut lexer = TytleLexer::new("MYVAR=(1+2)");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();
    let (tok4, loc4) = lexer.pop_current_token().unwrap();
    let (tok5, loc5) = lexer.pop_current_token().unwrap();
    let (tok6, loc6) = lexer.pop_current_token().unwrap();
    let (tok7, loc7) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("MYVAR".to_string()));

    assert_eq!(loc2, Location(1, 6));
    assert_eq!(tok2, Token::ASSIGN);

    assert_eq!(loc3, Location(1, 7));
    assert_eq!(tok3, Token::LPAREN);

    assert_eq!(loc4, Location(1, 8));
    assert_eq!(tok4, Token::VALUE("1".to_string()));

    assert_eq!(loc5, Location(1, 9));
    assert_eq!(tok5, Token::ADD);

    assert_eq!(loc6, Location(1, 10));
    assert_eq!(tok6, Token::VALUE("2".to_string()));

    assert_eq!(loc7, Location(1, 11));
    assert_eq!(tok7, Token::RPAREN);
}

#[test]
fn less_than_expr() {
    let mut lexer = TytleLexer::new("1<2");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::LT);

    assert_eq!(loc3, Location(1, 3));
    assert_eq!(tok3, Token::VALUE("2".to_string()));
}

#[test]
fn greater_than_expr() {
    let mut lexer = TytleLexer::new("1>2");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("1".to_string()));

    assert_eq!(loc2, Location(1, 2));
    assert_eq!(tok2, Token::GT);

    assert_eq!(loc3, Location(1, 3));
    assert_eq!(tok3, Token::VALUE("2".to_string()));
}

#[test]
fn procedure_call_expr() {
    let mut lexer = TytleLexer::new("FOO(:X, 10, 1 + 2)");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();
    let (tok4, loc4) = lexer.pop_current_token().unwrap();
    let (tok5, loc5) = lexer.pop_current_token().unwrap();
    let (tok6, loc6) = lexer.pop_current_token().unwrap();
    let (tok7, loc7) = lexer.pop_current_token().unwrap();
    let (tok8, loc8) = lexer.pop_current_token().unwrap();
    let (tok9, loc9) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("FOO".to_string()));

    assert_eq!(loc2, Location(1, 4));
    assert_eq!(tok2, Token::LPAREN);

    assert_eq!(loc3, Location(1, 5));
    assert_eq!(tok3, Token::VALUE(":X".to_string()));

    assert_eq!(loc4, Location(1, 7));
    assert_eq!(tok4, Token::COMMA);

    assert_eq!(loc5, Location(1, 9));
    assert_eq!(tok5, Token::VALUE("10".to_string()));

    assert_eq!(loc6, Location(1, 11));
    assert_eq!(tok6, Token::COMMA);

    assert_eq!(loc7, Location(1, 13));
    assert_eq!(tok7, Token::VALUE("1".to_string()));

    assert_eq!(loc8, Location(1, 15));
    assert_eq!(tok8, Token::ADD);

    assert_eq!(loc9, Location(1, 17));
    assert_eq!(tok9, Token::VALUE("2".to_string()));
}
