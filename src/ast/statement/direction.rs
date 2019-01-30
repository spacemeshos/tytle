#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "FORWARD" => Direction::Forward,
            "BACKWARD" => Direction::Backward,
            "LEFT" => Direction::Left,
            "RIGHT" => Direction::Right,
            _ => panic!("Undefined direction: {}", s),
        }
    }
}

use crate::ast::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionStmt {
    pub direction: Direction,
    pub expr: Expression,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward() {
        assert_eq!(Direction::from("FORWARD"), Direction::Forward);
    }

    #[test]
    fn backward() {
        assert_eq!(Direction::from("BACKWARD"), Direction::Backward);
    }

    #[test]
    fn left() {
        assert_eq!(Direction::from("LEFT"), Direction::Left);
    }

    #[test]
    fn right() {
        assert_eq!(Direction::from("RIGHT"), Direction::Right);
    }

    #[test]
    #[should_panic]
    fn invalid() {
        Direction::from("INVALID");
    }
}
