use std::collections::HashMap;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_input() -> HashMap<u64, u64> {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    let mut hm: HashMap<u64, u64> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        for num in line.split_whitespace() {
            hm.insert(num.to_string().parse::<u64>().unwrap(), 1);
        }
    }

    return hm;
}

pub fn step(hm: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_hm: HashMap<u64, u64> = HashMap::new();

    for (key, value) in hm.into_iter() {
        let stringified = key.to_string();

        if key == 0 {
            new_hm.entry(1).and_modify(|v| *v += value).or_insert(value);
        } else if stringified.len() % 2 == 0 {
            let left = stringified[..stringified.len() / 2]
                .to_string()
                .parse::<u64>()
                .unwrap();

            new_hm
                .entry(left)
                .and_modify(|v| *v += value)
                .or_insert(value);

            let right = stringified[stringified.len() / 2..stringified.len()]
                .to_string()
                .parse::<u64>()
                .unwrap();

            new_hm
                .entry(right)
                .and_modify(|v| *v += value)
                .or_insert(value);
        } else {
            new_hm
                .entry(key * 2024)
                .and_modify(|v| *v += value)
                .or_insert(value);
        }
    }

    return new_hm;
}

#[test]
fn test_step() {
    let mut hm: HashMap<u64, u64> = HashMap::from([(125, 1), (17, 1)]);
    for _ in 0..6 {
        hm = step(hm)
    }

    let sum: u64 = hm.into_iter().map(|(_, v)| v).sum();

    assert_eq!(sum, 22);

    let mut hm: HashMap<u64, u64> = HashMap::from([(125, 1), (17, 1)]);
    for _ in 0..25 {
        hm = step(hm)
    }

    let sum: u64 = hm.into_iter().map(|(_, v)| v).sum();

    assert_eq!(sum, 55312);
}
