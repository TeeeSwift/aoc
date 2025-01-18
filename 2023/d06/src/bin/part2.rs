use anyhow::Result;
use d06::*;

pub fn main() -> Result<()> {
    let (time, benchmark) = parse_input2("src/input")?;

    let e = earliest_win(time, benchmark).unwrap();
    let ways = time - e - e + 1;

    println!("{ways}");
    Ok(())
}
