pub mod node;

use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use node::Node;

pub fn parse_input() -> Vec<Vec<char>> {
    // let file = File::open("src/sample").unwrap();
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn get_measurements(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> (char, usize, usize, usize) {
    if visited.contains(&(start.0, start.1)) {
        return ('.', 0, 0, 0);
    };

    let value = grid[start.0][start.1];
    let mut area: usize = 0;
    let mut perimeter: usize = 0;
    let mut corner_count: usize = 0;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back((start.0, start.1));

    // Process the queue
    while let Some((y, x)) = queue.pop_front() {
        match visited.contains(&(y, x)) {
            true => continue,
            false => visited.insert((y, x)),
        };

        let value = grid[y][x];
        let mut node = Node::new(value, y, x, grid);

        let valid_neighbors = node.get_valid_neighbors();
        // println!("{}, ({} {})", value, y, x);

        // println!("{:?}", valid_neighbors);

        for neighbor in valid_neighbors.into_iter() {
            queue.push_back(neighbor)
        }

        area += 1;
        let edge_count = node.count_edges();
        perimeter += edge_count;
        corner_count += node.count_corners();

        // println!("{}", corner_count);
    }

    return (value, area, perimeter, corner_count);
}
