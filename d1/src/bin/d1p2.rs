use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut sum: i64 = 0;
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    let file = File::open("src/bin/input.txt").unwrap();
    // let file = File::open("src/bin/test.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().to_string().parse::<i64>().unwrap());
        right.push(iter.next().unwrap().to_string().parse::<i64>().unwrap());
    }

    let mut right_hashmap = HashMap::new();
    for num in right {
        let count = right_hashmap.entry(num).or_insert(0);
        *count += 1;
    }

    for value in left {
        let count = right_hashmap.get(&value).unwrap_or(&0);
        let quotient = value * count;
        sum += quotient;
    }

    println!("{}", sum);

    Ok(())
}
