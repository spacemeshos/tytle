mod location;
mod token;
mod tytle_lexer;

pub trait Lexer {
    fn buffer_more_tokens(&mut self);
    fn peek_current_token(&self) -> Option<&(Token, Location)>;
    fn peek_next_token(&self) -> Option<&(Token, Location)>;
    fn pop_current_token(&mut self) -> Option<(Token, Location)>;
}

pub use location::Location;
pub use token::Token;
pub use tytle_lexer::TytleLexer;
