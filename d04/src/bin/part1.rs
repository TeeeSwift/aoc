use std::fs::File;
use std::io::{BufRead, BufReader};

use d4::board::Board;
use d4::solver::{Solver, STRAIGHT};

// part 1
fn main() -> std::io::Result<()> {
    let file = File::open("src/input")?;
    // let file = File::open("src/sample")?;
    let reader = BufReader::new(file);

    // instantiate board
    let search_term = String::from("XMAS");
    let v: Vec<String> = reader.lines().map(|string| string.unwrap()).collect();

    let board = Board::new(v, search_term);
    let solver = STRAIGHT { board };

    // get starter locations
    let arr = solver.get_starter_locations();

    // check each origin
    let total: usize = arr
        .iter()
        .map(|origin_coordinates| solver.check_starter(*origin_coordinates))
        .sum();

    println!("total: {:?}", total);

    Ok(())
}
