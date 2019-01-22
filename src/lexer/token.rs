#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    VALUE(String),
}
