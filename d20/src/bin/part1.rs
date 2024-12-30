use std::collections::HashMap;

use d20::position::*;
use d20::*;

pub fn main() -> Result<(), String> {
    let grid: Vec<Vec<char>> = parse_input("src/input");
    let start: Position = find_start(&grid).unwrap();
    let path: Vec<Position> = generate_path(&grid, start);

    let shortcuts: HashMap<(Position, Position), usize> = get_shortcuts(path, 2);

    let shortcuts_over_100: usize = shortcuts.values().copied().filter(|&x| x >= 100).count();

    println!("{:?}", shortcuts_over_100);

    Ok(())
}
