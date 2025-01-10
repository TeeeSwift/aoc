use std::collections::HashMap;

use anyhow::Result;
use d04::*;

pub fn main() -> Result<()> {
    let v = parse_input("src/input")?;
    let mut map: HashMap<usize, usize> = (1..=v.len()).map(|key| (key, 1)).collect();

    for card in v {
        let copies = *map.get(&card.id).unwrap();

        let (id, count) = matches(card)?;
        for i in 0..count {
            map.entry(id + i + 1)
                .and_modify(|e| *e += copies)
                .or_insert(0);
        }
    }

    let sum: usize = map.values().sum();

    println!("{sum}");

    Ok(())
}
