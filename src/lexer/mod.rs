pub mod token;

use token::Token;

pub trait Lexer {
    fn next_token(&mut self) -> Token;
}

pub mod simple_lexer;
