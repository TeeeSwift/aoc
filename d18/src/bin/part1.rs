use std::sync::Arc;

use d18::*;

pub fn main() {
    let (mut grid, corruptions) = parse_input();
    // progress_grid(&mut grid, &mut corruptions, 12);
    progress_grid(&mut grid, Arc::new(&corruptions), 1024);
    let path = find_shortest_path(&mut grid).unwrap();
    println!("{}", path.len());
}
