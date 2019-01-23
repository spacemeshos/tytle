use super::location::Location;
use super::token::Token;
use crate::lexer::Lexer;

use std::str::Chars;

pub struct SimpleLexer<'a> {
    code_size: usize,
    code_chars: Chars<'a>,
    location: Location,
    tokens_buffer: Vec<(Token, Location)>,
}

impl<'a> SimpleLexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            location: Location::default(),
            code_chars: code.chars(),
            code_size: code.len(),
            tokens_buffer: Default::default(),
        }
    }
}

impl<'a> Lexer for SimpleLexer<'a> {
    fn next_token(&mut self) -> (Token, Location) {
        if self.tokens_buffer.len() == 0 {
            self.collect_more_tokens();
        }

        self.get_next_buffered_token()
    }
}

impl<'a> SimpleLexer<'a> {
    fn collect_more_tokens(&mut self) {
        let mut token = Vec::new();

        while let Some(ch) = self.code_chars.next() {
            match ch {
                '\n' => {
                    self.buffer_token(&token);
                    self.buffer_newline();
                    return;
                }
                ' ' => match token.len() {
                    0 => {
                        self.location.increment_column();
                        continue;
                    }
                    _ => {
                        self.buffer_token(&token);
                        self.location.increment_column();
                        return;
                    }
                },
                _ => {
                    self.location.increment_column();
                    token.push(ch);
                }
            }
        }

        self.buffer_token(&token);
        self.buffer_eof();
    }

    fn buffer_newline(&mut self) {
        self.tokens_buffer
            .insert(0, (Token::NEWLINE, self.location));

        self.location.next_line();
    }

    fn buffer_eof(&mut self) {
        self.location.next_line();

        self.tokens_buffer.insert(0, (Token::EOF, self.location));
    }

    fn buffer_token(&mut self, token_chars: &Vec<char>) {
        if token_chars.len() > 0 {
            let value = token_chars.iter().collect();

            let loc = Location(
                self.location.line(),
                self.location.column() - token_chars.len(),
            );

            let entry = (Token::VALUE(value), loc);
            self.tokens_buffer.insert(0, entry);
        }
    }

    fn get_next_buffered_token(&mut self) -> (Token, Location) {
        self.tokens_buffer.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{location::Location, simple_lexer::SimpleLexer, token::Token, Lexer};

    #[test]
    fn empty() {
        let mut lexer = SimpleLexer::new("");
        let (tok, loc) = lexer.next_token();

        assert_eq!(tok, Token::EOF);
        assert_eq!(loc, Location(2, 1));
    }

    #[test]
    fn just_spaces() {
        let mut lexer = SimpleLexer::new("   ");
        let (tok, loc) = lexer.next_token();

        assert_eq!(loc, Location(2, 1));
        assert_eq!(tok, Token::EOF)
    }

    #[test]
    fn one_line_1_token() {
        let mut lexer = SimpleLexer::new("1");

        let (tok1, loc1) = lexer.next_token();
        let (tok2, loc2) = lexer.next_token();

        assert_eq!(loc1, Location(1, 1));
        assert_eq!(tok1, Token::VALUE("1".to_string()));

        assert_eq!(loc2, Location(2, 1));
        assert_eq!(tok2, Token::EOF)
    }

    #[test]
    fn one_line_1_token_with_spaces() {
        let mut lexer = SimpleLexer::new(" 1  ");

        let (tok1, loc1) = lexer.next_token();
        let (tok2, loc2) = lexer.next_token();

        assert_eq!(loc1, Location(1, 2));
        assert_eq!(tok1, Token::VALUE("1".to_string()));

        assert_eq!(loc2, Location(2, 1));
        assert_eq!(tok2, Token::EOF)
    }

    #[test]
    fn one_line_2_tokens() {
        let mut lexer = SimpleLexer::new("111    222");

        let (tok1, loc1) = lexer.next_token();
        let (tok2, loc2) = lexer.next_token();
        let (tok3, loc3) = lexer.next_token();

        assert_eq!(loc1, Location(1, 1));
        assert_eq!(tok1, Token::VALUE("111".to_string()));

        assert_eq!(loc2, Location(1, 8));
        assert_eq!(tok2, Token::VALUE("222".to_string()));

        assert_eq!(loc3, Location(2, 1));
        assert_eq!(tok3, Token::EOF)
    }

    #[test]
    fn one_line_2_tokens_many_spaces() {
        let mut lexer = SimpleLexer::new("  1   2  ");

        let (tok1, loc1) = lexer.next_token();
        let (tok2, loc2) = lexer.next_token();
        let (tok3, loc3) = lexer.next_token();

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

        let (tok1, loc1) = lexer.next_token();
        let (tok2, loc2) = lexer.next_token();
        let (tok3, loc3) = lexer.next_token();
        let (tok4, loc4) = lexer.next_token();

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

        let (tok1, loc1) = lexer.next_token();
        let (tok2, loc2) = lexer.next_token();
        let (tok3, loc3) = lexer.next_token();
        let (tok4, loc4) = lexer.next_token();
        let (tok5, loc5) = lexer.next_token();
        let (tok6, loc6) = lexer.next_token();

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
}
