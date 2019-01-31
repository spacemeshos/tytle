#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOF,
    NEWLINE,

    MUL,
    ADD,

    LPAREN, // (
    RPAREN, // )

    LBRACKET, // [
    RBRACKET, // ]

    ASSIGN, // =

    LT, // <
    GT, // >

    VALUE(String),
}
