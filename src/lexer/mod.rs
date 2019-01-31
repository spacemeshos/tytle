mod location;
mod logos_lexer;
mod token;

pub trait Lexer {
    fn buffer_more_tokens(&mut self);
    fn peek_current_token(&self) -> Option<&(Token, Location)>;
    fn peek_next_token(&self) -> Option<&(Token, Location)>;
    fn pop_current_token(&mut self) -> Option<(Token, Location)>;
}

pub use location::Location;
pub use logos_lexer::LogosLexer;
pub use token::Token;
