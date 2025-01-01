pub mod keypad;
pub mod position;

use std::collections::{HashMap, VecDeque};

use crate::position::Position;

pub fn parse_input(filename: &str) -> Result<Vec<String>, std::io::Error> {
    use std::io::BufRead;
    let f = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(f);

    reader.lines().collect::<Result<Vec<String>, _>>()
}

pub fn extract_number(input: &str) -> Option<u64> {
    let number_str: String = input
        .chars()
        .filter(|c| c.is_ascii_digit()) // Filter only digits
        .collect();

    number_str.parse::<u64>().ok() // Convert the collected string to a number
}

pub fn shortest_command_sequences(grid: &[Vec<char>], p1: Position, p2: Position) -> Vec<String> {
    if p1 == p2 {
        return vec![String::from("")];
    }

    let p1_char = grid[p1.y][p1.x];

    // find shortest paths
    let mut cost_map: HashMap<char, usize> = HashMap::from([(p1_char, 0)]);
    let mut parent_map: HashMap<Position, Vec<Position>> = HashMap::new();

    let mut queue: VecDeque<Position> = VecDeque::from([p1]);

    while let Some(position) = queue.pop_front() {
        let source_char = grid[position.y][position.x];
        let &source_cost = cost_map.get(&source_char).unwrap_or(&0);

        // get neighbors in grid
        let mut neighbors: Vec<Position> = vec![];

        for (dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let newy = dy + position.y as isize;
            let newx = dx + position.x as isize;

            // see if new cell is in the grid
            if newy < 0 || newx < 0 || newy >= grid.len() as isize || newx >= grid[0].len() as isize
            {
                continue;
            };

            let newy = newy as usize;
            let newx = newx as usize;
            neighbors.push(Position { y: newy, x: newx });
        }

        for neighbor in neighbors {
            let neighbor_char = grid[neighbor.y][neighbor.x];
            if neighbor_char == '#' {
                continue;
            };

            let &neighbor_cost = cost_map.get(&neighbor_char).unwrap_or(&usize::MAX);

            if neighbor_cost > source_cost + 1 {
                cost_map.insert(neighbor_char, source_cost + 1);
                parent_map.insert(neighbor, vec![position]);
                queue.push_back(neighbor);
            }

            if neighbor_cost == source_cost + 1 {
                parent_map.entry(neighbor).and_modify(|e| {
                    if !e.contains(&position) {
                        e.push(position);
                    }
                });

                queue.push_back(neighbor);
            }
        }
    }

    // traverse from p2 to recreate each path....
    // how tf do i do this

    fn derive_path(
        position: Position,
        parent_map: &HashMap<Position, Vec<Position>>,
    ) -> Vec<Vec<Position>> {
        let parents = parent_map.get(&position);
        match parents {
            None => vec![vec![position]],
            Some(parents) => {
                let mut v: Vec<Vec<Position>> = vec![];

                for &parent in parents {
                    let parent_paths = derive_path(parent, parent_map);
                    for mut path in parent_paths {
                        path.push(position);
                        v.push(path);
                    }
                }
                v
            }
        }
    }

    let paths: Vec<Vec<Position>> = derive_path(p2, &parent_map);
    let mut strings: Vec<String> = vec![];

    for path in paths {
        let mut keys: Vec<char> = vec![];
        for i in 0..path.len() - 1 {
            let dy = path[i].y as isize - path[i + 1].y as isize;
            let dx = path[i].x as isize - path[i + 1].x as isize;

            match (dy, dx) {
                (-1, 0) => keys.push('v'),
                (1, 0) => keys.push('^'),
                (0, 1) => keys.push('<'),
                (0, -1) => keys.push('>'),
                _ => {}
            }
        }
        strings.push(keys.into_iter().collect::<String>())
    }
    fn is_valid_zigzag(s: &str) -> bool {
        let directions = ['<', '>', 'v', '^'];
        let mut last_seen = [usize::MAX; 4]; // Initialize with usize::MAX to indicate not seen

        for (i, c) in s.chars().enumerate() {
            if let Some(index) = directions.iter().position(|&d| d == c) {
                // Check if the same direction appeared earlier with other characters in between
                if last_seen[index] != usize::MAX && i - last_seen[index] > 1 {
                    return false; // If the same direction appeared earlier with another character in between
                }
                last_seen[index] = i; // Update the last seen index for the current direction
            }
        }

        true
    }

    strings.into_iter().filter(|s| is_valid_zigzag(s)).collect()
}

pub fn build_commands(grid: &[Vec<char>]) -> HashMap<(char, char), Vec<String>> {
    let mut map: HashMap<(char, char), Vec<String>> = HashMap::new();

    // let v = self.shortest_command_sequences(Position { y: 0, x: 0 }, Position { y: 2, x: 2 });

    for (i, row) in grid.iter().enumerate() {
        for (j, &char1) in row.iter().enumerate() {
            for (k, other_row) in grid.iter().enumerate() {
                for (l, &char2) in other_row.iter().enumerate() {
                    let v = shortest_command_sequences(
                        grid,
                        Position { y: i, x: j },
                        Position { y: k, x: l },
                    );

                    map.insert((char1, char2), v);
                }
            }
        }
    }
    map
}

#[test]
fn t_parse_input() -> Result<(), std::io::Error> {
    let v = parse_input("src/sample")?;
    assert_eq!(v, ["029A", "980A", "179A", "456A", "379A"]);
    Ok(())
}
