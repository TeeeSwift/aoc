use crate::parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn rules(line: String) -> (u16, u16) {
    println!("{}", line);

    let mut iter = line.split('|');
    return (
        iter.next().unwrap().parse::<u16>().unwrap(),
        iter.next().unwrap().parse::<u16>().unwrap(),
    );
}
pub fn updates(line: String) -> Vec<u16> {
    return line
        .split(',')
        .map(|num| num.parse::<u16>().unwrap())
        .collect();
}

pub fn parse_input() -> std::io::Result<(Vec<(u16, u16)>, Vec<Vec<u16>>)> {
    let file = File::open("src/input")?;
    // let file = File::open("src/sample")?;
    let reader = BufReader::new(file);

    let mut rules: Vec<(u16, u16)> = vec![];
    let mut updates: Vec<Vec<u16>> = vec![];

    let mut current_section: usize = 1;

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            // Switch to the second section when an empty line is encountered
            current_section = 2;
        } else {
            match current_section {
                1 => rules.push(parser::rules(line)),
                2 => updates.push(parser::updates(line)),
                _ => {}
            };
        }
    }

    return Ok((rules, updates));
}
