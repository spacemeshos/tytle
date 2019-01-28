pub mod location;
pub mod token;

use location::Location;
use token::Token;

pub trait Lexer {
    fn buffer_more_tokens(&mut self);
    fn peek_current_token(&self) -> Option<&(Token, Location)>;
    fn peek_next_token(&self) -> Option<&(Token, Location)>;
    fn pop_current_token(&mut self) -> Option<(Token, Location)>;
}

pub mod simple_lexer;
