use super::location::Location;
use super::token::Token;
use crate::lexer::Lexer;

use std::collections::VecDeque;

use std::str::Chars;

pub struct SimpleLexer<'a> {
    code_size: usize,
    code_chars: Chars<'a>,
    location: Location,
    reached_eof: bool,
    tokens_buffer: VecDeque<(Token, Location)>,
}

impl<'a> SimpleLexer<'a> {
    pub fn new(code: &'a str) -> Self {
        let mut lexer = Self {
            location: Location::default(),
            code_chars: code.chars(),
            code_size: code.len(),
            reached_eof: false,
            tokens_buffer: Default::default(),
        };

        lexer.buffer_more_tokens();

        lexer
    }
}

impl<'a> Lexer for SimpleLexer<'a> {
    fn peek_current_token(&self) -> Option<&(Token, Location)> {
        self.peek_nth_token(0)
    }

    fn peek_next_token(&self) -> Option<&(Token, Location)> {
        self.peek_nth_token(1)
    }

    fn pop_current_token(&mut self) -> Option<(Token, Location)> {
        self.buffer_more_tokens();

        self.tokens_buffer.pop_front()
    }

    fn buffer_more_tokens(&mut self) {
        if self.reached_eof {
            return;
        }

        let mut token = Vec::new();

        for _ in 1..5 {
            loop {
                let ch_opt = self.code_chars.next();

                if ch_opt.is_none() {
                    self.push_token(&mut token);
                    self.push_eof();
                    return;
                }

                let ch = ch_opt.unwrap();

                match ch {
                    '\n' => {
                        self.push_token(&mut token);
                        self.push_newline();
                        break;
                    }
                    '=' => {
                        self.push_token(&mut token);
                        self.push_assign();
                        self.location.increment_column();
                        break;
                    }
                    '+' | '*' => {
                        self.push_token(&mut token);
                        self.push_op(ch);
                        self.location.increment_column();
                        break;
                    }
                    '(' | ')' | '[' | ']' => {
                        self.push_token(&mut token);
                        self.push_bracket(ch);
                        self.location.increment_column();
                        break;
                    }
                    ' ' => match token.len() {
                        0 => {
                            self.location.increment_column();
                            continue;
                        }
                        _ => {
                            self.push_token(&mut token);
                            self.location.increment_column();
                            break;
                        }
                    },
                    _ => {
                        self.location.increment_column();
                        token.push(ch);
                    }
                }
            }
        }
    }
}

impl<'a> SimpleLexer<'a> {
    fn peek_nth_token(&self, nth: usize) -> Option<&(Token, Location)> {
        if self.tokens_buffer.len() > nth {
            self.tokens_buffer.get(nth)
        } else {
            if self.reached_eof {
                None
            } else {
                panic!("missing call to `buffer_more_tokens`")
            }
        }
    }

    fn push_token(&mut self, token_chars: &mut Vec<char>) {
        if token_chars.len() > 0 {
            let value = token_chars.iter().collect();

            let loc = Location(
                self.location.line(),
                self.location.column() - token_chars.len(),
            );

            let entry = (Token::VALUE(value), loc);

            self.tokens_buffer.push_back(entry);
        }

        token_chars.clear();
    }

    fn push_newline(&mut self) {
        self.tokens_buffer
            .push_back((Token::NEWLINE, self.location));

        self.location.next_line();
    }

    fn push_mul(&mut self) {
        self.tokens_buffer.push_back((Token::MUL, self.location));
    }

    fn push_op(&mut self, op: char) {
        let token = match op {
            '+' => Token::ADD,
            '*' => Token::MUL,
            _ => panic!(),
        };
        self.tokens_buffer.push_back((token, self.location));
    }

    fn push_assign(&mut self) {
        self.tokens_buffer.push_back((Token::ASSIGN, self.location));
    }

    fn push_bracket(&mut self, op: char) {
        let token = match op {
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '[' => Token::LBRACKET,
            ']' => Token::RBRACKET,
            _ => panic!(),
        };
        self.tokens_buffer.push_back((token, self.location));
    }

    fn push_eof(&mut self) {
        self.location.next_line();

        self.tokens_buffer.push_back((Token::EOF, self.location));
        self.reached_eof = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut lexer = SimpleLexer::new("");

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
}
