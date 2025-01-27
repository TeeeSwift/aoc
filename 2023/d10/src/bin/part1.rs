use anyhow::Result;
use d10::*;

pub fn main() -> Result<()> {
    let a = parse_input("src/input")?;
    let l = get_loop(&a)?;

    println!("{}", l.len() / 2);

    Ok(())
}
