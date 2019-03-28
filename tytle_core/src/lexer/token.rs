#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOF,
    NEWLINE,

    MUL,
    ADD,
    DIV,

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

impl ToString for Token {
    fn to_string(&self) -> String {
        let s = match self {
            Token::EOF => "End of file",
            Token::NEWLINE => "\n",
            Token::MUL => "*",
            Token::ADD => "+",
            Token::DIV => "/",
            Token::LPAREN => "(",
            Token::RPAREN => ")",
            Token::LBRACKET => "[",
            Token::RBRACKET => "]",
            Token::ASSIGN => "=",
            Token::COMMA => ",",
            Token::LT => "<",
            Token::GT => ">",
            Token::COLON => ":",
            Token::AND => "AND",
            Token::OR => "OR",
            Token::NOT => "NOT",
            Token::VALUE(s) => s,
        };

        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token(expected: &str, actual: Token) {
        assert_eq!(expected, actual.to_string());
    }

    #[test]
    pub fn token_eof() {
        assert_token("End of file", Token::EOF);
    }

    #[test]
    pub fn token_newline() {
        assert_token("\n", Token::NEWLINE);
    }

    #[test]
    pub fn token_mul() {
        assert_token("*", Token::MUL);
    }

    #[test]
    pub fn token_add() {
        assert_token("+", Token::ADD);
    }

    #[test]
    pub fn token_div() {
        assert_token("/", Token::DIV);
    }

    #[test]
    pub fn token_lparen() {
        assert_token("(", Token::LPAREN);
    }

    #[test]
    pub fn token_rparen() {
        assert_token(")", Token::RPAREN);
    }

    #[test]
    pub fn token_lbracket() {
        assert_token("[", Token::LBRACKET);
    }

    #[test]
    pub fn token_rbracket() {
        assert_token("]", Token::RBRACKET);
    }

    #[test]
    pub fn token_assign() {
        assert_token("=", Token::ASSIGN);
    }

    #[test]
    pub fn token_comma() {
        assert_token(",", Token::COMMA);
    }

    #[test]
    pub fn token_lt() {
        assert_token("<", Token::LT);
    }

    #[test]
    pub fn token_gt() {
        assert_token(">", Token::GT);
    }

    #[test]
    pub fn token_colon() {
        assert_token(":", Token::COLON);
    }

    #[test]
    pub fn token_and() {
        assert_token("AND", Token::AND);
    }

    #[test]
    pub fn token_or() {
        assert_token("OR", Token::OR);
    }

    #[test]
    pub fn token_not() {
        assert_token("NOT", Token::NOT);
    }

    #[test]
    pub fn token_value() {
        assert_token("ABC", Token::VALUE("ABC".to_string()));
    }
}
