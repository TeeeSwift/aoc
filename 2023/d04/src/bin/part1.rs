use anyhow::Result;
use d04::*;

pub fn main() -> Result<()> {
    let a = parse_input("src/input")?;
    let mut total: usize = 0;
    for card in a {
        let b = calculate(card)?;
        total += b;
    }

    println!("{total}");

    Ok(())
}
