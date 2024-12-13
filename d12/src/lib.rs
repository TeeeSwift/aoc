use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_input() -> Vec<Vec<char>> {
    // let file = File::open("src/sample").unwrap();
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn get_cost(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut area: usize = 0;
    let mut perimeter: usize = 0;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back((start.0, start.1));

    // Process the queue
    while let Some((y, x)) = queue.pop_front() {
        match visited.contains(&(y, x)) {
            true => continue,
            false => visited.insert((y, x)),
        };

        let mut valid_neighbors: usize = 0;
        let node = grid[y][x];
        println!("{}", node);

        // UP
        if y > 0 {
            if grid[y - 1][x] == node {
                valid_neighbors += 1;
                queue.push_back((y - 1, x));
            }
        };

        // RIGHT
        if x < grid[0].len() - 1 {
            if grid[y][x + 1] == node {
                valid_neighbors += 1;
                queue.push_back((y, x + 1));
            }
        };

        // DOWN
        if y < grid.len() - 1 {
            if grid[y + 1][x] == node {
                valid_neighbors += 1;
                queue.push_back((y + 1, x));
            }
        };

        // LEFT
        if x > 0 {
            if grid[y][x - 1] == node {
                valid_neighbors += 1;
                queue.push_back((y, x - 1));
            }
        };

        area += 1;
        perimeter += 4 - valid_neighbors;
        println!("{}, {}, {}, {}", y, x, area, perimeter);
        println!("{:?}", queue);
    }

    return area * perimeter;
}

#[test]
fn test_get_cost() {
    let grid = parse_input();

    // grid_rc
    //     .borrow()
    //     .iter()
    //     .for_each(|row| row.iter().for_each(|n| println!("{}, {}", n.y, n.x)));

    // assert_eq!(40, a);
}
