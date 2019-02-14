#[derive(Debug, Clone, PartialEq)]
pub enum AstWalkError {
    DuplicateGlobalVar(String),
    DuplicateProc(String),
    MissingVarDeclaration(String),
    Custom { message: String },
}

impl AstWalkError {
    pub fn new(message: &str) -> Self {
        AstWalkError::Custom {
            message: message.to_owned(),
        }
    }
}
