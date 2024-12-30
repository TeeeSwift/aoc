use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

pub mod position;
use position::Position;

type TimeSaved = usize;

pub fn get_shortcuts(
    path: Vec<Position>,
    max_cheat: usize,
) -> HashMap<(Position, Position), TimeSaved> {
    let mut shortcuts: HashMap<(Position, Position), usize> = HashMap::new();

    for i in 0..path.len() {
        for j in 1..path.len() {
            let position_i = path[i];
            let position_j = path[j];

            // see if i and j are 1 square apart
            if reachable_in_time(position_i, position_j, max_cheat) && (j as isize - i as isize > 2)
            {
                let time_saved: TimeSaved = j - i - min_steps(position_i, position_j);
                // is shortcut
                shortcuts.insert((position_i, position_j), time_saved);
            }
        }
    }

    shortcuts
}

pub fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let file = std::fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect::<Vec<char>>());
    }

    grid
}

pub fn _print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
}

pub fn find_start(grid: &[Vec<char>]) -> Option<Position> {
    for (i, line) in grid.iter().enumerate() {
        if let Some(x) = line.iter().position(|x| *x == 'S') {
            return Some(Position { y: i, x });
        }
    }

    None
}

pub fn generate_path(grid: &[Vec<char>], start: Position) -> Vec<Position> {
    let mut path: Vec<Position> = vec![start];
    let vectors: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    step(grid, start, start, &mut path, &vectors);

    path
}

fn step(
    grid: &[Vec<char>],
    position: Position,
    prev: Position,
    path: &mut Vec<Position>,
    vectors: &[(isize, isize)],
) {
    // for each position, get neighbors, add to path, call get_neighbor
    //  =

    for (dy, dx) in vectors {
        let y = position.y;
        let x = position.x;
        let newy = dy + y as isize;
        let newx = dx + x as isize;

        if newy < 0
            || newx < 0
            || newy >= grid.len() as isize
            || newx >= grid[0].len() as isize
            || (newy == prev.y as isize && newx == prev.x as isize)
            || grid[newy as usize][newx as usize] == '#'
        {
            continue;
        };

        let newy = newy as usize;
        let newx = newx as usize;

        path.push(Position { y: newy, x: newx });
        step(grid, Position { y: newy, x: newx }, position, path, vectors);
    }
}

pub fn reachable_in_time(i: Position, j: Position, max_cheat: usize) -> bool {
    min_steps(i, j) <= max_cheat
}

fn min_steps(i: Position, j: Position) -> usize {
    ((i.y as isize - j.y as isize).abs() + (i.x as isize - j.x as isize).abs()) as usize
}

#[test]
fn t_parse_input() {
    let grid = parse_input("src/sample");
    _print_grid(&grid);
}

#[test]
fn t_find_start() {
    let grid = parse_input("src/sample");
    let start = find_start(&grid).unwrap();
    assert_eq!(Position { y: 3, x: 1 }, start);
}

#[test]
fn t_generate_path() {
    let grid = parse_input("src/sample");
    let start = find_start(&grid).unwrap();
    let path = generate_path(&grid, start);
    assert_eq!(path.len(), 85);
}

#[test]
fn t_is_shortcut() {
    let i = Position { y: 0, x: 2 };
    let j = Position { y: 0, x: 0 };
    assert!(reachable_in_time(i, j, 2));

    let i = Position { y: 0, x: 1 };
    let j = Position { y: 0, x: 0 };
    assert!(reachable_in_time(i, j, 2));

    let i = Position { y: 0, x: 0 };
    let j = Position { y: 3, x: 3 };
    assert!(reachable_in_time(i, j, 6));
    assert!(reachable_in_time(i, j, 8));
    assert!(!reachable_in_time(i, j, 5));
}

#[test]
fn t_get_shortcuts_2sec() {
    let grid = parse_input("src/sample");
    let start = find_start(&grid).unwrap();
    let path = generate_path(&grid, start);
    let shortcuts = get_shortcuts(path, 2);

    println!("{:?}", shortcuts);

    assert_eq!(shortcuts.values().copied().filter(|&x| x == 4).count(), 14);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 6).count(), 2);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 8).count(), 4);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 10).count(), 2);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 12).count(), 3);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 20).count(), 1);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 36).count(), 1);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 38).count(), 1);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 40).count(), 1);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 64).count(), 1);
}

#[test]
fn t_get_shortcuts_20sec() {
    let grid = parse_input("src/sample");
    let start = find_start(&grid).unwrap();
    let path = generate_path(&grid, start);
    let shortcuts = get_shortcuts(path, 20);

    println!("{:?}", shortcuts);

    assert_eq!(shortcuts.values().copied().filter(|&x| x == 50).count(), 32);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 52).count(), 31);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 54).count(), 29);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 56).count(), 39);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 58).count(), 25);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 60).count(), 23);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 62).count(), 20);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 64).count(), 19);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 66).count(), 12);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 68).count(), 14);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 70).count(), 12);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 72).count(), 22);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 74).count(), 4);
    assert_eq!(shortcuts.values().copied().filter(|&x| x == 76).count(), 3);
}
