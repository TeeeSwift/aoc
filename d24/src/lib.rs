use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub mod gate;
pub mod operation;
pub mod wires;

use wires::Wires;

use crate::gate::*;

pub fn parse_input(filename: &str) -> (Wires, VecDeque<Gate>) {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut commands: VecDeque<Gate> = VecDeque::new();

    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut section: usize = 1;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            section = 2;
            continue;
        }
        match section {
            1 => {
                let mut binary_string = line.split(": ");
                map.insert(
                    binary_string.next().unwrap().to_string(),
                    binary_string.next().unwrap().parse::<usize>().unwrap(),
                );
            }
            2 => {
                let mut binary_string = line.split_whitespace();
                commands.push_back(Gate {
                    input1: binary_string.next().unwrap().to_string(),
                    op: binary_string.next().unwrap().to_string().parse().unwrap(),
                    input2: binary_string.next().unwrap().to_string(),
                    dest: {
                        binary_string.next();
                        binary_string.next().unwrap().to_string()
                    },
                })
            }
            _ => {}
        }
    }

    (Wires::new(map), commands)
}

pub struct E {}
impl std::fmt::Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "uh oh")
    }
}

#[test]
fn t_parse_input() {
    let (m, c) = parse_input("src/sample1");
    println!("{m:?}");
    println!("{c:?}");
}

// #[test]
// fn t_execute_commands() {
//     let (mut m, mut c) = parse_input("src/input");
//
//     while let Some(x) = c.pop_front() {
//         match x.execute(&mut m) {
//             Ok(x) => {}
//             Err(_) => c.push_back(x),
//         }
//     }
//
//     println!("{m:?}");
// }
//
// #[test]
// fn collect_and_print() {
//     let (mut m, mut c) = parse_input("src/sample2");
//
//     while let Some(x) = c.pop_front() {
//         match x.execute(&mut m) {
//             Ok(x) => {}
//             Err(_) => c.push_back(x),
//         }
//     }
//
//     let mut zs = m
//         .keys()
//         .filter(|x| x.starts_with("z"))
//         .collect::<Vec<&String>>();
//
//     zs.sort();
//     zs.reverse();
//
//     println!("{zs:?}");
//     let mut binary_string = String::new();
//
//     for key in zs {
//         let &a = m.get(key).unwrap();
//         binary_string = format!("{binary_string}{a}");
//     }
//
//     println!("{binary_string}");
//
//     let decimal = u64::from_str_radix(&binary_string, 2).unwrap();
//     println!("{decimal}");
// }
