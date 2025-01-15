use anyhow::Result;
use d05::*;

pub fn main() -> Result<()> {
    let (mut seeds, phases) = parse_input("src/input")?;
    let min: isize = seeds
        .iter_mut()
        .map(|e| {
            phases
                .iter()
                .for_each(|phase| *e = multi_convert(*e, phase));
            *e
        })
        .min()
        .unwrap();

    println!("{min}");

    Ok(())
}
