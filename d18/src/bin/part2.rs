use d18::*;

pub fn main() {
    let (grid, original_corruptions) = parse_input();

    for i in 1..original_corruptions.len() {
        let mut grid = grid.clone();
        let mut corruptions = original_corruptions.clone();
        progress_grid(&mut grid, &mut corruptions, i);
        match find_shortest_path(&mut grid) {
            Ok(_) => {}
            Err(_) => {
                println!("hello");
                println!("{}", i - 1);
                println!("{:?}", original_corruptions[i - 1]);
                break;
            }
        };
    }
}
