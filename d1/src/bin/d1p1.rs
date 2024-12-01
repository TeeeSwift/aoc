use std::fs::File;
use std::io::{BufRead, BufReader};

// part 1
fn main() -> std::io::Result<()> {
    let mut sum: i64 = 0;
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    let file = File::open("src/bin/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().to_string().parse::<i64>().unwrap());
        right.push(iter.next().unwrap().to_string().parse::<i64>().unwrap());
    }

    left.sort();
    right.sort();

    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs()
    }

    println!("{}", sum);

    Ok(())
}
