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
    COMMA,  // ,

    LT, // <
    GT, // >

    COLON, // :

    AND, // `AND`
    OR,  // `OR`
    NOT, // `NOT`

    VALUE(String),
}
