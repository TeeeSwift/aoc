use anyhow::Result;
use d09::*;

pub fn main() -> Result<()> {
    let d = parse_input("src/input")?;
    let mut total: isize = 0;

    for dataset in d {
        let dataset = diff_many(&dataset)?;
        let prev = find_prev(dataset)?;
        total += prev;
    }

    println!("total: {total}");

    Ok(())
}
