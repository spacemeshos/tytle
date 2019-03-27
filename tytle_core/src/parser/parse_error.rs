use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingColon,
    NewLineExpected,
    IdentifierExpected,
    MissingProcReturnType,
    InvalidDataType(String),
    InvalidIdentifierDeclaration(String),
    UnexpectedToken { expected: Token, actual: Token },
    UnexpectedKeyword { keyword: String },
    ReservedKeyword(String),
    Syntax { message: String },
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match *self {
            ParseError::MissingColon => "Missing colon".to_string(),
            ParseError::NewLineExpected => "New line expected".to_string(),
            ParseError::IdentifierExpected => "Indentifier expected".to_string(),
            ParseError::MissingProcReturnType => "Procedure is missing a return type".to_string(),
            ParseError::InvalidDataType(ref dt) => format!("Invalid data type: `{}`", dt),
            ParseError::InvalidIdentifierDeclaration(ref ident) => {
                format!("Invalid indentifier declaration: `{}`", ident)
            }
            ParseError::UnexpectedToken {
                ref expected,
                ref actual,
            } => format!(
                "Unexpected token: `{}` (expected `{}`)",
                actual.to_string(),
                expected.to_string()
            ),
            ParseError::UnexpectedKeyword { ref keyword } => {
                format!("Unexpected keyword: `{}`", keyword)
            }
            ParseError::ReservedKeyword(ref kw) => format!("Reserved keyword: `{}`", kw),
            ParseError::Syntax { ref message } => format!("Syntax error: `{}`", message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parse_err(expected: &str, err: ParseError) {
        assert_eq!(expected, ParseError::to_string(&err));
    }

    #[test]
    pub fn parse_error_missing_colon() {
        assert_parse_err("Missing colon", ParseError::MissingColon);
    }

    #[test]
    pub fn parse_error_new_line_expected() {
        assert_parse_err("New line expected", ParseError::NewLineExpected);
    }

    #[test]
    pub fn parse_error_iden_expected() {
        assert_parse_err("Indentifier expected", ParseError::IdentifierExpected);
    }

    #[test]
    pub fn parse_error_proc_missing_return_type() {
        assert_parse_err(
            "Procedure is missing a return type",
            ParseError::MissingProcReturnType,
        );
    }

    #[test]
    pub fn parse_error_invalid_data_type() {
        assert_parse_err(
            "Invalid data type: `FOO`",
            ParseError::InvalidDataType("FOO".to_string()),
        );
    }

    #[test]
    pub fn parse_error_invalid_ident_declare() {
        assert_parse_err(
            "Invalid indentifier declaration: `FOO`",
            ParseError::InvalidIdentifierDeclaration("FOO".to_string()),
        );
    }

    #[test]
    pub fn parse_error_unexpected_token() {
        assert_parse_err(
            "Unexpected token: `+` (expected `*`)",
            ParseError::UnexpectedToken {
                expected: Token::MUL,
                actual: Token::ADD,
            },
        );
    }

    #[test]
    pub fn parse_error_unexpected_keyword() {
        assert_parse_err(
            "Unexpected keyword: `PUBLIC`",
            ParseError::UnexpectedKeyword {
                keyword: "PUBLIC".to_string(),
            },
        );
    }

    #[test]
    pub fn parse_error_reserved_keyword() {
        assert_parse_err(
            "Reserved keyword: `TO`",
            ParseError::ReservedKeyword("TO".to_string()),
        );
    }

    #[test]
    pub fn parse_error_syntax() {
        assert_parse_err(
            "Syntax error: `bla`",
            ParseError::Syntax {
                message: "bla".to_string(),
            },
        );
    }
}
