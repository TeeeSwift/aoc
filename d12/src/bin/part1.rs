use std::collections::HashSet;

use d12::{get_cost, parse_input};

pub fn main() {
    let grid: Vec<Vec<char>> = parse_input();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut total: usize = 0;

    for (y, r) in grid.iter().enumerate() {
        for (x, _) in r.iter().enumerate() {
            total += get_cost(&grid, (y, x), &mut visited);
        }
    }

    println!("{}", total);
}
