use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub mod direction;
pub mod position;

use self::direction::Direction;
use self::direction::Direction::{Down, Left, Right, Up};
use self::position::Position;

pub fn parse_input() -> (Vec<Vec<char>>, Position, Vec<Position>) {
    // let filename = "src/sample";
    // let filename = "src/sample2";
    let filename = "src/input";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut v: Vec<Vec<char>> = vec![];
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect::<Vec<char>>();

        if let Some(col) = line.chars().position(|x| x == 'S') {
            start = (index, col);
        }

        if let Some(col) = line.chars().position(|x| x == 'E') {
            goal = (index, col);
        }

        v.push(row);
    }

    (
        v,
        Position(start.0, start.1, Right),
        vec![
            Position(goal.0, goal.1, Right),
            Position(goal.0, goal.1, Left),
            Position(goal.0, goal.1, Up),
            Position(goal.0, goal.1, Down),
        ],
    )
}

pub fn get_options(grid: &[Vec<char>], position: Position) -> Vec<(Position, usize)> {
    let (y, x, direction): (usize, usize, Direction) = (position.0, position.1, position.2);

    let mut options: Vec<(Position, usize)> = vec![];

    for (dy, dx, d_direction) in direction.get_vectors() {
        let newy = y as isize + dy;
        let newx = x as isize + dx;

        // see if newy newx is in the grid bounds
        if newy < 0 || newx < 0 || newy >= grid.len() as isize || newx >= grid.len() as isize {
            continue;
        }

        let newy = newy as usize;
        let newx = newx as usize;

        let dest_value: char = grid[newy][newx];

        if dest_value == '#' {
            continue;
        };

        let mut step_cost: usize = 1;
        if d_direction != direction {
            step_cost = 1001
        }

        options.push((Position(newy, newx, d_direction), step_cost));
    }

    options
}

pub fn _print_grid(grid: &[Vec<char>]) {
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>())
    }
}

#[test]
fn test_get_options() {
    let grid: Vec<Vec<char>> = vec![
        vec!['#', '.', '#'],
        vec!['.', '@', '#'],
        vec!['.', '#', '#'],
    ];

    let options = get_options(&grid, Position(2, 1, Left));
    println!("{:?}", options);
    assert_eq!(
        options,
        [(Position(1, 0, Left), 1), (Position(0, 1, Up), 1001)]
    );

    let grid: Vec<Vec<char>> = vec![
        vec!['#', '.', '#'],
        vec!['.', '@', '.'],
        vec!['.', '.', '#'],
    ];

    let options = get_options(&grid, Position(1, 1, Left));
    println!("{:?}", options);
    assert_eq!(
        options,
        [
            (Position(2, 1, Down), 1001),
            (Position(1, 0, Left), 1),
            (Position(0, 1, Up), 1001)
        ]
    );
}
