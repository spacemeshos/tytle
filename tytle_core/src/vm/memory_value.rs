#[derive(Debug, Clone, PartialEq)]
pub enum MemoryValue {
    Int(isize),
    Bool(bool),
    Str(String),
}
