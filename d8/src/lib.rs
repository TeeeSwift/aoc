use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_input() -> (Vec<Vec<char>>, HashMap<char, Vec<(i16, i16)>>) {
    let file = File::open("src/input").unwrap();
    // let file = File::open("src/sample").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = vec![];
    let mut antennas: HashMap<char, Vec<(i16, i16)>> = HashMap::new();

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(line.chars().collect::<Vec<char>>());
        for (col, character) in line.chars().enumerate() {
            if character != '.' {
                let entry = antennas.entry(character).or_insert(vec![]);

                entry.push((row as i16, col as i16));
            }
        }
    }

    (grid, antennas)
}

pub fn find_antinode(node1: &(i16, i16), node2: &(i16, i16)) -> Option<(i16, i16)> {
    let dy = node2.0 - node1.0;
    let dx = node2.1 - node1.1;

    if dy == 0 && dx == 0 {
        return None;
    };

    // determine vector
    Some((node2.0 + dy, node2.1 + dx))
}

pub fn find_antinodes(
    node1: &(i16, i16),
    node2: &(i16, i16),
    grid: Vec<Vec<char>>,
) -> Vec<(i16, i16)> {
    let mut current_node: (i16, i16) = (node2.0, node2.1);

    let dy = node2.0 - node1.0;
    let dx = node2.1 - node1.1;

    if dy == 0 && dx == 0 {
        return vec![];
    };

    let mut v = vec![];

    // add

    while current_node.0 >= 0
        && current_node.0 < grid.len() as i16
        && current_node.1 >= 0
        && current_node.1 < grid[0].len() as i16
    {
        v.push(current_node);
        current_node = (current_node.0 + dy, current_node.1 + dx);
    }

    // minus

    current_node = (node2.0, node2.1);

    while current_node.0 >= 0
        && current_node.0 < grid.len() as i16
        && current_node.1 >= 0
        && current_node.1 < grid[0].len() as i16
    {
        v.push(current_node);
        current_node = (current_node.0 - dy, current_node.1 - dx);
    }

    // determine vector
    return v;
}
