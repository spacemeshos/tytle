#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    NEWLINE,
    VALUE(String),
}
