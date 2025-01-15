use anyhow::Result;
use d05::*;

pub fn main() -> Result<()> {
    let (seeds, phases) = parse_input("src/input")?;

    // convert into ranges (13, 5) -> (13, 18)
    let mut tuples: Vec<(isize, isize)> = vec![];
    let mut i = 0;
    while i < seeds.len() {
        tuples.push((seeds[i], seeds[i] + seeds[i + 1]));
        i += 2;
    }

    tuples.sort();

    for phase in &phases {
        tuples = convert_seed_ranges(tuples, phase);
    }

    tuples.sort();
    let min = tuples.iter().min().unwrap();

    println!("{min:?}");

    Ok(())
}
