use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub fn main() -> Result<()> {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    let mut total: usize = 0;

    for line in reader.lines() {
        let line = line?;
        let line = line.replace(",", "");
        let line = line.replace(":", "");
        let line = line.replace(";", "");

        let mut red: Option<usize> = None;
        let mut blue: Option<usize> = None;
        let mut green: Option<usize> = None;

        let mut count: usize = 0;

        for (index, substring) in line.split_whitespace().enumerate() {
            if index == 0 || index == 1 {
                continue;
            };

            match index % 2 {
                0 => {
                    let a = substring.parse().unwrap();
                    count = a;
                }
                1 => match substring {
                    "red" => {
                        if count > red.unwrap_or(usize::MIN) {
                            red = Some(count);
                        }
                    }
                    "blue" => {
                        if count > blue.unwrap_or(usize::MIN) {
                            blue = Some(count);
                        }
                    }
                    "green" => {
                        if count > green.unwrap_or(usize::MIN) {
                            green = Some(count);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        total += red.unwrap() * blue.unwrap() * green.unwrap();
    }

    println!("{total}");

    Ok(())
}
