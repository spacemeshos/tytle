use std::default::Default;
use std::fmt;

#[derive(PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 0, column: 0 }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}:{}]", self.row, self.column)
    }
}
