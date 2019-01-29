#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    NEWLINE,

    MUL,
    ADD,

    LPAREN, // (
    RPAREN, // )

    LBRACKET, // [
    RBRACKET, // ]

    VALUE(String),
}
