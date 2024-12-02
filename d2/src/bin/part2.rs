use std::fs::File;
use std::io::{BufRead, BufReader};

// part 2
struct Report {
    values: Vec<i32>,
}

impl Report {
    fn new(vec: Vec<i32>) -> Self {
        Self { values: vec }
    }

    fn is_valid(&self) -> bool {
        let v = &self.values;

        let increasing: bool = (v[1] - v[0]).is_positive();
        let mut valid: bool = true;

        for index in 1..v.len() {
            let difference = v[index] - v[index - 1];

            if difference.is_positive() != increasing
                || difference.abs() < 1
                || difference.abs() > 3
            {
                valid = false;
                break;
            }
        }

        valid
    }

    fn try_adjustment(&self) -> bool {
        for i in 0..self.values.len() {
            let mut alternative: Vec<i32> = self.values.clone();
            alternative.remove(i);
            let report = Report::new(alternative);
            if report.is_valid() {
                return true;
            }
        }

        false
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/bin/input.txt")?;
    // let file = File::open("src/bin/sample.txt")?;
    let reader = BufReader::new(file);

    let mut valid_report_count: u16 = 0;

    println!(
        "{:<40} {:<10} {:<10} {}",
        "VALUES", "INITIAL", "RETRY", "TOTAL"
    );

    for line in reader.lines() {
        let line = line?;
        let v: Vec<i32> = line.split(' ').map(|v| v.parse::<i32>().unwrap()).collect();
        let report = Report::new(v);
        let formatted_vector = format!("{:?}", report.values);

        if report.is_valid() {
            valid_report_count += 1;
            println!(
                "{:<40} {:<10} {:<10} {}",
                formatted_vector, "[  ]", "[  ]", valid_report_count
            );
            continue;
        }

        if report.try_adjustment() {
            valid_report_count += 1;
            println!(
                "{:<40} {:<10} {:<10} {}",
                formatted_vector, "FAIL", "SUCCESS", valid_report_count
            );
        } else {
            println!(
                "{:<40} {:<10} {:<10} {}",
                formatted_vector, "FAIL", "[  ]", valid_report_count
            );
        }
    }

    println!("number of valid reports: {}", valid_report_count);

    Ok(())
}
