use d3::extract_factor_pairs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut sum: i32 = 0;
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // let str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let v = extract_factor_pairs(&line);
        for (n1, n2) in v.iter() {
            sum += n1 * n2;
        }
    }

    println!("-------------------");
    println!("Sum: {}", sum);
    Ok(())
}
