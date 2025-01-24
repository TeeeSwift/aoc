use anyhow::Result;
use d09::*;

pub fn main() -> Result<()> {
    let d = parse_input("src/input")?;
    let mut total: isize = 0;

    for dataset in d {
        let dataset = diff_many(&dataset)?;
        let next = find_next(dataset)?;
        total += next;
    }

    println!("{total}");

    Ok(())
}
