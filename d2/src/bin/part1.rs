use d2::Report;
use std::fs::File;
use std::io::{BufRead, BufReader};

// part 1
fn main() -> std::io::Result<()> {
    let file = File::open("src/bin/input.txt")?;
    // let file = File::open("src/bin/sample.txt")?;
    let reader = BufReader::new(file);

    let mut valid_report_count: u16 = 0;

    for line in reader.lines() {
        let line = line?;
        let v: Vec<i32> = line.split(' ').map(|v| v.parse::<i32>().unwrap()).collect();
        let report = Report::new(v);

        if report.is_valid() {
            valid_report_count += 1;
        }
    }

    println!("number of valid reports: {}", valid_report_count);

    Ok(())
}
