extern crate logos;

use logos::lexer::simple_lexer::SimpleLexer;
use logos::lexer::{Lexer, Location, Token};

#[test]
fn empty() {
    let lexer = SimpleLexer::new("");

    let (tok, loc) = lexer.peek_current_token().unwrap();

    assert_eq!(*tok, Token::EOF);
    assert_eq!(*loc, Location(2, 1));
}

#[test]
fn just_spaces() {
    let mut lexer = SimpleLexer::new("   ");

    // peek
    let (tok, loc) = lexer.peek_current_token().unwrap();
    assert_eq!(*loc, Location(2, 1));
    assert_eq!(*tok, Token::EOF);

    // pop
    let (tok, loc) = lexer.pop_current_token().unwrap();
    assert_eq!(loc, Location(2, 1));
    assert_eq!(tok, Token::EOF);
}

#[test]
fn one_line_1_token() {
    let mut lexer = SimpleLexer::new("111");

    // peek
    let (tok1, loc1) = lexer.peek_current_token().unwrap();
    let (tok2, loc2) = lexer.peek_next_token().unwrap();

    assert_eq!(*loc1, Location(1, 1));
    assert_eq!(*tok1, Token::VALUE("111".to_string()));
    assert_eq!(*loc2, Location(2, 1));
    assert_eq!(*tok2, Token::EOF);

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
    let mut lexer = SimpleLexer::new(" 1  ");

    // peek
    let (tok1, loc1) = lexer.peek_current_token().unwrap();
    let (tok2, loc2) = lexer.peek_next_token().unwrap();

    assert_eq!(*loc1, Location(1, 2));
    assert_eq!(*tok1, Token::VALUE("1".to_string()));
    assert_eq!(*loc2, Location(2, 1));
    assert_eq!(*tok2, Token::EOF);

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
    let mut lexer = SimpleLexer::new("111    222");

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
    let mut lexer = SimpleLexer::new("  1   2  ");

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
    let mut lexer = SimpleLexer::new("1 2 3");

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
    let mut lexer = SimpleLexer::new("1 22 \n 333 4444");

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
    let mut lexer = SimpleLexer::new("1+2");

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
    let mut lexer = SimpleLexer::new("1 + 2");

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
    let mut lexer = SimpleLexer::new("1*2");

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
    let mut lexer = SimpleLexer::new("(111)");

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
    let mut lexer = SimpleLexer::new("[111]");

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
    let mut lexer = SimpleLexer::new("([])");

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
    let mut lexer = SimpleLexer::new("[()]");

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
    let mut lexer = SimpleLexer::new("MyVar=10");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("MyVar".to_string()));

    assert_eq!(loc2, Location(1, 6));
    assert_eq!(tok2, Token::ASSIGN);

    assert_eq!(loc3, Location(1, 7));
    assert_eq!(tok3, Token::VALUE("10".to_string()));
}

#[test]
fn assign_a_composite_expr() {
    let mut lexer = SimpleLexer::new("MyVar=(1+2)");

    let (tok1, loc1) = lexer.pop_current_token().unwrap();
    let (tok2, loc2) = lexer.pop_current_token().unwrap();
    let (tok3, loc3) = lexer.pop_current_token().unwrap();
    let (tok4, loc4) = lexer.pop_current_token().unwrap();
    let (tok5, loc5) = lexer.pop_current_token().unwrap();
    let (tok6, loc6) = lexer.pop_current_token().unwrap();
    let (tok7, loc7) = lexer.pop_current_token().unwrap();

    assert_eq!(loc1, Location(1, 1));
    assert_eq!(tok1, Token::VALUE("MyVar".to_string()));

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
    let mut lexer = SimpleLexer::new("1<2");

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
    let mut lexer = SimpleLexer::new("1>2");

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
