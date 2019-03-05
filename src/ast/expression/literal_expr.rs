#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExpr {
    Bool(bool),
    Int(usize),
    Var(String, Option<u64>),
    Str(String),
}
