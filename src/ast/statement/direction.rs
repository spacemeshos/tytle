use crate::ast::expression::Expression;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
    SetX,
    SetY,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "FORWARD" => Direction::Forward,
            "BACKWARD" => Direction::Backward,
            "LEFT" => Direction::Left,
            "RIGHT" => Direction::Right,
            "SETX" => Direction::SetX,
            "SETY" => Direction::SetY,
            _ => panic!("Undefined direction: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionStmt {
    pub direction: Direction,
    pub expr: Expression,
}

#[cfg(test)]
mod tests {
    #[macro_use]
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
    fn set_x() {
        assert_eq!(Direction::from("SETX"), Direction::SetX);
    }

    #[test]
    fn set_y() {
        assert_eq!(Direction::from("SETY"), Direction::SetY);
    }

    #[test]
    #[should_panic]
    fn invalid() {
        Direction::from("INVALID");
    }
}
