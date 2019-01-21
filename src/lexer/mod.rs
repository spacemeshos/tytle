mod token;

use token::Token;

use std::str::Chars;

pub trait Lexer {
    fn next_token(&mut self) -> Token;
}

struct SimpleLexer<'a> {
    code_size: usize,
    code_chars: Chars<'a>,
    buffer: Vec<char>,
}

impl<'a> SimpleLexer<'a> {
    fn new(code: &'a str) -> Self {
        Self {
            code_chars: code.chars(),
            code_size: code.len(),
            buffer: Vec::new(),
        }
    }
}

impl<'a> Lexer for SimpleLexer<'a> {
    fn next_token(&mut self) -> Token {
        let mut token = Vec::new();

        // let mut ch_opt = self.code_chars.

        while let Some(ch) = self.code_chars.next() {
            match ch {
                ' ' | '\n' => match token.len() {
                    0 => continue,
                    _ => return Token::VALUE(token.iter().collect()),
                },
                _ => token.push(ch),
            }
        }

        match token.len() {
            0 => Token::EOF,
            _ => Token::VALUE(token.iter().collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::token::Token;
    use super::Lexer;
    use super::SimpleLexer;

    #[test]
    fn empty() {
        let mut lexer = SimpleLexer::new("");
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
        let mut lexer = SimpleLexer::new("1 2 \n 3 4");

        let tok1 = lexer.next_token();
        let tok2 = lexer.next_token();
        let tok3 = lexer.next_token();
        let tok4 = lexer.next_token();
        let tok5 = lexer.next_token();

        assert_eq!(tok1, Token::VALUE("1".to_string()));
        assert_eq!(tok2, Token::VALUE("2".to_string()));
        assert_eq!(tok3, Token::VALUE("3".to_string()));

        assert_eq!(tok4, Token::VALUE("4".to_string()));
        assert_eq!(tok5, Token::EOF)
    }
}
