use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub fn main() -> Result<()> {
    // let file = File::open("src/sample").unwrap();
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut text_to_num = HashMap::new();
        text_to_num.insert("0", 0);
        text_to_num.insert("1", 1);
        text_to_num.insert("2", 2);
        text_to_num.insert("3", 3);
        text_to_num.insert("4", 4);
        text_to_num.insert("5", 5);
        text_to_num.insert("6", 6);
        text_to_num.insert("7", 7);
        text_to_num.insert("8", 8);
        text_to_num.insert("9", 9);

        let mut left = ("", usize::MAX);
        let mut right = ("", usize::MIN);

        for substring in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"] {
            if let Some(index) = line.find(substring) {
                if index <= left.1 {
                    left = (substring, index);
                }
            };

            if let Some(index) = line.rfind(substring) {
                if index >= right.1 {
                    right = (substring, index);
                }
            };
        }

        println!("{} {} {line}", left.0, right.0);
        total += *text_to_num.get(&left.0).unwrap() * 10;
        total += *text_to_num.get(&right.0).unwrap();
    }

    println!("{total}");

    Ok(())
}
