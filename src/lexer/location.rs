use std::default::Default;
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub struct Location(pub usize, pub usize);

impl Default for Location {
    fn default() -> Self {
        Self(1, 1)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}:{}]", self.line(), self.column())
    }
}

impl Location {
    pub fn next_line(&mut self) {
        self.0 += 1;
        self.1 = 1;
    }

    pub fn increment_column(&mut self) {
        self.1 += 1;
    }

    pub fn line(&self) -> usize {
        self.0
    }

    pub fn column(&self) -> usize {
        self.1
    }
}
