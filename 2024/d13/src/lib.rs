use regex::Regex;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub mod button;
pub mod machine;
pub mod prize;

use machine::Machine;
pub fn parse_input() -> HashMap<isize, Machine> {
    // let file = File::open("src/sample").unwrap();
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    let mut machines: HashMap<isize, Machine> = HashMap::new();

    let mut i: isize = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let machine = machines.entry(i).or_insert(Machine::new());

        let button_regex = Regex::new(r"Button\s(\w):\sX\+(\d+),\sY\+(\d+)").unwrap();
        if let Some(captures) = button_regex.captures(&line) {
            let coins: isize = match &captures[1] {
                "A" => 3,
                "B" => 1,
                _ => 0,
            };

            let x = captures[2].parse::<isize>().unwrap();
            let y = captures[3].parse::<isize>().unwrap();
            machine.set_button(&captures[1], x, y, coins);
            continue;
        }

        let prize_regex = Regex::new(r"X=(\d+),\sY=(\d+)").unwrap();
        if let Some(captures) = prize_regex.captures(&line) {
            let x = captures[1].parse::<isize>().unwrap();
            let y = captures[2].parse::<isize>().unwrap();
            machine.set_prize(x, y);
            continue;
        }

        i += 1;
    }

    return machines;
}

#[test]
fn test_parse_input() {
    parse_input();
}
