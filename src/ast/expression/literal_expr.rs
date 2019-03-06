#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExpr {
    Bool(bool),
    Int(usize),
    Str(String),
    Var(String, Option<u64>),
}
