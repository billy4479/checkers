use std::fmt::Display;

use crate::board::CheckerError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Coordinate {
    pub fn check(&self) -> Result<(), CheckerError> {
        if self.x >= 8 || self.y >= 8 {
            Err(CheckerError::InvalidCoordinate(*self))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct MoveInfo {
    pub from: Coordinate,
    pub to: Coordinate,

    pub capturing: Vec<Coordinate>,
}

impl Display for MoveInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self.from, self.to)?;

        if !self.capturing.is_empty() {
            write!(f, " (capturing ")?;
            for (i, captures) in self.capturing.iter().enumerate() {
                write!(f, "{captures}")?;
                if i != self.capturing.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, ")")?;
        }

        Ok(())
    }
}
