use std::{
    collections::HashMap,
    fs::{self},
};

pub mod computer;

use regex::Regex;

pub fn parse_input() -> (usize, usize, usize, Vec<usize>) {
    let mut registers: HashMap<char, usize> = HashMap::new();

    // let filename = "src/sample2";
    let filename = "src/input";

    let re = Regex::new(
        r"Register A:\s*(\d+)\s*Register B:\s*(\d+)\s*Register C:\s*(\d+)\s*Program:\s*([\d,]+)",
    )
    .unwrap();

    let s = fs::read_to_string(filename).unwrap();

    let captures = re.captures(&s).unwrap();

    registers.insert('A', captures[1].parse().unwrap());
    registers.insert('B', captures[2].parse().unwrap());
    registers.insert('C', captures[3].parse().unwrap());

    let mut s = captures[4].to_string();
    s.retain(|c| !c.is_whitespace());
    let commands = s
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (
        captures[1].parse::<usize>().unwrap(),
        captures[2].parse::<usize>().unwrap(),
        captures[3].parse::<usize>().unwrap(),
        commands,
    )
}
