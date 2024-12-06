use crate::board::Board;
use crate::solver::Solver;

pub struct CROSS {
    pub board: Board,
}

impl Solver for CROSS {
    fn get_board(&self) -> &Board {
        &self.board
    }

    fn get_starter_char(&self, string: &str) -> char {
        return string.chars().nth(string.len() / 2).unwrap();
    }

    fn get_ranges(&self, origin: (usize, usize), word: &str) -> Vec<Vec<(i32, i32)>> {
        let mut ranges: Vec<Vec<(i32, i32)>> = vec![];

        let vectors: Vec<(isize, isize)> = vec![
            (-1, -1), // NorthWest
            (-1, 1),  // NorthEast
            (1, 1),   // SouthEast
            (1, -1),  // SouthWest
        ];

        for (dy, dx) in vectors {
            let mut range: Vec<(i32, i32)> = vec![];
            for index in 0..word.len() {
                let half_len = word.len() / 2;
                let new_y = origin.0 as i32 + (index as i32 - half_len as i32) * dy as i32;
                let new_x = origin.1 as i32 + (index as i32 - half_len as i32) * dx as i32;
                range.push((new_y, new_x));
            }
            ranges.push(range);
        }

        return ranges;
    }

    fn check_starter(&self, start_location: (usize, usize)) -> usize {
        let ranges: Vec<Vec<(i32, i32)>> = self.get_ranges(start_location, &self.board.word);

        let results: usize = ranges
            .iter()
            .map(|range| self.board.join_range(range.clone()))
            .filter(|x| *x == self.board.word)
            .count();

        if results == 2 {
            return 1 as usize;
        }

        0 as usize
    }
}
