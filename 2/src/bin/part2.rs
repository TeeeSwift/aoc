use d2::Report;
use std::fs::File;
use std::io::{BufRead, BufReader};

// part 2
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
