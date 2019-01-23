pub mod location;
pub mod token;

use location::Location;
use token::Token;

pub trait Lexer {
    fn next_token(&mut self) -> (Token, Location);
}

pub mod simple_lexer;
