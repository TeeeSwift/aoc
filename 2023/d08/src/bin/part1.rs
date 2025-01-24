use anyhow::Result;
use d08::*;

pub fn main() -> Result<()> {
    let (instructions, map) = parse_input("src/input")?;
    let mut count: usize = 0;
    let mut current: String = String::from("AAA");
    let mut i: usize = 0;

    while current != *"ZZZ" {
        let inst = instructions[i];
        let (left, right) = map.get(&current).unwrap();
        match inst {
            'L' => {
                current = left.clone();
            }
            'R' => {
                current = right.clone();
            }
            _ => {}
        };

        count += 1;

        i += 1;
        if i == instructions.len() {
            i = 0;
        };
    }

    println!("{count}");

    Ok(())
}
