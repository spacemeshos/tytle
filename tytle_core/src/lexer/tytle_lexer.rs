use super::location::Location;
use super::token::Token;
use crate::lexer::Lexer;

use std::collections::VecDeque;

use std::str::Chars;

pub struct TytleLexer<'lex> {
    code_chars: Chars<'lex>,
    location: Location,
    reached_eof: bool,
    tokens_buffer: VecDeque<(Token, Location)>,
}

impl<'lex> TytleLexer<'lex> {
    pub fn new(code: &'lex str) -> Self {
        let mut lexer = Self {
            location: Location::default(),
            code_chars: code.chars(),
            reached_eof: false,
            tokens_buffer: Default::default(),
        };

        lexer.buffer_more_tokens();

        lexer
    }
}

impl<'lex> Lexer for TytleLexer<'lex> {
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
                    ':' => {
                        self.push_token(&mut token);
                        self.push_colon();
                        self.location.increment_column();
                        break;
                    }
                    ',' => {
                        self.push_token(&mut token);
                        self.push_comma();
                        self.location.increment_column();
                        break;
                    }
                    '>' => {
                        self.push_token(&mut token);
                        self.push_greater_than();
                        self.location.increment_column();
                        break;
                    }
                    '<' => {
                        self.push_token(&mut token);
                        self.push_less_than();
                        self.location.increment_column();
                        break;
                    }
                    '+' | '*' | '/' => {
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

impl<'lex> TytleLexer<'lex> {
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
            let value = token_chars.iter().collect::<String>();

            let loc = Location(
                self.location.line(),
                self.location.column() - token_chars.len(),
            );

            let token = match value.as_str() {
                "AND" => Token::AND,
                "OR" => Token::OR,
                "NOT" => Token::NOT,
                _ => Token::VALUE(value),
            };

            let entry = (token, loc);

            self.tokens_buffer.push_back(entry);
        }

        token_chars.clear();
    }

    fn push_newline(&mut self) {
        self.tokens_buffer
            .push_back((Token::NEWLINE, self.location));

        self.location.next_line();
    }

    fn push_op(&mut self, op: char) {
        let token = match op {
            '+' => Token::ADD,
            '*' => Token::MUL,
            '/' => Token::DIV,
            _ => panic!(),
        };
        self.tokens_buffer.push_back((token, self.location));
    }

    fn push_comma(&mut self) {
        self.tokens_buffer.push_back((Token::COMMA, self.location));
    }

    fn push_assign(&mut self) {
        self.tokens_buffer.push_back((Token::ASSIGN, self.location));
    }

    fn push_colon(&mut self) {
        self.tokens_buffer.push_back((Token::COLON, self.location));
    }

    fn push_less_than(&mut self) {
        self.tokens_buffer.push_back((Token::LT, self.location));
    }

    fn push_greater_than(&mut self) {
        self.tokens_buffer.push_back((Token::GT, self.location));
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
