use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Schematic = Vec<Vec<char>>;

pub fn parse_input(filename: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut keys: Vec<Vec<Vec<char>>> = vec![];
    let mut locks: Vec<Vec<Vec<char>>> = vec![];

    let mut v: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            if v[0][0] == '#' {
                locks.push(v.clone());
            } else {
                keys.push(v.clone());
            }

            v.clear();
            continue;
        }

        v.push(line.chars().collect());
    }

    if v[0][0] == '#' {
        locks.push(v.clone());
    } else {
        keys.push(v.clone());
    }

    (keys, locks)
}

pub fn serialize(schematic: &[Vec<char>]) -> Vec<usize> {
    let mut v: Vec<usize> = vec![];
    for i in 0..schematic[0].len() {
        v.push(
            schematic.len()
                - 1
                - schematic
                    .iter()
                    .map(|row| row[i])
                    .filter(|&char| char == '.')
                    .count(),
        )
    }

    v
}

pub fn fits(key: &Schematic, lock: &Schematic) -> bool {
    let max = key.len() - 1;
    let key = serialize(key);
    let lock = serialize(lock);
    println!("{key:?} {lock:?}");

    for (index, value) in key.iter().enumerate() {
        if *value + lock[index] >= max {
            return false;
        }
    }

    true
}

#[test]
fn t_parse_input() {
    parse_input("src/sample");
}

#[test]
fn t_serialize() {
    let (keys, locks) = parse_input("src/sample");
    let a = serialize(&locks[0]);
    println!("{a:?}");

    let a = serialize(&keys[0]);
    println!("{a:?}");
}

#[test]
fn t_fits() {
    let (keys, locks) = parse_input("src/sample");
    assert!(fits(&locks[0], &keys[2]));
    assert!(!fits(&locks[0], &keys[0]));
    assert!(fits(&locks[1], &keys[1]));
}
