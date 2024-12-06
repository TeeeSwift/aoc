use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub use guard::Guard;

pub mod guard;

pub fn parse_input() -> Vec<Vec<char>> {
    let file = File::open("src/input").unwrap();
    // let file = File::open("src/sample").unwrap();
    let reader = BufReader::new(file);
    let v: Vec<Vec<char>> = reader
        .lines()
        .map(|string| string.unwrap().chars().collect::<Vec<char>>())
        .collect();

    return v;
}
