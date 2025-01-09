use crate::board::Board;
use crate::solver::Solver;

pub struct STRAIGHT {
    pub board: Board,
}

impl Solver for STRAIGHT {
    fn get_board(&self) -> &Board {
        &self.board
    }

    fn get_starter_char(&self, string: &str) -> char {
        string.chars().next().unwrap()
    }

    fn get_ranges(&self, origin: (usize, usize), word: &str) -> Vec<Vec<(i32, i32)>> {
        let mut ranges: Vec<Vec<(i32, i32)>> = vec![];

        let vectors: Vec<(isize, isize)> = vec![
            (0, 1),   // East
            (0, -1),  // West
            (-1, 0),  // North
            (1, 0),   // South
            (-1, -1), // NorthWest
            (-1, 1),  // NorthEast
            (1, 1),   // SouthEast
            (1, -1),  // SouthWest
        ];

        for (dy, dx) in vectors {
            let mut range: Vec<(i32, i32)> = vec![];
            for (index, _) in word.char_indices() {
                let new_y = origin.0 as i32 + index as i32 * dy as i32;
                let new_x = origin.1 as i32 + index as i32 * dx as i32;
                range.push((new_y, new_x));
            }
            ranges.push(range);
        }

        return ranges;
    }

    // returns number of successful matches for given starter location
    fn check_starter(&self, start_location: (usize, usize)) -> usize {
        let ranges: Vec<Vec<(i32, i32)>> = self.get_ranges(start_location, &self.board.word);

        let results: usize = ranges
            .iter()
            .map(|range| self.board.join_range(range.clone()))
            .filter(|x| *x == self.board.word)
            .count();

        return results;
    }
}
