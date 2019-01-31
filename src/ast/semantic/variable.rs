#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub global: bool,
    pub name: String,
}
