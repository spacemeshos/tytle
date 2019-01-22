mod token;

use token::Token;

pub trait Lexer {
    fn next_token(&mut self) -> Token;
}

mod simple_lexer;
