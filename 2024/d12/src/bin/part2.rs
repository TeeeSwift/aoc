use std::collections::HashSet;

use d12::{get_measurements, parse_input};

pub fn main() {
    let grid: Vec<Vec<char>> = parse_input();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut total: usize = 0;

    for (y, r) in grid.iter().enumerate() {
        for (x, _) in r.iter().enumerate() {
            let (character, area, perimeter, edges) = get_measurements(&grid, (y, x), &mut visited);

            if area > 0 {
                println!(
                    "{} area: {}, perimeter: {}, edges: {}",
                    character, area, perimeter, edges
                );
            }
            total += area * edges;
        }
    }

    println!("{}", total);
}
