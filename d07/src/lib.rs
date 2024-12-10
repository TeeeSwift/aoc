use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod equation;
use equation::{Equation, Operations};

pub fn parse_input(operations: Vec<Operations>) -> Vec<Equation> {
    let file = File::open("src/input").unwrap();
    // let file = File::open("src/sample").unwrap();
    let reader = BufReader::new(file);

    let mut v: Vec<Equation> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split(':');

        let target: u64 = iter.next().unwrap().parse().unwrap();
        let numbers: Vec<u64> = iter
            .next()
            .unwrap()
            .trim_ascii()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        v.push(equation::Equation {
            target,
            numbers,
            operations: operations.clone(),
            combinations: vec![],
            potential_results: vec![],
        })
    }

    return v;
}
