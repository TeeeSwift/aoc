use anyhow::Result;
use d06::*;

pub fn main() -> Result<()> {
    let races = parse_input("src/input")?;

    let total: usize = races
        .into_iter()
        .map(|(time, benchmark)| {
            let e = earliest_win(time, benchmark).unwrap();

            time - e - e + 1
        })
        .product();

    println!("{total}");
    Ok(())
}
