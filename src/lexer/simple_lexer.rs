use super::token::Token;
use crate::lexer::Lexer;

use std::str::Chars;

pub struct SimpleLexer<'a> {
    code_size: usize,
    code_chars: Chars<'a>,
    tokens_buffer: Vec<Token>,
}

impl<'a> SimpleLexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code_chars: code.chars(),
            code_size: code.len(),
            tokens_buffer: Default::default(),
        }
    }
}

impl<'a> Lexer for SimpleLexer<'a> {
    fn next_token(&mut self) -> Token {
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
                    0 => continue,
                    _ => {
                        self.buffer_token(&token);
                        return;
                    }
                },
                _ => token.push(ch),
            }
        }

        self.buffer_token(&token);
        self.buffer_eof();
    }

    fn buffer_newline(&mut self) {
        self.tokens_buffer.insert(0, Token::NEWLINE);
    }

    fn buffer_eof(&mut self) {
        self.tokens_buffer.insert(0, Token::EOF);
    }

    fn buffer_token(&mut self, token_chars: &Vec<char>) {
        if token_chars.len() > 0 {
            let value = token_chars.iter().collect();
            self.tokens_buffer.insert(0, Token::VALUE(value));
        }
    }

    fn get_next_buffered_token(&mut self) -> Token {
        self.tokens_buffer.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{simple_lexer::SimpleLexer, token::Token, Lexer};

    #[test]
    fn empty() {
        let mut lexer = SimpleLexer::new("");
        let tok = lexer.next_token();
        assert_eq!(tok, Token::EOF)
    }

    #[test]
    fn just_spaces() {
        let mut lexer = SimpleLexer::new("   ");
        let tok = lexer.next_token();
        assert_eq!(tok, Token::EOF)
    }

    #[test]
    fn one_line_1_token() {
        let mut lexer = SimpleLexer::new("1");

        let tok1 = lexer.next_token();
        let tok2 = lexer.next_token();

        assert_eq!(tok1, Token::VALUE("1".to_string()));
        assert_eq!(tok2, Token::EOF)
    }

    #[test]
    fn one_line_1_token_with_spaces() {
        let mut lexer = SimpleLexer::new(" 1  ");

        let tok1 = lexer.next_token();
        let tok2 = lexer.next_token();

        assert_eq!(tok1, Token::VALUE("1".to_string()));
        assert_eq!(tok2, Token::EOF)
    }

    #[test]
    fn one_line_2_tokens() {
        let mut lexer = SimpleLexer::new("1 2");

        let tok1 = lexer.next_token();
        let tok2 = lexer.next_token();
        let tok3 = lexer.next_token();

        assert_eq!(tok1, Token::VALUE("1".to_string()));
        assert_eq!(tok2, Token::VALUE("2".to_string()));
        assert_eq!(tok3, Token::EOF)
    }

    #[test]
    fn one_line_2_tokens_many_spaces() {
        let mut lexer = SimpleLexer::new("  1   2  ");

        let tok1 = lexer.next_token();
        let tok2 = lexer.next_token();
        let tok3 = lexer.next_token();

        assert_eq!(tok1, Token::VALUE("1".to_string()));
        assert_eq!(tok2, Token::VALUE("2".to_string()));
        assert_eq!(tok3, Token::EOF)
    }

    #[test]
    fn one_line_3_tokens() {
        let mut lexer = SimpleLexer::new("1 2 3");

        let tok1 = lexer.next_token();
        let tok2 = lexer.next_token();
        let tok3 = lexer.next_token();
        let tok4 = lexer.next_token();

        assert_eq!(tok1, Token::VALUE("1".to_string()));
        assert_eq!(tok2, Token::VALUE("2".to_string()));
        assert_eq!(tok3, Token::VALUE("3".to_string()));
        assert_eq!(tok4, Token::EOF)
    }

    #[test]
    fn two_lines() {
        let mut lexer = SimpleLexer::new("1 22 \n 333 4444");

        let tok1 = lexer.next_token();
        let tok2 = lexer.next_token();
        let tok3 = lexer.next_token();
        let tok4 = lexer.next_token();
        let tok5 = lexer.next_token();
        let tok6 = lexer.next_token();

        assert_eq!(tok1, Token::VALUE("1".to_string()));
        assert_eq!(tok2, Token::VALUE("22".to_string()));
        assert_eq!(tok3, Token::NEWLINE);
        // assert_eq!(tok4, Token::VALUE("333".to_string()));
        //
        // assert_eq!(tok5, Token::VALUE("4444".to_string()));
        // assert_eq!(tok6, Token::EOF)
    }
}
