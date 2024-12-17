use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub mod direction;
pub mod grid;
pub mod node;

use self::grid::Grid;

pub fn parse_input() -> (Grid, String, (usize, usize)) {
    let filename = "src/sample";
    // let filename = "src/sample2";
    // let filename = "src/input";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut empty_line_number = 0;
    let mut robot_position: (usize, usize) = (0, 0);

    let mut v: Vec<Vec<char>> = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains('@') {
            robot_position.0 = index;
            robot_position.1 = line.chars().position(|x| x == '@').unwrap();
        }
        let row: Vec<char> = line.chars().collect::<Vec<char>>();

        v.push(row);

        if line.len() == 0 {
            empty_line_number = index + 1;
            break;
        };
    }

    let grid = Grid::new(v);

    let file = File::open(filename).unwrap();
    let reader2 = BufReader::new(file);

    let mut command_sequence: String = String::new();

    for line in reader2.lines().skip(empty_line_number) {
        let line = line.unwrap();

        for char in line.chars() {
            if char != '\n' {
                command_sequence.push(char);
            }
        }
    }

    return (grid, command_sequence, robot_position);
}

pub fn parse_input2() -> (Grid, String, (usize, usize)) {
    // let filename = "src/sample";
    // let filename = "src/sample2";
    let filename = "src/input";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut empty_line_number = 0;
    let mut robot_position: (usize, usize) = (0, 0);

    let mut v: Vec<Vec<char>> = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row: Vec<char> = vec![];
        line.chars().enumerate().for_each(|(i, c)| match c {
            '#' => {
                row.push('#');
                row.push('#');
            }
            '.' => {
                row.push('.');
                row.push('.');
            }
            '@' => {
                row.push('@');
                row.push('.');
                robot_position.0 = index;
                robot_position.1 = i * 2;
            }
            'O' => {
                row.push('[');
                row.push(']');
            }

            _ => {}
        });

        v.push(row);

        if line.len() == 0 {
            empty_line_number = index + 1;
            break;
        };
    }

    let grid = Grid::new(v);

    let file = File::open(filename).unwrap();
    let reader2 = BufReader::new(file);

    let mut command_sequence: String = String::new();

    for line in reader2.lines().skip(empty_line_number) {
        let line = line.unwrap();

        for char in line.chars() {
            if char != '\n' {
                command_sequence.push(char);
            }
        }
    }

    return (grid, command_sequence, robot_position);
}
