use d18::*;

pub fn main() {
    let (mut grid, mut corruptions) = parse_input();
    println!("{:?}", corruptions);
    // progress_grid(&mut grid, &mut corruptions, 12);
    progress_grid(&mut grid, &mut corruptions, 1024);
    let path = find_shortest_path(&mut grid).unwrap();
    println!("{}", path.len());
}
