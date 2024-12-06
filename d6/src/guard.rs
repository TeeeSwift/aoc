use std::collections::HashSet;
use std::hash::Hash;
use std::{thread, time};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Stop {
    pub coordinates: (usize, usize),
    pub direction: Direction,
}

#[derive(Clone)]
pub struct Guard {
    pub coordinates: (usize, usize),
    pub direction: Direction,
    pub out_of_bounds: bool,
    pub looped: bool,
    pub stops: HashSet<Stop>,
    pub grid: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub enum PatrolResult {
    EXIT,
    LOOP,
}

impl Direction {
    pub fn get_vector(&self) -> (isize, isize) {
        match self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        }
    }

    pub fn get_character(&self) -> char {
        match self {
            Direction::UP => '^' as char,
            Direction::DOWN => 'v' as char,
            Direction::LEFT => '<' as char,
            Direction::RIGHT => '>' as char,
        }
    }
}

impl Guard {
    pub fn new(v: Vec<Vec<char>>) -> Self {
        let grid = v;
        let coordinates = grid
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| {
                row.iter()
                    .position(|x| *x == '^')
                    .map(|col_index| (row_index, col_index))
            })
            .unwrap();

        Guard {
            coordinates,
            direction: Direction::UP,
            out_of_bounds: false,
            looped: false,
            grid,
            stops: HashSet::new(),
        }
    }

    pub fn get_dest_coordinates(&self) -> Option<(usize, usize)> {
        let (dy, dx) = self.direction.get_vector();
        let (y, x) = self.coordinates;
        let new_y = dy + y as isize;
        let new_x = dx + x as isize;

        if new_y < 0
            || new_x < 0
            || new_y >= self.grid.len() as isize
            || new_x >= self.grid[0].len() as isize
        {
            return None;
        }

        Some((new_y as usize, new_x as usize))
    }

    pub fn turn_right(&mut self) {
        let stop = Stop {
            coordinates: self.coordinates,
            direction: self.direction,
        };

        if self.stops.contains(&stop) {
            self.looped = true;
        } else {
            self.stops.insert(stop);

            match self.direction {
                Direction::UP => self.direction = Direction::RIGHT,
                Direction::DOWN => self.direction = Direction::LEFT,
                Direction::LEFT => self.direction = Direction::UP,
                Direction::RIGHT => self.direction = Direction::DOWN,
            };
        };
        // self.print_room();
    }

    pub fn next(&mut self) {
        // get next tile coordinates
        let dest = self.get_dest_coordinates();

        // if there is no next tile, set out_of_bounds to true and return
        if dest.is_none() {
            self.out_of_bounds = true;
            return;
        }

        let dest = dest.unwrap();

        // get the value of next tile
        let next_cell_value = self.grid[dest.0][dest.1];
        match next_cell_value {
            '#' => self.turn_right(),
            _ => self.step_forward((dest.0, dest.1)),
        }
    }

    pub fn step_forward(&mut self, dest: (usize, usize)) {
        let (row, col) = self.coordinates;
        let (new_y, new_x) = dest;

        self.grid[row][col] = 'X';
        // set value of current position to *
        self.grid[new_y][new_x] = self.direction.get_character();
        // set value of destination to Guard Icon
        self.coordinates = dest;
        // self.print_room();
    }

    pub fn patrol(&mut self) -> PatrolResult {
        while !self.looped && !self.out_of_bounds {
            self.next();
        }

        if self.looped {
            return PatrolResult::LOOP;
        }

        return PatrolResult::EXIT;
    }

    fn print_room(&self) {
        thread::sleep(time::Duration::from_millis(100));
        print!("\x1b[2J\x1b[H");
        for row in self.grid.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}

#[test]
fn get_dest_coordinates() {
    let v = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '.', '^', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];

    let guard = Guard::new(v);
    let dest = guard.get_dest_coordinates();
    assert_eq!(dest.unwrap(), (0, 2));

    let v = vec![
        vec!['.', '.', '^', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];

    let guard = Guard::new(v);
    let dest = guard.get_dest_coordinates();
    assert!(dest.is_none());
}

#[test]
fn turn_right() {
    let v = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '.', '^', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];
    let mut guard = Guard::new(v);

    guard.turn_right();
    assert!(matches!(guard.direction, Direction::RIGHT))
}

#[test]
fn test_loop() {
    let v = vec![
        vec!['.', '.', '#', '.', '.'],
        vec!['.', '#', '^', '.', '#'],
        vec!['.', '.', '.', '#', '.'],
    ];
    let mut guard = Guard::new(v);
    guard.patrol();

    for row in guard.grid.iter() {
        println!("{:?}", row);
    }
    assert!(guard.looped);
}
