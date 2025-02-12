use std::collections::HashMap;

use anyhow::Result;
use d12::*;

pub fn main() -> Result<()> {
    let records: Vec<Record> = parse_input("src/input")?;
    let mut total: usize = 0;
    let mut map: HashMap<(String, Vec<usize>), usize> = HashMap::new();

    for (old_string, old_nums) in records {
        let mut string: String = old_string.clone();
        let mut nums: Vec<usize> = old_nums.clone();

        for _ in 0..4 {
            string.push('?');
            string.push_str(&old_string);
            nums.extend(old_nums.clone());
        }

        total += count(&string, &nums, &mut map)?;
    }

    println!("{total}");

    Ok(())
}
