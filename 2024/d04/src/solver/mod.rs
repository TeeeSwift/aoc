use crate::board::Board;

pub use self::cross::CROSS;
pub use self::straight::STRAIGHT;

// Import the specific implementations from their respective files
pub mod cross;
pub mod straight;

pub trait Solver {
    fn get_board(&self) -> &Board;
    fn get_starter_char(&self, string: &str) -> char;
    fn get_starter_locations(&self) -> Vec<(usize, usize)> {
        let starter_char = self.get_starter_char(&self.get_board().word);

        let mut starter_locations: Vec<(usize, usize)> = Vec::new();

        for (row, str) in self.get_board().board.iter().enumerate() {
            for (col, char) in str.char_indices() {
                if char == starter_char {
                    starter_locations.push((row, col));
                }
            }
        }

        starter_locations
    }
    fn check_starter(&self, starter_location: (usize, usize)) -> usize;
    fn get_ranges(&self, origin: (usize, usize), word: &str) -> Vec<Vec<(i32, i32)>>;
}
