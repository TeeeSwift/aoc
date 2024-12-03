use d3::parse;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let sum = parse(&input);

    println!("-------------------");
    println!("Sum: {}", sum);

    Ok(())
}
