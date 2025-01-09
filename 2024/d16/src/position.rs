use crate::direction::Direction;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Position(pub usize, pub usize, pub Direction);
