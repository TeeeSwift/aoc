use std::collections::HashMap;

use anyhow::Result;
use d12::*;

pub fn main() -> Result<()> {
    let records: Vec<Record> = parse_input("src/input")?;
    let mut total: usize = 0;
    let mut map: HashMap<(String, Vec<usize>), usize> = HashMap::new();

    for (string, nums) in records {
        total += count(&string, &nums, &mut map)?;
    }

    println!("{total}");

    Ok(())
}
