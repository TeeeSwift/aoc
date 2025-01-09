#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::{Down, Left, Right, Up};

impl Direction {
    pub fn get_vectors(&self) -> Vec<(isize, isize, Direction)> {
        match self {
            Right => vec![(-1, 0, Up), (0, 1, Right), (1, 0, Down)],
            Down => vec![(0, -1, Left), (0, 1, Right), (1, 0, Down)],
            Left => vec![(1, 0, Down), (0, -1, Left), (-1, 0, Up)],
            Up => vec![(0, -1, Left), (0, 1, Right), (-1, 0, Up)],
        }
    }
}
