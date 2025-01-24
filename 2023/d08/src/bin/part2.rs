use anyhow::Result;
use d08::*;

pub fn main() -> Result<()> {
    let (instructions, map) = parse_input("src/input")?;
    let currents: Vec<String> = map
        .iter()
        .filter(|(k, _)| k.chars().nth(2).unwrap() == 'A')
        .map(|(k, _)| k.clone())
        .collect();

    let counts = currents
        .into_iter()
        .map(|start| simulate(start, &instructions, &map).unwrap())
        .collect::<Vec<usize>>();

    let sync = lcm_of_list(&counts);

    println!("{sync:?}");

    Ok(())
}
